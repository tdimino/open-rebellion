
int FUN_00549a20(undefined4 param_1)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  int local_1c;
  undefined4 local_18;
  int *local_14;
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647c98;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  uVar1 = FUN_005434b0(&local_14,&local_10,&local_18);
  local_1c = 0;
  iVar2 = FUN_0055c650(&local_1c);
  if ((iVar2 == 0) || (uVar1 == 0)) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  if (local_14 != (int *)0x0) {
    iVar3 = (**(code **)(*local_14 + 0x2f0))(local_1c,10,param_1);
    if ((iVar3 == 0) || (iVar2 == 0)) {
      iVar2 = 0;
    }
    else {
      iVar2 = 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar2;
}

