
void __thiscall FUN_0060e440(void *this,undefined4 param_1)

{
  *(undefined4 *)((int)this + 0x98) = param_1;
  if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  }
  return;
}

