
void __thiscall FUN_00408240(void *this,int param_1,int param_2)

{
  if (param_1 != 0) {
    FUN_00417020(0x474,(int)this,0,0x1217,0x1300);
    *(undefined4 *)((int)this + param_2 * 4 + 0xa8) = 1;
    FUN_00408f10((int)this);
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
    return;
  }
  FUN_00417020(0x475,(int)this,0,0x1219,0x1300);
  return;
}

