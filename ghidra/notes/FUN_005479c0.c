
bool FUN_005479c0(uint *param_1,uint *param_2,uint *param_3,uint param_4)

{
  int *piVar1;
  undefined4 *puVar2;
  uint uVar3;
  int iVar4;
  int *piVar5;
  undefined3 extraout_var;
  uint *puVar6;
  bool bVar7;
  bool bVar8;
  uint local_28;
  uint local_24;
  undefined4 local_20 [2];
  void *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006479a0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  piVar1 = (int *)FUN_00505d40(param_3);
  bVar8 = piVar1 != (int *)0x0;
  FUN_004ece30(&local_24);
  local_4 = 0;
  if (piVar1 != (int *)0x0) {
    puVar2 = FUN_0050c640(piVar1);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_24,puVar2);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  FUN_004ece30(&local_28);
  local_4._0_1_ = 2;
  if (piVar1 != (int *)0x0) {
    uVar3 = (uint)piVar1[9] >> 6 & 3;
    if (uVar3 == 1) {
      iVar4 = 1;
    }
    else if (uVar3 == 2) {
      iVar4 = 2;
    }
    else {
      iVar4 = 0;
    }
    piVar5 = FUN_004f2d10(iVar4,param_1);
    bVar8 = piVar5 != (int *)0x0;
    if (piVar5 != (int *)0x0) {
      puVar2 = FUN_0050c640(piVar5);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_28,puVar2);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
  }
  FUN_004f4340(local_14);
  local_4._0_1_ = 4;
  bVar7 = false;
  if (bVar8) {
    bVar8 = FUN_004f44b0(local_14,param_3,0);
    bVar7 = CONCAT31(extraout_var,bVar8) != 0;
  }
  puVar6 = FUN_004ece40(&local_28);
  if (puVar6 != (uint *)0x0) {
    puVar6 = FUN_004ece40(&local_24);
    if (puVar6 != (uint *)0x0) {
      puVar2 = FUN_004f4340(local_20);
      local_4._0_1_ = 5;
      iVar4 = FUN_00547ba0(local_18,0,0,(uint)piVar1[9] >> 6 & 3,param_1,param_2,(int *)&local_28,
                           &local_24,(int)local_14,puVar2,(void *)0x0,0,1,param_4);
      if ((iVar4 == 0) || (!bVar7)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      local_4._0_1_ = 4;
      FUN_004f4380(local_20);
    }
  }
  local_4._0_1_ = 2;
  FUN_004f4380(local_14);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return bVar7;
}

