
int __thiscall FUN_004f9650(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = FUN_0053a170(this,param_1);
  iVar2 = 0;
  if (iVar1 != 0) {
    FUN_005f60f0(param_1);
    iVar1 = FUN_005589b0(param_1);
    iVar2 = 0;
    if (iVar1 != 0) {
      iVar2 = FUN_005589b0(param_1);
      if (iVar2 != 0) {
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        if ((iVar2 != 0) && ((*(uint *)((int)this + 0x24) & 0x30) == 0)) {
          iVar2 = (**(code **)(**(int **)((int)this + 0x54) + 0xc))(param_1);
        }
      }
    }
  }
  return iVar2;
}

