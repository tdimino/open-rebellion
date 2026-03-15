
void __thiscall FUN_0040fa70(void *this,int param_1,int param_2,int param_3)

{
  void *this_00;
  
  if ((*(int *)((int)this + 0x120) != param_1) || (param_2 != 0)) {
    this_00 = *(void **)((int)this + 0x150);
    *(int *)((int)this + 0x120) = param_1;
    if ((this_00 != (void *)0x0) && (*(int *)((int)this + 0x154) != 0)) {
      if (param_1 == 1) {
        FUN_006030f0(this_00,4);
        FUN_006030c0(*(void **)((int)this + 0x154),4);
      }
      else {
        FUN_006030c0(this_00,4);
        FUN_006030f0(*(void **)((int)this + 0x154),4);
      }
    }
    if (param_3 != 0) {
      FUN_0040fe10(this);
      FUN_0040fd20(this,0);
    }
  }
  return;
}

