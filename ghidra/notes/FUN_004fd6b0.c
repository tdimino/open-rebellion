
int __fastcall FUN_004fd6b0(undefined4 param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 local_4c [7];
  int local_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006402e0;
  pvStack_c = ExceptionList;
  iVar4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00502db0(local_4c,param_1,1);
  local_4 = 0;
  FUN_00513120((int)local_4c);
  while (local_30 != 0) {
    piVar1 = (int *)FUN_0052bed0((int)local_4c);
    iVar2 = (**(code **)(*piVar1 + 0x1dc))();
    uVar5 = 1;
    iVar4 = iVar4 + iVar2;
    uVar3 = FUN_0052bed0((int)local_4c);
    FUN_005039d0(auStack_2c,uVar3,uVar5);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00513120((int)auStack_2c);
    while (iStack_10 != 0) {
      piVar1 = (int *)FUN_0052bed0((int)auStack_2c);
      iVar2 = (**(code **)(*piVar1 + 0x1dc))();
      iVar4 = iVar4 + iVar2;
      FUN_005130d0((int)auStack_2c);
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00503ad0(auStack_2c);
    FUN_005130d0((int)local_4c);
  }
  local_4 = 0xffffffff;
  FUN_00502eb0(local_4c);
  ExceptionList = pvStack_c;
  return iVar4;
}

