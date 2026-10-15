
void __thiscall FUN_004950d0(int param_1,int param_2,undefined4 param_3)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  undefined4 *puVar7;
  undefined4 uVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00636010;
  pvStack_c = ExceptionList;
  if (param_2 != 2) {
    if (param_2 != 0x182) {
      return;
    }
    ExceptionList = &pvStack_c;
    iVar1 = FUN_004f36a0(*(undefined4 *)(param_1 + 0x20),param_3);
    if (iVar1 == 0) {
      ExceptionList = pvStack_c;
      return;
    }
    FUN_005003d0(iVar1,3);
    iStack_4 = 0;
    FUN_00513120();
    iVar1 = FUN_0052bed0();
    if (iVar1 != 0) {
      puVar7 = &param_2;
      FUN_0052bed0(puVar7);
      uVar2 = FUN_004025b0(puVar7);
      iStack_4._0_1_ = 1;
      FUN_004f26d0(uVar2);
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      FUN_00619730();
    }
    *(undefined4 *)(param_1 + 0x40) = 1;
    iStack_4 = 0xffffffff;
    FUN_005004d0();
    ExceptionList = pvStack_c;
    return;
  }
  ExceptionList = &pvStack_c;
  FUN_004f3dd0(*(undefined4 *)(param_1 + 0x20),2);
  if (*(int *)(param_1 + 100) == 1) {
    uVar2 = *(undefined4 *)(param_1 + 0x20);
    uVar8 = 1;
  }
  else {
    if (*(int *)(param_1 + 100) != 2) {
      iVar1 = 0;
      goto LAB_004951ca;
    }
    uVar2 = *(undefined4 *)(param_1 + 0x20);
    uVar8 = 2;
  }
  iVar1 = FUN_004f3dd0(uVar2,uVar8);
LAB_004951ca:
  iVar3 = FUN_004f3220(*(undefined4 *)(param_1 + 0x20),param_1 + 0x60);
  iVar4 = FUN_004f2d10(*(undefined4 *)(param_1 + 0x20),param_1 + 0x68);
  if (iVar3 == 0) {
    iVar6 = 0;
  }
  else {
    iVar6 = iVar3 + 0x30;
  }
  if (iVar4 == 0) {
    iVar5 = 0;
  }
  else {
    iVar5 = iVar4 + 0x30;
  }
  FUN_0060b9d0(param_1 + 0x44,0x70c8,iVar5,iVar6,0,0);
  if (iVar1 == 0) {
    if (iVar3 == 0) {
      iVar6 = 0;
    }
    else {
      iVar6 = iVar3 + 0x30;
    }
    if (iVar4 == 0) {
      iVar1 = 0;
    }
    else {
      iVar1 = iVar4 + 0x30;
    }
    iVar3 = 0;
    uVar2 = 0x70ca;
  }
  else {
    if (iVar3 == 0) {
      iVar3 = 0;
    }
    else {
      iVar3 = iVar3 + 0x30;
    }
    if (iVar4 == 0) {
      iVar6 = 0;
    }
    else {
      iVar6 = iVar4 + 0x30;
    }
    iVar1 = iVar1 + 0x30;
    uVar2 = 0x70c9;
  }
  FUN_0060b9d0(param_1 + 0x50,uVar2,iVar1,iVar6,iVar3,0);
  *(undefined4 *)(param_1 + 0x5c) = 0x6f;
  ExceptionList = pvStack_c;
  return;
}

