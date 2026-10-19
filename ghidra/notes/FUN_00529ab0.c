
undefined4 __thiscall FUN_00529ab0(void *this,int param_1,int param_2,void *param_3)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = FUN_0052a2c0(this,param_3);
  uVar2 = FUN_0052a140(this,param_2,param_1 + 8,param_3);
  if ((uVar2 != 0) && (uVar1 != 0)) {
    return 1;
  }
  return 0;
}

