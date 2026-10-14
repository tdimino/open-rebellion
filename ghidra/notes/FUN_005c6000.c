
uint __thiscall FUN_005c6000(void *param_1,int *param_2)

{
  uint uVar1;
  int iVar2;
  
  uVar1 = (**(code **)(*param_2 + 4))();
  iVar2 = FUN_005ca4b0(param_1,uVar1);
  if (iVar2 == 0) {
    uVar1 = FUN_005f5440(param_1,param_2);
    return uVar1;
  }
  return 0;
}

