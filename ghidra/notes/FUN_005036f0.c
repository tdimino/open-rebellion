
uint __fastcall FUN_005036f0(int *param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  uint uVar5;
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640af8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = thunk_FUN_005581b0(param_1);
  FUN_004fcd00(local_2c,param_1,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    uVar4 = 0;
    if (iVar2 == 0) goto LAB_00503790;
    iVar2 = 0;
    FUN_005130d0((int)local_2c);
  }
  uVar4 = 0;
  if (iVar2 != 0) {
    iVar2 = param_1[0x18];
    if ((iVar2 < 0) || (iVar3 = (**(code **)(*param_1 + 0x244))(), iVar3 < iVar2)) {
      uVar4 = 0;
    }
    else {
      uVar4 = 1;
    }
    if ((uVar4 != 0) &&
       (iVar2 = param_1[0x18], iVar3 = (**(code **)(*param_1 + 0x244))(), iVar2 == iVar3)) {
      uVar4 = (uint)param_1[0x14] >> 3 & 1;
    }
  }
LAB_00503790:
  local_30 = 0;
  uVar5 = 0;
  if (uVar4 != 0) {
    bVar1 = FUN_004f6b50(param_1,&local_30);
    uVar5 = CONCAT31(extraout_var,bVar1);
    if ((uVar5 != 0) && (((param_1[0x14] & 8U) == 0 || ((param_1[0x14] & 0x4000U) == 0)))) {
      uVar5 = ~*(uint *)(local_30 + 0x50) >> 3 & 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return uVar5;
}

