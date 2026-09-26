
undefined4 __thiscall FUN_0058d080(void *this,undefined4 *param_1,int *param_2)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650008;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = 0;
  *param_2 = 0;
  param_2[1] = 0;
  param_2[2] = 0;
  param_2[3] = 0;
  FUN_0053b760(local_2c,*(int *)((int)this + 8),1,*(uint *)(*(int *)((int)this + 8) + 0x24) >> 6 & 3
              );
  local_4 = 0;
  iVar1 = FUN_00513180((int)local_2c);
  if (iVar1 != 0) {
    iVar1 = FUN_0053e290(iVar1 + -1);
    iVar2 = 0;
    iVar3 = 0;
    FUN_00513120((int)local_2c);
    while ((local_10 != 0 && (iVar3 == 0))) {
      if (iVar2 < iVar1) {
        iVar2 = iVar2 + 1;
      }
      else {
        iVar3 = FUN_0052bed0((int)local_2c);
      }
    }
    *param_2 = iVar3;
    *param_1 = 1;
  }
  local_4 = 0xffffffff;
  FUN_0053b7e0(local_2c);
  ExceptionList = pvStack_c;
  return 1;
}

