
undefined4 __thiscall FUN_0050c820(void *this,int param_1,int param_2,void *param_3)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = FUN_0053fa60(0x38e,(uint)(param_1 != 3),this,(int *)(*(int *)((int)this + 0x54) + 0x40),
                       param_3);
  uVar2 = FUN_0053fa60(0x38e,(uint)(param_2 != 3),this,(int *)(*(int *)((int)this + 0x54) + 0x40),
                       param_3);
  if ((uVar2 != 0) && (uVar1 != 0)) {
    return 1;
  }
  return 0;
}

