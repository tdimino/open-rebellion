
undefined4 __thiscall FUN_00612580(void *this,int *param_1,int *param_2)

{
  void *this_00;
  undefined4 uVar1;
  
  this_00 = FUN_00612770(this,param_2);
  uVar1 = 0;
  if (this_00 != (void *)0x0) {
    FUN_00616cd0(param_1,8,1);
    FUN_006138b0(this_00,param_1);
    if (*param_2 == 2) {
      FUN_006168a0(param_1);
      *(undefined4 *)((int)this + 0x4c) = 0;
      *(undefined4 *)((int)this + 0x50) = 0;
      return *(undefined4 *)((int)this_00 + 0x18);
    }
    FUN_00613890(this_00,*(undefined4 *)((int)this + 0x4c),
                 *(int *)((int)this + 0x50) + 8 + *(int *)((int)this + 0x3c));
    uVar1 = *(undefined4 *)((int)this_00 + 0x18);
  }
  return uVar1;
}

