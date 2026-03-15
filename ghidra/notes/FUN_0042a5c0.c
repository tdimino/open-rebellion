
void __thiscall FUN_0042a5c0(void *this,LPARAM param_1)

{
  HWND pHVar1;
  HWND pHVar2;
  
  PostMessageA(*(HWND *)((int)this + 0x18),0x409,0,0);
  if (*(int *)((int)this + 0xc0) == 2) {
    if (*(undefined4 **)((int)this + 0xc4) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0xc4))(1);
      *(undefined4 *)((int)this + 0xc4) = 0;
    }
    pHVar1 = *(HWND *)((int)this + 0x18);
    *(undefined4 *)((int)this + 0xc0) = 1;
    pHVar2 = GetCapture();
    if (pHVar2 == pHVar1) {
      ReleaseCapture();
    }
  }
  PostMessageA(*(HWND *)((int)this + 0x18),0x468,6,param_1);
  return;
}

