
void __thiscall FUN_006008e0(void *this,LPARAM param_1)

{
  DWORD dwThreadId;
  
  if (DAT_006be5a4 == (HWND)0x0) {
    DAT_006be5a4 = CreateWindowExA(0,s_tooltips_class32_006ac7c0,(LPCSTR)0x0,1,-0x80000000,
                                   -0x80000000,-0x80000000,-0x80000000,*(HWND *)((int)this + 0x18),
                                   (HMENU)0x0,*(HINSTANCE *)((int)this + 0x1c),(LPVOID)0x0);
    if (DAT_006be5a4 != (HWND)0x0) {
      SendMessageA(DAT_006be5a4,0x401,1,0);
      SendMessageA(DAT_006be5a4,0x403,0,param_1);
      dwThreadId = GetCurrentThreadId();
      DAT_006be5a8 = SetWindowsHookExA(3,(HOOKPROC)&lpfn_00600e50,(HINSTANCE)0x0,dwThreadId);
    }
  }
  return;
}

