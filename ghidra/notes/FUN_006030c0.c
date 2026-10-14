
void __thiscall FUN_006030c0(void *this,uint param_1)

{
  *(uint *)((int)this + 0xc4) = *(uint *)((int)this + 0xc4) | param_1;
  if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  }
  return;
}

