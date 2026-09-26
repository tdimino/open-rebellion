
int * __fastcall FUN_004d7330(int param_1)

{
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  int *piVar4;
  void *pvVar5;
  uint *puVar6;
  int iVar7;
  int *piVar8;
  undefined1 *puVar9;
  undefined1 **ppuVar10;
  uint local_68;
  undefined1 *local_64;
  int local_60;
  int *local_5c;
  void *local_58;
  undefined4 local_54 [2];
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c986;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_5c = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_60 = param_1;
  FUN_004f4340(local_54);
  iVar7 = *(int *)(param_1 + 0x2c);
  local_64 = &stack0xffffff80;
  local_4 = 0;
  FUN_004f26d0(&stack0xffffff80,(uint *)(param_1 + 0x54));
  pvVar3 = FUN_00403d30((void *)(iVar7 + 0x2c));
  piVar8 = local_5c;
  if (pvVar3 != (void *)0x0) {
    local_64 = (undefined1 *)(*(int *)((int)pvVar3 + 0x74) - *(int *)((int)pvVar3 + 0x7c));
    piVar8 = (int *)(*(int *)((int)pvVar3 + 100) - *(int *)((int)pvVar3 + 0x6c));
  }
  piVar4 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x54));
  if (piVar4 != (int *)0x0) {
    if (0 < (int)piVar8) {
      FUN_005278c0(local_4c,piVar4,1);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00513120((int)local_4c);
      for (; (local_30 != 0 && (0 < (int)piVar8)); piVar8 = (int *)((int)piVar8 + -1)) {
        pvVar3 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 2;
        local_58 = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          puVar6 = &local_68;
          pvVar5 = (void *)FUN_0052bed0((int)local_4c);
          puVar6 = FUN_004025b0(pvVar5,puVar6);
          bVar1 = true;
          local_4 = CONCAT31(local_4._1_3_,3);
          pvVar3 = FUN_004f5b10(pvVar3,puVar6,0);
        }
        local_4 = 2;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4 = CONCAT31(local_4._1_3_,1);
        if (pvVar3 != (void *)0x0) {
          FUN_004f57b0(local_54,pvVar3);
        }
        FUN_005130d0((int)local_4c);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_005279c0(local_4c);
    }
    puVar9 = local_64;
    bVar1 = false;
    if (0 < (int)local_64) {
      FUN_00527550(local_2c,piVar4,1);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00513120((int)local_2c);
      while (bVar2 = false, local_10 != 0) {
        if ((int)puVar9 < 1) goto LAB_004d762b;
        pvVar3 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 5;
        local_58 = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          ppuVar10 = &local_64;
          pvVar5 = (void *)FUN_0052bed0((int)local_2c);
          puVar6 = FUN_004025b0(pvVar5,(uint *)ppuVar10);
          bVar1 = true;
          local_4 = CONCAT31(local_4._1_3_,6);
          pvVar3 = FUN_004f5b10(pvVar3,puVar6,0);
        }
        local_4 = 5;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4 = CONCAT31(local_4._1_3_,4);
        if (pvVar3 != (void *)0x0) {
          FUN_004f57b0(local_54,pvVar3);
        }
        puVar9 = puVar9 + -1;
        FUN_005130d0((int)local_2c);
      }
      if (0 < (int)puVar9) {
        FUN_005272e0(local_4c,piVar4,1);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00513120((int)local_4c);
        for (; (local_30 != 0 && (0 < (int)puVar9)); puVar9 = puVar9 + -1) {
          pvVar3 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 8;
          local_58 = pvVar3;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            ppuVar10 = &local_64;
            pvVar5 = (void *)FUN_0052bed0((int)local_4c);
            puVar6 = FUN_004025b0(pvVar5,(uint *)ppuVar10);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,9);
            pvVar3 = FUN_004f5b10(pvVar3,puVar6,0);
          }
          local_4 = 8;
          if (bVar2) {
            bVar2 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,7);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_54,pvVar3);
          }
          FUN_005130d0((int)local_4c);
        }
        local_4._1_3_ = (undefined3)(local_4 >> 8);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00527360(local_4c);
      }
LAB_004d762b:
      local_4 = local_4 & 0xffffff00;
      FUN_00527650(local_2c);
    }
  }
  iVar7 = thunk_FUN_005f5060((int)local_54);
  piVar8 = local_5c;
  if ((iVar7 != 0) && (piVar8 = (int *)FUN_004f5cd0(0x200), piVar8 != (int *)0x0)) {
    piVar8[8] = *(int *)(local_60 + 0x30);
    (**(code **)(*piVar8 + 0x24))();
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_54);
  ExceptionList = pvStack_c;
  return piVar8;
}

