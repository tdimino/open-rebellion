
uint __thiscall FUN_005c5d80(void *param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = FUN_005c61d0(param_1,*(int *)((int)param_2 + 0x18));
  if (iVar1 == 0) {
    uVar2 = FUN_005c61a0(param_1,param_2);
    return uVar2;
  }
  return 0;
}

