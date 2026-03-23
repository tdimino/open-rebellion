
int __thiscall FUN_0050be00(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined4 uVar3;
  undefined3 extraout_var;
  void *this_00;
  int iVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  undefined4 uVar8;
  void *pvVar9;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641c60;
  pvStack_c = ExceptionList;
  iVar7 = 1;
  iVar5 = 0;
  iVar4 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ffe70(local_4c,this,4);
  local_4 = 0;
  FUN_00513120((int)local_4c);
  while (local_30 != 0) {
    iVar2 = FUN_0052bed0((int)local_4c);
    uVar6 = *(uint *)(iVar2 + 0x24) >> 6 & 3;
    iVar2 = FUN_0052bed0((int)local_4c);
    if ((((*(uint *)(iVar2 + 0x58) >> 1 & 1) == 0) ||
        (iVar2 = FUN_0052bed0((int)local_4c), (*(uint *)(iVar2 + 0x58) >> 2 & 1) == 0)) &&
       (iVar7 != 0)) {
      iVar7 = 1;
    }
    else {
      iVar7 = 0;
    }
    if ((iVar7 != 0) &&
       (iVar2 = FUN_0052bed0((int)local_4c), (*(uint *)(iVar2 + 0x58) >> 2 & 1) != 0)) {
      uVar8 = 4;
      uVar3 = FUN_0052bed0((int)local_4c);
      FUN_004fcd00(local_2c,uVar3,uVar8);
      local_4 = CONCAT31(local_4._1_3_,1);
      bVar1 = FUN_005131b0((int)local_2c);
      if ((CONCAT31(extraout_var,bVar1) == 0) &&
         (iVar2 = FUN_0052bed0((int)local_4c), (*(uint *)(iVar2 + 0x50) >> 3 & 1) == 0)) {
        iVar5 = iVar5 + (uint)(uVar6 == 1);
        iVar4 = iVar4 + (uint)(uVar6 == 2);
      }
      else {
        iVar2 = 0;
        pvVar9 = param_1;
        this_00 = (void *)FUN_0052bed0((int)local_4c);
        FUN_004fe310(this_00,iVar2,pvVar9);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_004fcd80(local_2c);
    }
    FUN_005130d0((int)local_4c);
  }
  if (iVar7 != 0) {
    if ((iVar5 < 2) && (iVar4 < 2)) {
      iVar7 = 1;
    }
    else {
      iVar7 = 0;
    }
  }
  if (iVar5 == 0) {
    iVar5 = FUN_00512d00(this,1,0,param_1);
    if ((iVar5 == 0) || (iVar7 == 0)) {
      iVar7 = 0;
    }
    else {
      iVar7 = 1;
    }
  }
  if (iVar4 == 0) {
    iVar5 = FUN_00512d00(this,2,0,param_1);
    if ((iVar5 == 0) || (iVar7 == 0)) {
      iVar7 = 0;
    }
    else {
      iVar7 = 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_004fff70(local_4c);
  ExceptionList = pvStack_c;
  return iVar7;
}

