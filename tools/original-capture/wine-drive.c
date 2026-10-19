/*
 * wine-drive.c — Windows-side input and screenshot helper for Star Wars
 * Rebellion running under Wine on macOS.  Cross-compiled with mingw-w64
 * into a helper and the capture DLL it loads into the game:
 *
 *   i686-w64-mingw32-gcc -O2 -o drive.exe wine-drive.c -lgdi32 -luser32
 *   i686-w64-mingw32-gcc -O2 -shared -DWINE_CAP_DLL -DDRIVE_BUILD='"ID"' \
 *       -Wl,--kill-at -o drive-cap.dll wine-drive.c -lgdi32 -luser32
 *
 * and, from the same source, a 64-bit helper whose mouse input leaves the
 * Mac pointer where it is (see "input without warping" below):
 *
 *   x86_64-w64-mingw32-gcc -O2 -o drive64.exe wine-drive.c -lgdi32 -luser32
 *
 * wine-setup.sh builds all three, with the first 16 hex digits of the
 * source's SHA-256 as the build ID.
 *
 * All commands find the game window by title.  Input makes it Wine's
 * foreground window and injects hardware input (see "input" below), so the
 * Wine window need not be the macOS foreground app.  Coordinates are in the
 * reference 800x600 frame used by the VM capture rig (vm-game.sh); they are
 * scaled to the actual client rectangle before delivery.
 *
 * Wine's macOS driver keeps a window's pixels in a surface inside the
 * process that owns it, so a BitBlt from another process reads black.
 * cap therefore runs the copy inside REBEXE.EXE: it loads drive-cap.dll
 * there (drive.exe's own folder) and starts cap_thread on a remote thread.
 *
 * Usage:
 *   wine drive.exe find
 *   wine drive.exe click X Y
 *   wine drive.exe rclick X Y
 *   wine drive.exe dclick X Y
 *   wine drive.exe drag X1 Y1 X2 Y2
 *   wine drive.exe key NAME [COUNT]
 *   wine drive.exe type TEXT
 *   wine drive.exe cap FILE.bmp
 *   wine drive.exe size             (prints client width and height)
 *   wine drive.exe clock            (prints the game day and step count)
 *   wine drive.exe timescale K      (runs the game's clock K times faster)
 *   wine drive.exe stopat DAY       (pauses the game at the first step of DAY;
 *                                    0 releases it; disarm drops a stop not
 *                                    yet reached)
 *   wine drive.exe hook             (installs the step hook: day log, holds)
 *   wine drive.exe messages N FILE  (the newest N Message Index entries)
 *   wine drive.exe peek PTR OFF [N] (prints the dword at *PTR + OFF; with N,
 *                                    a histogram of N reads 1 ms apart)
 *   wine drive.exe cursor on|off    (GetCursorPos in the game reads the point
 *                                    drive64.exe last sent, or Wine's own)
 *
 * drive64.exe takes the same verbs except the ones that run inside the game
 * (cap, timescale, stopat, hook, messages, cursor), plus
 *   wine drive64.exe selftest       (checks that a move reaches Wine's
 *                                    cursor and not the Mac pointer)
 */

#include <windows.h>
#include <tlhelp32.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define REF_W 800
#define REF_H 600

/* ---------- window search ---------- */

static HWND find_rebellion(void) {
    HWND h = FindWindowA(NULL, "Star Wars Rebellion");
    if (!h)
        h = FindWindowA(NULL, "Star Wars Rebellion ");  /* trailing space variant */
    return h;
}

/* The game's 640x480 drawing area in client coordinates.  Under cnc-ddraw
   it is the whole client; Wine's builtin ddraw keeps a full-screen window
   when the game's 640x480 mode change fails, and the game draws at its
   origin. */
static RECT game_area(HWND hwnd) {
    RECT rc;
    GetClientRect(hwnd, &rc);
    if (rc.right > 640)
        rc.right = 640;
    if (rc.bottom > 480)
        rc.bottom = 480;
    return rc;
}

/* The point drive64.exe last sent, in screen coordinates, packed as two
   SHORTs so one store publishes it.  drive-cap.dll creates the section in
   the game, so it lives as long as the game. */
#define CURSOR_SECTION "Local\\or-drive-cursor"
struct drive_cursor { volatile LONG active; volatile LONG xy; };

#ifndef WINE_CAP_DLL

/* ---------- input ----------
 *
 * Input goes in through SendInput, as hardware input.  The game polls
 * GetAsyncKeyState for Escape and button 1 (FUN_005f7240, FUN_006036a0),
 * and Wine runs its WH_KEYBOARD/WH_MOUSE hooks only for hardware messages,
 * so posted messages never reach it.  cnc-ddraw reports every key up while
 * the game is not Wine's foreground window and, without devmode, drops
 * mouse input until it locks the cursor; setup sets devmode in [REBEXE].
 * Making the game Wine's foreground leaves the macOS frontmost app alone,
 * but each of drive.exe's mouse moves warps the Mini's real pointer;
 * drive64.exe's do not.
 */

static int focus(HWND hwnd) {
    if (GetForegroundWindow() != hwnd) {
        SetForegroundWindow(hwnd);
        Sleep(50);
    }
    if (GetForegroundWindow() != hwnd) {
        fprintf(stderr, "Cannot make the game the foreground window\n");
        return 1;
    }
    return 0;
}

/* FALSE when Wine inserted no event. */
static BOOL send_input(INPUT *in) {
    return SendInput(1, in, sizeof(*in)) == 1;
}

#ifdef _WIN64

/* ---------- input without warping (drive64.exe) ----------
 *
 * Wine warps the Mac pointer after injected input only: send_hardware_message
 * calls the driver's SetCursorPos when SEND_HWMSG_INJECTED is set
 * (win32u/message.c), and macdrv_SetCursorPos is CGWarpMouseCursorPosition.
 * The Mac driver delivers its own events with flags 0 through
 * NtUserSendHardwareInput (winemac.drv/mouse.c), an inline over the win32u
 * syscall NtUserCallHwndParam; mouse input goes in the same way here.  The
 * server moves its cursor, updates the key state GetAsyncKeyState reads and
 * queues ordinary hardware messages, which the game's hooks see.  Absolute
 * coordinates are raw screen pixels, which equal ClientToScreen's while no
 * display mode is emulated; selftest checks that they do.  The WoW64 thunk
 * passes the INPUT pointer through unconverted, so only a 64-bit caller
 * can lay it out right.  Keys stay on SendInput, which never warps.
 */

/* NtUserCallHwndParam_SendHardwareInput (include/ntuser.h, wine-10.0):
   Wine's own numbering, so selftest checks it against the engine. */
#ifndef SEND_HARDWARE_INPUT
#define SEND_HARDWARE_INPUT 28
#endif

struct send_hardware_input_params { UINT flags; const INPUT *input; LPARAM lparam; };

static ULONG_PTR (WINAPI *call_hwnd_param)(HWND, DWORD_PTR, DWORD);
static struct drive_cursor *cursor_state;

/* Send one mouse INPUT as the Mac driver would; FALSE when Wine refused. */
static BOOL hardware_input(HWND hwnd, const INPUT *in) {
    if (!call_hwnd_param) {
        HMODULE win32u = LoadLibraryA("win32u.dll");
        call_hwnd_param = (ULONG_PTR (WINAPI *)(HWND, DWORD_PTR, DWORD))
            (win32u ? GetProcAddress(win32u, "NtUserCallHwndParam") : NULL);
        if (!call_hwnd_param) {
            fprintf(stderr, "win32u.dll has no NtUserCallHwndParam\n");
            return FALSE;
        }
    }
    struct send_hardware_input_params params = { 0, in, 0 };
    return call_hwnd_param(hwnd, (DWORD_PTR)&params, SEND_HARDWARE_INPUT) == 0;
}

/* Publish pt for drive-cap.dll's GetCursorPos, once the game has made the
   section ("cursor on"). */
static void publish_cursor(POINT pt) {
    if (!cursor_state) {
        HANDLE section = OpenFileMappingA(FILE_MAP_WRITE, FALSE, CURSOR_SECTION);
        if (!section)
            return;
        cursor_state = MapViewOfFile(section, FILE_MAP_WRITE, 0, 0, sizeof(*cursor_state));
        CloseHandle(section);
        if (!cursor_state)
            return;
    }
    InterlockedExchange(&cursor_state->xy, (LONG)MAKELONG((SHORT)pt.x, (SHORT)pt.y));
}

#endif /* _WIN64 */

/* The screen point of reference point (rx, ry). */
static POINT screen_point(HWND hwnd, int rx, int ry) {
    RECT rc = game_area(hwnd);
    POINT pt = { (LONG)((long long)rx * rc.right / REF_W),
                 (LONG)((long long)ry * rc.bottom / REF_H) };
    ClientToScreen(hwnd, &pt);
    return pt;
}

static void key_event(UINT vk, BOOL up) {
    INPUT in;
    memset(&in, 0, sizeof(in));
    in.type = INPUT_KEYBOARD;
    in.ki.wVk = (WORD)vk;
    in.ki.wScan = (WORD)MapVirtualKeyA(vk, MAPVK_VK_TO_VSC);
    switch (vk) {
    case VK_LEFT: case VK_RIGHT: case VK_UP: case VK_DOWN:
    case VK_HOME: case VK_END: case VK_PRIOR: case VK_NEXT: case VK_DELETE:
        in.ki.dwFlags |= KEYEVENTF_EXTENDEDKEY;
    }
    if (up)
        in.ki.dwFlags |= KEYEVENTF_KEYUP;
    send_input(&in);
}

/* Move the pointer to reference point (rx, ry).  Wine maps an injected
   absolute move to left + (dx * width >> 16) on the primary monitor, so
   round up; a hardware move carries the pixel itself.  FALSE when Wine
   refused the input. */
static BOOL mouse_move(HWND hwnd, int rx, int ry) {
    POINT pt = screen_point(hwnd, rx, ry);
    INPUT in;
    memset(&in, 0, sizeof(in));
    in.type = INPUT_MOUSE;
    in.mi.dwFlags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
#ifdef _WIN64
    in.mi.dx = pt.x;
    in.mi.dy = pt.y;
    publish_cursor(pt);
    return hardware_input(hwnd, &in);
#else
    long long w = GetSystemMetrics(SM_CXSCREEN);
    long long h = GetSystemMetrics(SM_CYSCREEN);
    in.mi.dx = (LONG)((pt.x * 65536LL + w - 1) / w);
    in.mi.dy = (LONG)((pt.y * 65536LL + h - 1) / h);
    return send_input(&in);
#endif
}

static BOOL mouse_button(HWND hwnd, DWORD flag) {
    INPUT in;
    memset(&in, 0, sizeof(in));
    in.type = INPUT_MOUSE;
    in.mi.dwFlags = flag;
#ifdef _WIN64
    return hardware_input(hwnd, &in);
#else
    (void)hwnd;
    return send_input(&in);
#endif
}

/* The gestures below send every event even after one is refused, so a
   button that went down always gets its release, and return 1 when any
   event was refused. */

/* Buttons are held under 100 ms: FUN_006036a0 posts its command again
   for as long as button 1 reads down. */
static BOOL press(HWND hwnd, DWORD down, DWORD up) {
    BOOL ok = mouse_button(hwnd, down);
    Sleep(60);
    ok &= mouse_button(hwnd, up);
    Sleep(40);
    return ok;
}

static int send_click(HWND hwnd, int rx, int ry) {
    BOOL ok = mouse_move(hwnd, rx, ry);
    Sleep(40);
    ok &= press(hwnd, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP);
    return !ok;
}

static int send_rclick(HWND hwnd, int rx, int ry) {
    BOOL ok = mouse_move(hwnd, rx, ry);
    Sleep(40);
    ok &= press(hwnd, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP);
    return !ok;
}

static int send_dclick(HWND hwnd, int rx, int ry) {
    BOOL ok = mouse_move(hwnd, rx, ry);
    Sleep(40);
    ok &= mouse_button(hwnd, MOUSEEVENTF_LEFTDOWN);
    Sleep(40);
    ok &= mouse_button(hwnd, MOUSEEVENTF_LEFTUP);
    Sleep(80);
    ok &= mouse_button(hwnd, MOUSEEVENTF_LEFTDOWN);
    Sleep(40);
    ok &= mouse_button(hwnd, MOUSEEVENTF_LEFTUP);
    Sleep(40);
    return !ok;
}

/* Drag in steps of at most 8 reference pixels, 16 ms apart. */
static int send_drag(HWND hwnd, int rx1, int ry1, int rx2, int ry2) {
    BOOL ok = mouse_move(hwnd, rx1, ry1);
    Sleep(40);
    ok &= mouse_button(hwnd, MOUSEEVENTF_LEFTDOWN);
    Sleep(100);
    int span = abs(rx2 - rx1) > abs(ry2 - ry1) ? abs(rx2 - rx1) : abs(ry2 - ry1);
    int steps = span / 8 + 1;
    for (int i = 1; i <= steps; i++) {
        ok &= mouse_move(hwnd, rx1 + (rx2 - rx1) * i / steps,
                         ry1 + (ry2 - ry1) * i / steps);
        Sleep(16);
    }
    Sleep(100);
    ok &= mouse_button(hwnd, MOUSEEVENTF_LEFTUP);
    Sleep(40);
    return !ok;
}

#ifdef _WIN64

/* Wine's GetCursorPos returns the server's cursor for 100 ms after the last
   mouse input and the Mac driver's, which is the Mac pointer, after that
   (win32u/input.c get_cursor_pos, macdrv_GetCursorPos).  So a read just
   after a move shows where the server put the cursor, and a read 150 ms
   after any input shows the Mac pointer as Wine sees it.  Refuse (125)
   when the server cursor did not reach the target or the Mac pointer
   moved: a wrong SEND_HARDWARE_INPUT, an emulated display mode (raw
   pixels no longer equal screen coordinates), or a warp. */
static int selftest(HWND hwnd) {
    printf("code=%d\n", SEND_HARDWARE_INPUT);
    Sleep(150);
    POINT before, server, after;
    GetCursorPos(&before);
    POINT target = screen_point(hwnd, REF_W / 2, REF_H / 2);
    if (target.x == before.x && target.y == before.y)
        target.x += 10;
    INPUT in;
    memset(&in, 0, sizeof(in));
    in.type = INPUT_MOUSE;
    in.mi.dx = target.x;
    in.mi.dy = target.y;
    in.mi.dwFlags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
    BOOL sent = hardware_input(hwnd, &in);
    GetCursorPos(&server);
    Sleep(150);
    GetCursorPos(&after);
    printf("target=%ld,%ld\n", target.x, target.y);
    printf("server=%ld,%ld\n", server.x, server.y);
    printf("mac_before=%ld,%ld\n", before.x, before.y);
    printf("mac_after=%ld,%ld\n", after.x, after.y);
    if (!sent) {
        fprintf(stderr, "Wine refused the hardware input\n");
        return 125;
    }
    if (server.x != target.x || server.y != target.y) {
        fprintf(stderr, "The server cursor did not reach the target\n");
        return 125;
    }
    if (after.x != before.x || after.y != before.y) {
        fprintf(stderr, "The Mac pointer moved during the test\n");
        return 125;
    }
    return 0;
}

#endif /* _WIN64 */

/* ---------- key name to VK mapping ---------- */

struct key_entry { const char *name; UINT vk; };
static const struct key_entry key_table[] = {
    {"esc",       VK_ESCAPE},   {"escape",    VK_ESCAPE},
    {"enter",     VK_RETURN},   {"return",    VK_RETURN},
    {"space",     VK_SPACE},    {"tab",       VK_TAB},
    {"backspace", VK_BACK},     {"delete",    VK_DELETE},
    {"left",      VK_LEFT},     {"right",     VK_RIGHT},
    {"up",        VK_UP},       {"down",      VK_DOWN},
    {"home",      VK_HOME},     {"end",       VK_END},
    {"pageup",    VK_PRIOR},    {"pagedown",  VK_NEXT},
    {"f1",  VK_F1},  {"f2",  VK_F2},  {"f3",  VK_F3},  {"f4",  VK_F4},
    {"f5",  VK_F5},  {"f6",  VK_F6},  {"f7",  VK_F7},  {"f8",  VK_F8},
    {"f9",  VK_F9},  {"f10", VK_F10}, {"f11", VK_F11}, {"f12", VK_F12},
    {NULL, 0}
};

static int send_key(HWND hwnd, const char *name, int count) {
    /* Check for modifier+key (e.g. alt+f4) */
    char buf[64];
    strncpy(buf, name, sizeof(buf) - 1);
    buf[sizeof(buf) - 1] = '\0';

    /* Lowercase the name for matching */
    for (char *p = buf; *p; p++)
        if (*p >= 'A' && *p <= 'Z') *p += 32;

    UINT mod_vk = 0;
    char *key_part = buf;
    char *plus = strchr(buf, '+');
    if (plus) {
        *plus = '\0';
        key_part = plus + 1;
        if (strcmp(buf, "alt") == 0) mod_vk = VK_MENU;
        else if (strcmp(buf, "ctrl") == 0 || strcmp(buf, "control") == 0) mod_vk = VK_CONTROL;
        else if (strcmp(buf, "shift") == 0) mod_vk = VK_SHIFT;
        else {
            fprintf(stderr, "Unknown modifier: %s\n", buf);
            return 1;
        }
    }

    /* Look up key */
    UINT vk = 0;
    for (const struct key_entry *e = key_table; e->name; e++) {
        if (strcmp(key_part, e->name) == 0) {
            vk = e->vk;
            break;
        }
    }
    /* Single printable character */
    if (!vk && strlen(key_part) == 1) {
        char c = key_part[0];
        if (c >= 'a' && c <= 'z') vk = VkKeyScanA(c) & 0xFF;
        else if (c >= '0' && c <= '9') vk = c;
        else vk = VkKeyScanA(c) & 0xFF;
    }
    if (!vk) {
        fprintf(stderr, "Unknown key: %s\n", key_part);
        return 1;
    }

    /* Down and up go in separate calls, 200 ms apart, so a poll between
       them reads the key down (FUN_005f7240).  Runs of more than three are
       for edit fields, which read messages, and hold 60 ms. */
    DWORD hold = count > 3 ? 60 : 200;
    for (int i = 0; i < count; i++) {
        if (mod_vk) {
            key_event(mod_vk, FALSE);
            Sleep(30);
        }
        key_event(vk, FALSE);
        Sleep(hold);
        key_event(vk, TRUE);
        if (mod_vk) {
            Sleep(30);
            key_event(mod_vk, TRUE);
        }
        Sleep(40);
    }
    return 0;
}

/* Type printable ASCII into the focused field, one key per character. */
static int type_text(const char *text) {
    for (const char *c = text; *c; c++) {
        SHORT scan = VkKeyScanA(*c);
        if (scan == -1) {
            fprintf(stderr, "Cannot type character 0x%02x\n", (unsigned char)*c);
            return 1;
        }
        BOOL shift = (scan & 0x100) != 0;
        if (shift)
            key_event(VK_SHIFT, FALSE);
        key_event(scan & 0xff, FALSE);
        Sleep(30);
        key_event(scan & 0xff, TRUE);
        if (shift)
            key_event(VK_SHIFT, TRUE);
        Sleep(30);
    }
    return 0;
}

#else /* WINE_CAP_DLL */

/* ---------- GDI screenshot (runs inside the game) ---------- */

static int capture(HWND hwnd, const char *path) {
    RECT rc = game_area(hwnd);
    int w = rc.right;
    int h = rc.bottom;
    if (w <= 0 || h <= 0) {
        fprintf(stderr, "Client rect is empty (%dx%d)\n", w, h);
        return 1;
    }

    HDC hdc_win = GetDC(hwnd);
    if (!hdc_win) {
        fprintf(stderr, "GetDC failed\n");
        return 1;
    }
    HDC hdc_mem = CreateCompatibleDC(hdc_win);
    HBITMAP hbm = CreateCompatibleBitmap(hdc_win, w, h);
    HGDIOBJ old = SelectObject(hdc_mem, hbm);
    BOOL copied = BitBlt(hdc_mem, 0, 0, w, h, hdc_win, 0, 0, SRCCOPY);
    SelectObject(hdc_mem, old);
    if (!copied) {
        fprintf(stderr, "BitBlt failed (%lu)\n", GetLastError());
        DeleteObject(hbm);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_win);
        return 1;
    }

    BITMAPINFOHEADER bi;
    memset(&bi, 0, sizeof(bi));
    bi.biSize = sizeof(bi);
    bi.biWidth = w;
    bi.biHeight = -h;  /* top-down */
    bi.biPlanes = 1;
    bi.biBitCount = 24;
    bi.biCompression = BI_RGB;

    int stride = ((w * 3 + 3) & ~3);
    int data_size = stride * h;
    unsigned char *pixels = (unsigned char *)malloc(data_size);
    if (!pixels) {
        fprintf(stderr, "malloc failed\n");
        DeleteObject(hbm);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_win);
        return 1;
    }
    if (GetDIBits(hdc_mem, hbm, 0, h, pixels, (BITMAPINFO *)&bi, DIB_RGB_COLORS) != h) {
        fprintf(stderr, "GetDIBits failed\n");
        free(pixels);
        DeleteObject(hbm);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_win);
        return 1;
    }

    BITMAPFILEHEADER bf;
    memset(&bf, 0, sizeof(bf));
    bf.bfType = 0x4D42;
    bf.bfSize = sizeof(bf) + sizeof(bi) + data_size;
    bf.bfOffBits = sizeof(bf) + sizeof(bi);

    FILE *f = fopen(path, "wb");
    if (!f) {
        fprintf(stderr, "Cannot open %s\n", path);
        free(pixels);
        DeleteObject(hbm);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_win);
        return 1;
    }
    fwrite(&bf, sizeof(bf), 1, f);
    fwrite(&bi, sizeof(bi), 1, f);
    fwrite(pixels, data_size, 1, f);
    fclose(f);

    printf("%dx%d -> %s\n", w, h, path);

    free(pixels);
    DeleteObject(hbm);
    DeleteDC(hdc_mem);
    ReleaseDC(hwnd, hdc_win);
    return 0;
}

/* drive.exe compares this with the copy the game holds before calling any
   export: the game keeps the first drive-cap.dll it loaded. */
#ifndef DRIVE_BUILD
#define DRIVE_BUILD __DATE__ " " __TIME__
#endif
__declspec(dllexport) const char drive_build[] = DRIVE_BUILD;

__declspec(dllexport) DWORD WINAPI cap_thread(LPVOID path) {
    HWND hwnd = find_rebellion();
    if (!hwnd)
        return 1;
    return (DWORD)capture(hwnd, (const char *)path);
}

/* ---------- game time scale (runs inside the game) ----------
 *
 * The strategic step waits in phase 0 until the speed timer at GameOptions
 * +0x110 has run past _DAT_006588f0 (FUN_0041e390, FUN_00415e20); that
 * timer reads QueryPerformanceCounter (FUN_005fbbe0).  Animations and the
 * droids run on periodic tickers that divide timeGetTime() by their period
 * (FUN_00441d70).  The game calls no other time API while the galaxy runs
 * (Wine relay trace).  REBEXE's own imports of both are pointed at scaled
 * versions returning virt0 + (real - real0) * k: continuous and monotonic,
 * re-based at each change of k.  cnc-ddraw imports timeGetTime separately
 * and keeps real time for its frame limiter.  A step does the same work at
 * any k; only its pace changes.
 */

struct time_scale { DWORD real0, virt0, k; LONGLONG qreal0, qvirt0; };
static struct time_scale scales[2] = {{0, 0, 1, 0, 0}, {0, 0, 1, 0, 0}};
static struct time_scale *volatile current = &scales[0];
static DWORD (WINAPI *real_time)(void);
static BOOL (WINAPI *real_qpc)(LARGE_INTEGER *);

static DWORD WINAPI scaled_time(void) {
    const struct time_scale *s = current;
    return s->virt0 + (real_time() - s->real0) * s->k;
}

/* The day the game clock reads: *DAT_006b2fcc + 0x10 (FUN_0051ce00,
   FUN_004fcf00), or 0 with no game in progress. */
static DWORD game_day(void) {
    const DWORD *sched = *(DWORD *const *)0x006b2fcc;
    return sched ? sched[0x10 / 4] : 0;
}

static BOOL WINAPI scaled_qpc(LARGE_INTEGER *count) {
    const struct time_scale *s = current;
    LARGE_INTEGER now;
    if (!real_qpc(&now))
        return FALSE;
    count->QuadPart = s->qvirt0 + (now.QuadPart - s->qreal0) * (LONGLONG)s->k;
    return TRUE;
}

/* ---------- hold at a day (runs inside the game) ----------
 *
 * The game's Pause parks the step machine in phase 2 of FUN_0041dff0, which
 * holds while FUN_0041e300(T) finds the step count (FUN_0041e1b0, sched
 * +0xc) equal to the step object's stop T+0x10 (T = *DAT_006b1218).  The
 * menu's Pause (FUN_0041d2f0 -> FUN_00436890) sets the stop and each side's
 * hold C+0x48 (FUN_004888d0; sides at *DAT_006b14e4 +0x10 and +0x14); once
 * both holds are 0, FUN_004361d0 clears the stop.  Parked in phase 2 the
 * game acts on nothing: the main loop (FUN_00413560) still dispatches
 * window messages, but each pass FUN_004361d0 calls both sides' slot +0x28
 * (FUN_00486520, vtable 0x0065ad70), which runs the side's pending orders
 * (FUN_00488b90) only while its hold is 0 and FUN_0041dc10 finds no stop.
 * The frame does not redraw either (captures while held are identical).
 * FUN_0041e300 is
 * replaced by a jump to held_step: from stop_day on it sets the stop and
 * both holds to the present step, once, then answers as the original.  The
 * step that rolled the day has run phase 0 (FUN_0051dea0) but none of the
 * day's timers (phase 5).  "Resume Game Play?" stays shut: only
 * FUN_00415e60 opens it.
 */
static volatile LONG stop_day;

static int *side_holds(int n) {
    int session = *(int *)0x006b14e4;
    int side = session ? *(int *)(session + 0x10 + 4 * n) : 0;
    return side ? (int *)(side + 0x48) : NULL;
}

/* The day each step began on, for messages, which carry only the step
   they were filed on (msg +0x38, FUN_004c4ce0).  Reset when a load builds
   a new scheduler. */
#define DAY_LOG 8192
static DWORD day_log_day[DAY_LOG], day_log_step[DAY_LOG], day_log_sched;
static int day_log_n;

static void log_day(int step) {
    DWORD sched = *(DWORD *)0x006b2fcc, day = game_day();
    if (sched != day_log_sched) {
        day_log_sched = sched;
        day_log_n = 0;
    }
    if (!sched || (day_log_n && day_log_day[day_log_n - 1] == day))
        return;
    if (day_log_n == DAY_LOG)
        return;
    day_log_day[day_log_n] = day;
    day_log_step[day_log_n++] = (DWORD)step;
}

/* The day a step fell on, or 0 when it is older than the log (printed
   as day=?). */
static DWORD day_of_step(DWORD step) {
    DWORD day = 0;
    for (int i = 0; i < day_log_n && day_log_step[i] <= step; i++)
        day = day_log_day[i];
    return day;
}

/* 0 idle, 1 asked for, 2 being written by the game's thread.  The asker
   cancels only a request the game has not taken, so a request given up on
   never writes its file later. */
static volatile LONG dump_pending;
static char dump_path[MAX_PATH];
static long dump_count;
static HANDLE dump_done;

static void write_text(FILE *f, const char *text) {
    for (int i = 0; text && text[i] && i < 400; i++)
        fputc(text[i] == '\r' || text[i] == '\n' || text[i] == '\t' ? ' ' : text[i], f);
}

/* The local side's Message Index list, *(*DAT_006b14e4 + 0xc) + 0x7c
   (FUN_00436780): a threaded tree keyed at +0x18, newest at the far right
   (FUN_005f5080), each earlier one its predecessor (FUN_005f5c60).  A
   message is its node: +0x24 bit 0 read, +0x34 category mask, +0x38 the
   filing step, title char* at +0x48, body char* at +0x54, kind from
   vtable slot +0x10 (FUN_0048a060). */
static void dump_messages(void) {
    FILE *f = fopen(dump_path, "w");
    if (!f)
        return;
    int session = *(int *)0x006b14e4;
    int side = session ? *(int *)(session + 0xc) : 0;
    int node = side ? *(int *)(side + 0x7c + 4) : 0;
    if (node)
        while (*(int *)(node + 8))
            node = *(int *)(node + 8);
    for (long n = 0; node && n < dump_count; n++) {
        DWORD step = *(DWORD *)(node + 0x38);
        int kind = ((int (__attribute__((thiscall)) *)(int))(*(int **)node)[0x10 / 4])(node);
        DWORD day = day_of_step(step);
        if (day)
            fprintf(f, "day=%lu ", day);
        else
            fputs("day=? ", f);
        fprintf(f, "step=%lu kind=%d mask=0x%03lx %s\t", step, kind,
                *(DWORD *)(node + 0x34) & 0xfff, *(DWORD *)(node + 0x24) & 1 ? "read" : "unread");
        write_text(f, *(const char **)(node + 0x48));
        fputs("\t", f);
        write_text(f, *(const char **)(node + 0x54));
        fputs("\n", f);
        /* Predecessor: the left subtree's rightmost, else the first
           ancestor reached from its right. */
        if (*(int *)(node + 4)) {
            node = *(int *)(node + 4);
            while (*(int *)(node + 8))
                node = *(int *)(node + 8);
        } else {
            int child = node;
            node = *(int *)(node + 0xc);
            while (node && *(int *)(node + 8) != child) {
                child = node;
                node = *(int *)(node + 0xc);
            }
        }
    }
    fclose(f);
}

/* Runs on the game's window thread every pass of the step machine. */
static BOOL __fastcall held_step(int step_object) {
    int step = ((int (__cdecl *)(void))0x0041e1b0)();
    log_day(step);
    if (stop_day && game_day() >= (DWORD)stop_day) {
        int *hold0 = side_holds(0), *hold1 = side_holds(1);
        if (hold0 && hold1) {
            *(int *)(step_object + 0x10) = step;
            *hold0 = step;
            *hold1 = step;
            InterlockedExchange(&stop_day, 0);
        }
    }
    if (InterlockedCompareExchange(&dump_pending, 2, 1) == 1) {
        dump_messages();
        InterlockedExchange(&dump_pending, 0);
        SetEvent(dump_done);
    }
    return step == *(int *)(step_object + 0x10);
}

/* Suspend every other thread of the game into threads[]; the count, or -1
   when they cannot all be listed (none is left suspended then). */
#define MAX_THREADS 256
static int suspend_others(HANDLE *threads) {
    HANDLE snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if (snap == INVALID_HANDLE_VALUE)
        return -1;
    DWORD pid = GetCurrentProcessId(), self = GetCurrentThreadId();
    THREADENTRY32 te;
    te.dwSize = sizeof(te);
    int n = 0;
    for (BOOL more = Thread32First(snap, &te); more; more = Thread32Next(snap, &te)) {
        if (te.th32OwnerProcessID != pid || te.th32ThreadID == self)
            continue;
        if (n == MAX_THREADS) {
            n = -1;
            break;
        }
        HANDLE t = OpenThread(THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT, FALSE, te.th32ThreadID);
        if (!t)
            continue;  /* it has exited */
        if (SuspendThread(t) == (DWORD)-1) {
            CloseHandle(t);
            continue;
        }
        threads[n++] = t;
    }
    CloseHandle(snap);
    if (n < 0)
        for (int i = 0; i < MAX_THREADS; i++) {
            ResumeThread(threads[i]);
            CloseHandle(threads[i]);
        }
    return n;
}

/* Write jmp target over the first five bytes of fn.  Any of the game's
   threads may be running fn (cnc-ddraw's as well as the window thread, for
   GetCursorPos), so the write happens with all of them suspended outside
   the bytes being replaced. */
static int write_jump(BYTE *fn, void *target) {
    DWORD old;
    if (!VirtualProtect(fn, 5, PAGE_EXECUTE_READWRITE, &old))
        return 1;
    int rc = 1;
    for (int tries = 0; tries < 100 && rc != 0; tries++) {
        HANDLE threads[MAX_THREADS];
        int n = suspend_others(threads);
        if (n < 0)
            break;
        BOOL clear = TRUE;
        for (int i = 0; i < n && clear; i++) {
            CONTEXT ctx;
            ctx.ContextFlags = CONTEXT_CONTROL;
            clear = GetThreadContext(threads[i], &ctx) &&
                    (ctx.Eip < (DWORD)fn || ctx.Eip >= (DWORD)fn + 5);
        }
        if (clear) {
            LONG rel = (LONG)((BYTE *)target - (fn + 5));
            memcpy(fn + 1, &rel, 4);
            fn[0] = 0xe9;
            rc = 0;
        }
        for (int i = 0; i < n; i++) {
            ResumeThread(threads[i]);
            CloseHandle(threads[i]);
        }
        if (rc != 0)
            Sleep(1);
    }
    VirtualProtect(fn, 5, old, &old);
    FlushInstructionCache(GetCurrentProcess(), fn, 5);
    return rc;
}

/* Write jmp held_step over FUN_0041e300, once.  It must call FUN_0041e1b0
   within its first bytes; refuse any other code. */
static int install_hold(void) {
    BYTE *fn = (BYTE *)0x0041e300;
    if (fn[0] == 0xe9)
        return 0;
    int found = 0;
    for (int i = 0; i < 8 && !found; i++)
        found = fn[i] == 0xe8 && (DWORD)(fn + i + 5 + *(LONG *)(fn + i + 1)) == 0x0041e1b0;
    if (!found)
        return 1;
    return write_jump(fn, held_step);
}

/* ---------- synthetic cursor (runs inside the game) ----------
 *
 * drive64.exe moves Wine's cursor without moving the Mac pointer, and
 * Wine's GetCursorPos returns that cursor only for 100 ms after the last
 * mouse input; after that it asks the Mac driver, which reports the Mac
 * pointer (win32u/input.c get_cursor_pos).  REBEXE imports GetCursorPos,
 * and cnc-ddraw's own GetCursorPos hook calls user32's.  With "cursor on",
 * user32!GetCursorPos jumps to synthetic_cursor_pos, which returns the
 * point drive64.exe last published in CURSOR_SECTION; with "cursor off" it
 * returns what win32u's NtUserGetCursorPos says, which is all user32's
 * GetCursorPos does (user32/input.c).  The section is created here, so it
 * lives as long as the game, and drive64.exe publishes only once it exists.
 */
static struct drive_cursor *cursor_state;
static BOOL (WINAPI *nt_get_cursor_pos)(POINT *);

static BOOL WINAPI synthetic_cursor_pos(POINT *pt) {
    if (pt && cursor_state->active) {
        LONG xy = cursor_state->xy;
        pt->x = (SHORT)LOWORD(xy);
        pt->y = (SHORT)HIWORD(xy);
        return TRUE;
    }
    return nt_get_cursor_pos(pt);
}

/* Make the section, then the jump, once.  The thread drive.exe starts here
   calls nothing that asks the Mac driver for the cursor, since such a call
   never returns on that thread; drive.exe reads the start point instead. */
static int install_cursor(void) {
    if (cursor_state)
        return 0;
    BYTE *fn = (BYTE *)GetProcAddress(GetModuleHandleA("user32.dll"), "GetCursorPos");
    HMODULE win32u = GetModuleHandleA("win32u.dll");
    nt_get_cursor_pos = (BOOL (WINAPI *)(POINT *))
        (win32u ? GetProcAddress(win32u, "NtUserGetCursorPos") : NULL);
    HANDLE section = CreateFileMappingA(INVALID_HANDLE_VALUE, NULL, PAGE_READWRITE, 0,
                                        sizeof(struct drive_cursor), CURSOR_SECTION);
    struct drive_cursor *state = section ?
        MapViewOfFile(section, FILE_MAP_WRITE, 0, 0, sizeof(*state)) : NULL;
    if (!fn || !nt_get_cursor_pos || !state)
        return 1;
    cursor_state = state;
    if (write_jump(fn, synthetic_cursor_pos) != 0) {
        cursor_state = NULL;
        return 1;
    }
    return 0;
}

/* Find REBEXE's import slot bound to module!name; *target gets the bound
   function.  Matching the bound address needs no import name table. */
static IMAGE_THUNK_DATA *find_import(const char *module, const char *name, FARPROC *target) {
    BYTE *exe = (BYTE *)GetModuleHandleA(NULL);
    IMAGE_NT_HEADERS *nt = (IMAGE_NT_HEADERS *)(exe + ((IMAGE_DOS_HEADER *)exe)->e_lfanew);
    IMAGE_DATA_DIRECTORY dir =
        nt->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT];
    *target = GetProcAddress(GetModuleHandleA(module), name);
    if (!*target || !dir.VirtualAddress)
        return NULL;
    for (IMAGE_IMPORT_DESCRIPTOR *imp = (IMAGE_IMPORT_DESCRIPTOR *)(exe + dir.VirtualAddress);
         imp->Name; imp++)
        for (IMAGE_THUNK_DATA *slot = (IMAGE_THUNK_DATA *)(exe + imp->FirstThunk);
             slot->u1.Function; slot++)
            if ((FARPROC)(ULONG_PTR)slot->u1.Function == *target)
                return slot;
    return NULL;
}

static void write_slot(IMAGE_THUNK_DATA *slot, void *replacement) {
    DWORD old;
    VirtualProtect(slot, sizeof(*slot), PAGE_READWRITE, &old);
    slot->u1.Function = (ULONG_PTR)replacement;
    VirtualProtect(slot, sizeof(*slot), old, &old);
}

/* Point both imports at the scaled versions, or neither. */
static int patch_time_imports(void) {
    FARPROC time_fn, qpc_fn;
    IMAGE_THUNK_DATA *time_slot = find_import("winmm.dll", "timeGetTime", &time_fn);
    IMAGE_THUNK_DATA *qpc_slot = find_import("kernel32.dll", "QueryPerformanceCounter", &qpc_fn);
    if (!time_slot || !qpc_slot)
        return 1;
    real_time = (DWORD (WINAPI *)(void))time_fn;
    real_qpc = (BOOL (WINAPI *)(LARGE_INTEGER *))qpc_fn;
    write_slot(time_slot, scaled_time);
    write_slot(qpc_slot, scaled_qpc);
    return 0;
}

/* Fill the idle slot with scale k from the present virtual times, then
   publish it with one pointer store. */
static void rebase(DWORD k) {
    struct time_scale *next = current == &scales[0] ? &scales[1] : &scales[0];
    LARGE_INTEGER qnow, qvirt;
    DWORD now = real_time();
    real_qpc(&qnow);
    next->virt0 = scaled_time();
    next->real0 = now;
    scaled_qpc(&qvirt);
    next->qvirt0 = qvirt.QuadPart;
    next->qreal0 = qnow.QuadPart;
    next->k = k;
    InterlockedExchangePointer((PVOID volatile *)&current, next);
}

/* Set the scale to the decimal integer at arg (1 restores real pace). */
__declspec(dllexport) DWORD WINAPI speed_thread(LPVOID arg) {
    long k = strtol((const char *)arg, NULL, 10);
    if (k < 1 || k > 64)
        return 2;
    if (!real_time && patch_time_imports() != 0)
        return 1;
    /* Fill the idle slot, then publish it with one pointer store. */
    rebase((DWORD)k);
    return 0;
}

/* Install the hook early, so the day log covers the game from its load. */
__declspec(dllexport) DWORD WINAPI hook_thread(LPVOID arg) {
    (void)arg;
    return install_hold() != 0;
}

/* Write the newest messages to a file, from the game's thread: arg is
   "N|PATH".  Returns 3 when the game thread did not serve it in 5 s, and
   then withdraws the request; 4 while an earlier listing is still being
   written. */
__declspec(dllexport) DWORD WINAPI messages_thread(LPVOID arg) {
    const char *bar = strchr((const char *)arg, '|');
    if (!bar || strlen(bar + 1) >= sizeof(dump_path))
        return 2;
    if (install_hold() != 0)
        return 1;
    if (dump_pending)
        return 4;
    if (!dump_done)
        dump_done = CreateEventA(NULL, FALSE, FALSE, NULL);
    dump_count = strtol((const char *)arg, NULL, 10);
    strcpy(dump_path, bar + 1);
    ResetEvent(dump_done);
    InterlockedExchange(&dump_pending, 1);
    if (WaitForSingleObject(dump_done, 5000) == WAIT_OBJECT_0)
        return 0;
    if (InterlockedCompareExchange(&dump_pending, 0, 1) == 1)
        return 3;
    /* The game took it as the wait ended: let it finish. */
    return WaitForSingleObject(dump_done, 2000) == WAIT_OBJECT_0 ? 0 : 4;
}

/* Pause the game at the first step of day arg (0 releases a hold made
   here or by the menu's Pause: both sides' holds go to 0 and the game
   clears its stop).  Returns 3 for a day that has already begun, whose
   first step is past: held_step would hold at once, on a later step.
   "disarm" drops a stop not yet reached and leaves any hold alone. */
__declspec(dllexport) DWORD WINAPI stop_thread(LPVOID arg) {
    if (strcmp((const char *)arg, "disarm") == 0) {
        InterlockedExchange(&stop_day, 0);
        return 0;
    }
    long day = strtol((const char *)arg, NULL, 10);
    if (day < 0)
        return 2;
    if (day > 0 && (DWORD)day <= game_day())
        return 3;
    if (install_hold() != 0)
        return 1;
    if (day == 0) {
        InterlockedExchange(&stop_day, 0);
        int *hold0 = side_holds(0), *hold1 = side_holds(1);
        if (hold0)
            *hold0 = 0;
        if (hold1)
            *hold1 = 0;
    } else {
        InterlockedExchange(&stop_day, day);
    }
    return 0;
}

/* "on X Y" makes GetCursorPos read the published point, starting at
   screen point (X, Y); "off" makes it read Wine's own again. */
__declspec(dllexport) DWORD WINAPI cursor_thread(LPVOID arg) {
    long x, y;
    BOOL on = sscanf((const char *)arg, "on %ld %ld", &x, &y) == 2;
    if (!on && strcmp((const char *)arg, "off") != 0)
        return 2;
    if (!on && !cursor_state)
        return 0;
    if (install_cursor() != 0)
        return 1;
    if (on)
        InterlockedExchange(&cursor_state->xy, (LONG)MAKELONG((SHORT)x, (SHORT)y));
    InterlockedExchange(&cursor_state->active, on);
    return 0;
}

#endif /* WINE_CAP_DLL */

#ifndef WINE_CAP_DLL

/* ---------- calls inside the game ----------
 *
 * drive-cap.dll is 32-bit like the game, and these load it here to find
 * its exports, so drive64.exe has none of them. */

#ifndef _WIN64

/* Run a function at a remote address on a new thread in the game.  Returns
   2 when the thread outlives the wait: it may still read its argument. */
static int remote_call(HANDLE proc, LPTHREAD_START_ROUTINE fn, LPVOID arg,
                       DWORD *result) {
    HANDLE th = CreateRemoteThread(proc, NULL, 0, fn, arg, 0, NULL);
    if (!th) {
        fprintf(stderr, "CreateRemoteThread failed (%lu)\n", GetLastError());
        return 1;
    }
    DWORD wait = WaitForSingleObject(th, 10000);
    GetExitCodeThread(th, result);
    CloseHandle(th);
    if (wait != WAIT_OBJECT_0) {
        fprintf(stderr, "Remote thread timed out\n");
        return 2;
    }
    return 0;
}

/* LoadLibraryA in the game returns a copy it already holds, which may be an
   older build than this drive.exe's: compare the two copies' drive_build
   strings before trusting export offsets. */
static BOOL same_module(HANDLE proc, DWORD base, HMODULE local) {
    const char *mine = (const char *)GetProcAddress(local, "drive_build");
    if (!mine)
        return FALSE;
    size_t len = strlen(mine) + 1;
    char theirs[128];
    return len <= sizeof(theirs) &&
           ReadProcessMemory(proc, (LPCVOID)(ULONG_PTR)(base + (mine - (const char *)local)),
                             theirs, len, NULL) &&
           memcmp(theirs, mine, len) == 0;
}

/* Load drive-cap.dll into the game and run its export fn(arg) there on a
   remote thread; the export's exit code lands in *result. */
static int call_in_game(HWND hwnd, const char *fn, const char *arg, DWORD *result) {
    char dll[MAX_PATH];
    DWORD n = GetModuleFileNameA(NULL, dll, sizeof(dll));
    char *slash = strrchr(dll, '\\');
    if (!n || !slash || (size_t)(slash - dll) + sizeof("\\drive-cap.dll") > sizeof(dll)) {
        fprintf(stderr, "Cannot locate drive-cap.dll\n");
        return 1;
    }
    strcpy(slash + 1, "drive-cap.dll");

    /* The DLL loads at one base in the game; each export sits at the same
       offset from it as in this process. */
    HMODULE local = LoadLibraryA(dll);
    FARPROC local_fn = local ? GetProcAddress(local, fn) : NULL;
    if (!local_fn) {
        fprintf(stderr, "Cannot load %s (%lu)\n", dll, GetLastError());
        return 1;
    }

    DWORD pid = 0;
    GetWindowThreadProcessId(hwnd, &pid);
    HANDLE proc = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION |
                              PROCESS_VM_OPERATION | PROCESS_VM_WRITE |
                              PROCESS_VM_READ | SYNCHRONIZE, FALSE, pid);
    if (!proc) {
        fprintf(stderr, "OpenProcess(%lu) failed (%lu)\n", pid, GetLastError());
        return 1;
    }

    size_t dll_len = strlen(dll) + 1, arg_len = strlen(arg) + 1;
    char *remote = VirtualAllocEx(proc, NULL, dll_len + arg_len,
                                  MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    int rc = 1, call = 1;
    DWORD base = 0;
    if (!remote ||
        !WriteProcessMemory(proc, remote, dll, dll_len, NULL) ||
        !WriteProcessMemory(proc, remote + dll_len, arg, arg_len, NULL)) {
        fprintf(stderr, "Cannot write to the game (%lu)\n", GetLastError());
    } else if ((call = remote_call(proc, (LPTHREAD_START_ROUTINE)
                                       GetProcAddress(GetModuleHandleA("kernel32.dll"),
                                                      "LoadLibraryA"),
                                   remote, &base)) != 0) {
        /* remote_call said why. */
    } else if (!base) {
        fprintf(stderr, "The game could not load %s\n", dll);
    } else if (!same_module(proc, base, local)) {
        fprintf(stderr, "The game holds an older drive-cap.dll; restart the game\n");
    } else if ((call = remote_call(proc, (LPTHREAD_START_ROUTINE)(ULONG_PTR)(base +
                                       ((char *)local_fn - (char *)local)),
                                   remote + dll_len, result)) == 0) {
        rc = 0;
    }

    /* A thread still running after the wait keeps its strings: leak them. */
    if (remote && call != 2)
        VirtualFreeEx(proc, remote, 0, MEM_RELEASE);
    CloseHandle(proc);
    return rc;
}

static int capture_in_game(HWND hwnd, const char *path) {
    DWORD result = 1;
    if (call_in_game(hwnd, "cap_thread", path, &result) != 0)
        return 1;
    if (result != 0) {
        fprintf(stderr, "Capture inside the game failed\n");
        return 1;
    }
    RECT rc = game_area(hwnd);
    printf("%ldx%ld -> %s\n", rc.right, rc.bottom, path);
    return 0;
}

static int set_timescale(HWND hwnd, const char *k) {
    DWORD result = 1;
    if (call_in_game(hwnd, "speed_thread", k, &result) != 0)
        return 1;
    if (result == 2) {
        fprintf(stderr, "Scale must be an integer from 1 to 64\n");
        return 2;
    }
    if (result != 0) {
        fprintf(stderr, "Cannot find REBEXE's timeGetTime or QueryPerformanceCounter import\n");
        return 1;
    }
    printf("timescale=%s\n", k);
    return 0;
}

static int install_in_game(HWND hwnd) {
    DWORD result = 1;
    if (call_in_game(hwnd, "hook_thread", "", &result) != 0)
        return 1;
    if (result != 0) {
        fprintf(stderr, "FUN_0041e300 is not the expected code; no hook installed\n");
        return 1;
    }
    return 0;
}

static int list_messages(HWND hwnd, const char *n, const char *path) {
    char arg[MAX_PATH + 16];
    DWORD result = 1;
    snprintf(arg, sizeof(arg), "%s|%s", n, path);
    if (call_in_game(hwnd, "messages_thread", arg, &result) != 0)
        return 1;
    if (result == 3) {
        fprintf(stderr, "The game did not reach a step in 5 s (a modal dialog open?)\n");
        return 1;
    }
    if (result == 4) {
        fprintf(stderr, "An earlier listing is still being written; try again\n");
        return 1;
    }
    if (result != 0) {
        fprintf(stderr, "Cannot list messages (%lu)\n", result);
        return 1;
    }
    FILE *f = fopen(path, "r");
    if (!f) {
        fprintf(stderr, "No listing at %s\n", path);
        return 1;
    }
    char line[1024];
    while (fgets(line, sizeof(line), f))
        fputs(line, stdout);
    fclose(f);
    return 0;
}

static int set_stop(HWND hwnd, const char *day) {
    DWORD result = 1;
    if (call_in_game(hwnd, "stop_thread", day, &result) != 0)
        return 1;
    if (result == 2) {
        fprintf(stderr, "Day must be 0 or more\n");
        return 2;
    }
    if (result == 3) {
        fprintf(stderr, "Day %s has already begun: pick a later day (see clock)\n", day);
        return 2;
    }
    if (result != 0) {
        fprintf(stderr, "FUN_0041e300 is not the expected code; no hold installed\n");
        return 1;
    }
    printf("stopat=%s\n", day);
    return 0;
}

static int set_cursor(HWND hwnd, const char *mode) {
    char arg[64];
    POINT pt;
    if (strcmp(mode, "on") == 0 && GetCursorPos(&pt))
        snprintf(arg, sizeof(arg), "on %ld %ld", pt.x, pt.y);
    else
        snprintf(arg, sizeof(arg), "%s", mode);
    DWORD result = 1;
    if (call_in_game(hwnd, "cursor_thread", arg, &result) != 0)
        return 1;
    if (result == 2) {
        fprintf(stderr, "cursor takes on or off\n");
        return 2;
    }
    if (result != 0) {
        fprintf(stderr, "Cannot hook GetCursorPos in the game\n");
        return 1;
    }
    printf("cursor=%s\n", mode);
    return 0;
}

#endif /* !_WIN64 */

/* ---------- game clock ---------- */

/* The day scheduler is *DAT_006b2fcc (FUN_0051ce00).  Its +0x10 is the day
   the game clock reads (FUN_004fcf00) and +0xc counts steps
   (FUN_0051dea0); a day runs the larger side speed word plus one steps
   (timer-scheduler.md).  REBEXE loads at 0x400000 without relocations, so
   the address is fixed. */
static int read_clock(HWND hwnd) {
    DWORD pid = 0;
    GetWindowThreadProcessId(hwnd, &pid);
    HANDLE proc = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, FALSE, pid);
    if (!proc) {
        fprintf(stderr, "OpenProcess(%lu) failed (%lu)\n", pid, GetLastError());
        return 1;
    }
    DWORD sched = 0, fields[2] = {0, 0};
    BOOL ok = ReadProcessMemory(proc, (LPCVOID)0x006b2fcc, &sched, sizeof(sched), NULL) &&
              sched &&
              ReadProcessMemory(proc, (LPCVOID)(ULONG_PTR)(sched + 0xc), fields,
                                sizeof(fields), NULL);
    CloseHandle(proc);
    if (!ok) {
        fprintf(stderr, "No game clock (no game in progress?)\n");
        return 1;
    }
    printf("day=%lu steps=%lu sched=0x%08lx\n", fields[1], fields[0], sched);
    return 0;
}

/* Read the dword at *ptr + off in the game, once or n times. */
static int peek(HWND hwnd, const char *ptr_arg, const char *off_arg, const char *n_arg) {
    DWORD ptr = strtoul(ptr_arg, NULL, 0), off = strtoul(off_arg, NULL, 0);
    long n = n_arg ? strtol(n_arg, NULL, 10) : 0;
    if (n < 0 || n > 100000) {
        fprintf(stderr, "Samples must be 0-100000\n");
        return 2;
    }
    DWORD pid = 0;
    GetWindowThreadProcessId(hwnd, &pid);
    HANDLE proc = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, FALSE, pid);
    if (!proc) {
        fprintf(stderr, "OpenProcess(%lu) failed (%lu)\n", pid, GetLastError());
        return 1;
    }
    enum { DISTINCT = 64 };
    DWORD values[DISTINCT], counts[DISTINCT], distinct = 0, other = 0;
    int rc = 0;
    for (long i = 0; i < (n ? n : 1); i++) {
        DWORD obj = 0, value = 0;
        if (!ReadProcessMemory(proc, (LPCVOID)(ULONG_PTR)ptr, &obj, sizeof(obj), NULL) || !obj ||
            !ReadProcessMemory(proc, (LPCVOID)(ULONG_PTR)(obj + off), &value, sizeof(value), NULL)) {
            fprintf(stderr, "Cannot read *0x%08lx + 0x%lx\n", ptr, off);
            rc = 1;
            break;
        }
        if (!n) {
            printf("0x%08lx (%lu)\n", value, value);
            break;
        }
        DWORD j = 0;
        while (j < distinct && values[j] != value)
            j++;
        if (j < distinct)
            counts[j]++;
        else if (distinct < DISTINCT) {
            values[distinct] = value;
            counts[distinct++] = 1;
        } else
            other++;
        Sleep(1);
    }
    for (DWORD j = 0; n && !rc && j < distinct; j++)
        printf("0x%08lx %lu\n", values[j], counts[j]);
    if (other)
        printf("other %lu\n", other);
    CloseHandle(proc);
    return rc;
}

/* Parse n reference points from args into c (x, y, x, y): decimal integers
   inside the 800x600 frame, so a typo never lands as (0, 0). */
static int parse_points(char **args, int n, int *c) {
    for (int i = 0; i < 2 * n; i++) {
        int limit = i % 2 ? REF_H : REF_W;
        char *end;
        long v = strtol(args[i], &end, 10);
        if (end == args[i] || *end || v < 0 || v >= limit) {
            fprintf(stderr, "%s must be an integer from 0 to %d\n", i % 2 ? "Y" : "X", limit - 1);
            return 1;
        }
        c[i] = (int)v;
    }
    return 0;
}

/* ---------- main ---------- */

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr,
            "Usage: drive.exe find | click X Y | rclick X Y | dclick X Y\n"
            "       drive.exe drag X1 Y1 X2 Y2 | key NAME [COUNT] | type TEXT | cap FILE\n"
            "       drive.exe size | clock | timescale K | stopat DAY | hook | messages N FILE\n"
            "       drive.exe peek PTR OFF [N] | cursor on|off\n"
            "       drive64.exe: the same but cap, timescale, stopat, hook, messages and\n"
            "       cursor, plus selftest\n"
            "Coordinates are in the 800x600 reference frame.\n");
        return 2;
    }

    if (strcmp(argv[1], "find") == 0) {
        HWND h = find_rebellion();
        printf("%s\n", h ? "found" : "not_found");
        return h ? 0 : 1;
    }

    HWND hwnd = find_rebellion();
    if (!hwnd) {
        fprintf(stderr, "Rebellion window not found\n");
        return 1;
    }

    if (strcmp(argv[1], "size") == 0) {
        RECT rc;
        GetClientRect(hwnd, &rc);
        printf("%ldx%ld\n", rc.right, rc.bottom);
        return 0;
    }

    if (strcmp(argv[1], "clock") == 0)
        return read_clock(hwnd);
    if (strcmp(argv[1], "peek") == 0 && argc >= 4)
        return peek(hwnd, argv[2], argv[3], argc >= 5 ? argv[4] : NULL);
#ifdef _WIN64
    const char *in_game[] = {"cap", "timescale", "stopat", "hook", "messages", "cursor", NULL};
    for (const char **verb = in_game; *verb; verb++)
        if (strcmp(argv[1], *verb) == 0) {
            fprintf(stderr, "%s runs inside the game: use drive.exe\n", *verb);
            return 2;
        }
    if (strcmp(argv[1], "selftest") == 0)
        return focus(hwnd) != 0 ? 1 : selftest(hwnd);
#else
    if (strcmp(argv[1], "timescale") == 0 && argc >= 3)
        return set_timescale(hwnd, argv[2]);
    if (strcmp(argv[1], "stopat") == 0 && argc >= 3)
        return set_stop(hwnd, argv[2]);
    if (strcmp(argv[1], "hook") == 0)
        return install_in_game(hwnd);
    if (strcmp(argv[1], "messages") == 0 && argc >= 4)
        return list_messages(hwnd, argv[2], argv[3]);
    if (strcmp(argv[1], "cursor") == 0 && argc >= 3)
        return set_cursor(hwnd, argv[2]);
    if (strcmp(argv[1], "cap") == 0 && argc >= 3)
        return capture_in_game(hwnd, argv[2]);
#endif

    /* Mouse verbs take one point, drag two; parse them before any input. */
    int points = 0, c[4];
    if ((strcmp(argv[1], "click") == 0 || strcmp(argv[1], "rclick") == 0 ||
         strcmp(argv[1], "dclick") == 0) && argc >= 4)
        points = 1;
    else if (strcmp(argv[1], "drag") == 0 && argc >= 6)
        points = 2;
    if (parse_points(argv + 2, points, c) != 0)
        return 2;

    if (focus(hwnd) != 0)
        return 1;

    if (points) {
        int refused;
        if (strcmp(argv[1], "click") == 0)
            refused = send_click(hwnd, c[0], c[1]);
        else if (strcmp(argv[1], "rclick") == 0)
            refused = send_rclick(hwnd, c[0], c[1]);
        else if (strcmp(argv[1], "dclick") == 0)
            refused = send_dclick(hwnd, c[0], c[1]);
        else
            refused = send_drag(hwnd, c[0], c[1], c[2], c[3]);
        if (refused)
            fprintf(stderr, "Wine refused some of the mouse input\n");
        return refused;
    }
    if (strcmp(argv[1], "key") == 0 && argc >= 3) {
        int count = argc >= 4 ? atoi(argv[3]) : 1;
        if (count < 1 || count > 200) {
            fprintf(stderr, "Count must be 1-200\n");
            return 2;
        }
        return send_key(hwnd, argv[2], count);
    }
    if (strcmp(argv[1], "type") == 0 && argc >= 3) {
        return type_text(argv[2]);
    }

    fprintf(stderr, "Unknown command: %s\n", argv[1]);
    return 2;
}

#endif /* !WINE_CAP_DLL */
