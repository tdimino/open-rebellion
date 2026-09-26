
int * __fastcall FUN_004d0d30(uint param_1)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  int *piVar5;
  void *pvVar6;
  uint *puVar7;
  int iVar8;
  void *pvVar9;
  int *local_64;
  uint local_60;
  void *local_5c;
  undefined4 local_58;
  undefined4 local_54 [2];
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063beaa;
  pvStack_c = ExceptionList;
  pvVar9 = (void *)0x0;
  bVar1 = false;
  local_64 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_60 = param_1;
  FUN_004f4340(local_54);
  uVar3 = *(uint *)(param_1 + 0x5c) >> 0x18;
  local_4 = 0;
  local_5c = (void *)0x90;
  local_58 = 0x98;
  if ((uVar3 < 0x90) || (bVar2 = true, 0x97 < uVar3)) {
    bVar2 = false;
  }
  FUN_00619730();
  if (bVar2) {
    iVar8 = *(int *)(param_1 + 0x2c);
    FUN_004f26d0(&stack0xffffff80,(uint *)(param_1 + 0x5c));
    pvVar4 = FUN_00403d30((void *)(iVar8 + 0x2c));
    uVar3 = local_60;
    if (pvVar4 != (void *)0x0) {
      piVar5 = FUN_004f3220(*(int *)(local_60 + 0x30),(uint *)(param_1 + 0x5c));
      if (piVar5 != (int *)0x0) {
        if ((*(uint *)((int)pvVar4 + 0x28) & 0x200000) == 0) {
          FUN_0052cab0(local_4c,piVar5,1);
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00513120((int)local_4c);
          while (local_30 != 0) {
            pvVar9 = (void *)FUN_00618b70(0x20);
            local_4._0_1_ = 2;
            local_5c = pvVar9;
            if (pvVar9 == (void *)0x0) {
              pvVar9 = (void *)0x0;
            }
            else {
              puVar7 = &local_60;
              pvVar6 = (void *)FUN_0052bed0((int)local_4c);
              puVar7 = FUN_004025b0(pvVar6,puVar7);
              bVar1 = true;
              local_4 = CONCAT31(local_4._1_3_,3);
              pvVar9 = FUN_004f5b10(pvVar9,puVar7,0);
            }
            local_4 = 2;
            if (bVar1) {
              bVar1 = false;
              FUN_00619730();
            }
            local_4 = CONCAT31(local_4._1_3_,1);
            if (pvVar9 != (void *)0x0) {
              FUN_004f57b0(local_54,pvVar9);
            }
            FUN_005130d0((int)local_4c);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_0052cbb0(local_4c);
        }
        bVar1 = false;
        if ((*(uint *)((int)pvVar4 + 0x28) & 0x3800000) == 0) {
          FUN_0052c420(local_4c,piVar5,1);
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00513120((int)local_4c);
          while (local_30 != 0) {
            pvVar9 = (void *)FUN_00618b70(0x20);
            local_4._0_1_ = 5;
            local_5c = pvVar9;
            if (pvVar9 == (void *)0x0) {
              pvVar9 = (void *)0x0;
            }
            else {
              puVar7 = &local_60;
              pvVar6 = (void *)FUN_0052bed0((int)local_4c);
              puVar7 = FUN_004025b0(pvVar6,puVar7);
              bVar1 = true;
              local_4 = CONCAT31(local_4._1_3_,6);
              pvVar9 = FUN_004f5b10(pvVar9,puVar7,0);
            }
            local_4 = 5;
            if (bVar1) {
              bVar1 = false;
              FUN_00619730();
            }
            local_4 = CONCAT31(local_4._1_3_,4);
            if (pvVar9 != (void *)0x0) {
              FUN_004f57b0(local_54,pvVar9);
            }
            FUN_005130d0((int)local_4c);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_0052c520(local_4c);
        }
        bVar1 = false;
        FUN_0051c270(local_2c,piVar5,1);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          pvVar9 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 8;
          local_5c = pvVar9;
          if (pvVar9 == (void *)0x0) {
            pvVar9 = (void *)0x0;
          }
          else {
            puVar7 = &local_60;
            pvVar6 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar6,puVar7);
            bVar1 = true;
            local_4 = CONCAT31(local_4._1_3_,9);
            pvVar9 = FUN_004f5b10(pvVar9,puVar7,0);
          }
          local_4 = 8;
          if (bVar1) {
            bVar1 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,7);
          if (pvVar9 != (void *)0x0) {
            FUN_004f57b0(local_54,pvVar9);
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0051c2f0(local_2c);
        if ((*(int *)((int)pvVar4 + 0x84) == 0) && (pvVar9 == (void *)0x0)) {
          pvVar9 = (void *)0x0;
          if ((*(int *)(*(int *)(uVar3 + 0x2c) + 0x1d0) < *(int *)(*(int *)(uVar3 + 0x2c) + 0x1d4))
             || (*(int *)((int)pvVar4 + 0x50) == 0)) {
            FUN_0052d1f0(local_2c,piVar5,1);
            local_4._0_1_ = 10;
            FUN_00513120((int)local_2c);
            if (local_10 != 0) {
              pvVar6 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 0xb;
              local_5c = pvVar6;
              if (pvVar6 == (void *)0x0) {
                pvVar9 = (void *)0x0;
              }
              else {
                puVar7 = &local_60;
                pvVar9 = (void *)FUN_0052bed0((int)local_2c);
                puVar7 = FUN_004025b0(pvVar9,puVar7);
                local_4 = CONCAT31(local_4._1_3_,0xc);
                pvVar9 = FUN_004f5b10(pvVar6,puVar7,0);
              }
              local_4 = 0xb;
              if (pvVar6 != (void *)0x0) {
                FUN_00619730();
              }
              local_4._0_1_ = 10;
              if (pvVar9 != (void *)0x0) {
                FUN_004f57b0(local_54,pvVar9);
              }
            }
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_0052d2f0(local_2c);
          }
          if ((pvVar9 == (void *)0x0) &&
             ((*(int *)(*(int *)(uVar3 + 0x2c) + 0x1d4) <= *(int *)(*(int *)(uVar3 + 0x2c) + 0x1d0)
              || (*(int *)((int)pvVar4 + 0x54) == 0)))) {
            FUN_0052d610(local_2c,piVar5,1);
            local_4._0_1_ = 0xd;
            FUN_00513120((int)local_2c);
            if (local_10 != 0) {
              pvVar9 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 0xe;
              local_5c = pvVar9;
              if (pvVar9 == (void *)0x0) {
                pvVar4 = (void *)0x0;
              }
              else {
                puVar7 = &local_60;
                pvVar4 = (void *)FUN_0052bed0((int)local_2c);
                puVar7 = FUN_004025b0(pvVar4,puVar7);
                local_4 = CONCAT31(local_4._1_3_,0xf);
                pvVar4 = FUN_004f5b10(pvVar9,puVar7,0);
              }
              local_4 = 0xe;
              if (pvVar9 != (void *)0x0) {
                FUN_00619730();
              }
              local_4._0_1_ = 0xd;
              if (pvVar4 != (void *)0x0) {
                FUN_004f57b0(local_54,pvVar4);
              }
            }
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_0052d710(local_2c);
          }
        }
      }
      iVar8 = thunk_FUN_005f5060((int)local_54);
      if ((iVar8 != 0) && (local_64 = (int *)FUN_004f5cd0(0x200), local_64 != (int *)0x0)) {
        local_64[8] = *(int *)(uVar3 + 0x30);
        (**(code **)(*local_64 + 0x24))();
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_54);
  ExceptionList = pvStack_c;
  return local_64;
}

