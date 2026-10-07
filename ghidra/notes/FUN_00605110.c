
void __thiscall FUN_00605110(void *this,int param_1,int param_2)

{
  if ((param_1 != *(int *)((int)this + 0xb4)) || (param_2 != *(int *)((int)this + 0xb8))) {
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  }
  *(int *)((int)this + 0xb4) = param_1;
  *(int *)((int)this + 0xb8) = param_2;
  FUN_00605fc0(this);
  FUN_006060f0(this);
  return;
}

