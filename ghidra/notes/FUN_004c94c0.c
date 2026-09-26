
int * __fastcall FUN_004c94c0(int param_1)

{
  int iVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *pvVar4;
  int *piVar5;
  uint *puVar6;
  bool bVar7;
  uint local_a4;
  int local_a0;
  uint local_9c;
  int *local_98;
  undefined1 *local_94;
  undefined1 local_90 [4];
  undefined4 local_8c [4];
  undefined4 local_7c [2];
  undefined1 local_74 [4];
  undefined1 local_70 [4];
  undefined1 local_6c [4];
  undefined1 local_68 [4];
  undefined1 local_64 [4];
  undefined1 local_60 [4];
  undefined1 local_5c [4];
  undefined1 local_58 [4];
  undefined1 local_54 [4];
  undefined1 local_50 [4];
  undefined1 local_4c [4];
  undefined1 local_48 [4];
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b103;
  pvStack_c = ExceptionList;
  local_98 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004355d0(local_8c);
  local_4 = 0;
  local_a0 = 0;
  FUN_004ece30(&local_9c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_a4);
  local_4._0_1_ = 2;
  FUN_004f4340(local_7c);
  local_4._0_1_ = 3;
  pvVar2 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),5,0x3802,0,&DAT_00000002);
  FUN_00435790((void *)(param_1 + 0x40),(int)pvVar2);
  puVar3 = FUN_004357b0((void *)(param_1 + 0x40),local_90);
  local_4._0_1_ = 4;
  FUN_004f26d0((void *)(param_1 + 0x54),puVar3);
  do {
    local_4._0_1_ = 3;
    puVar6 = (uint *)(param_1 + 0x54);
    FUN_00619730();
    local_44 = 8;
    local_40 = 0x10;
    if (((*puVar6 >> 0x18 < 8) || (0xf < *puVar6 >> 0x18)) || (local_a0 != 0)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    FUN_00619730();
    if (!bVar7) {
      FUN_005f58b0((void *)(param_1 + 0x40));
      local_4._0_1_ = 2;
      FUN_004f4380(local_7c);
      local_4._0_1_ = 1;
      FUN_00619730();
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00435610(local_8c);
      ExceptionList = pvStack_c;
      return local_98;
    }
    iVar1 = *(int *)(param_1 + 0x2c);
    local_94 = &stack0xffffff48;
    FUN_004f26d0(&stack0xffffff48,puVar6);
    pvVar2 = FUN_00419e40((void *)(iVar1 + 0x78));
    if (pvVar2 != (void *)0x0) {
      if ((*(uint *)((int)pvVar2 + 0x38) & 0x400) == 0) {
        iVar1 = *(int *)(param_1 + 0x2c);
        local_94 = &stack0xffffff48;
        FUN_004f26d0(&stack0xffffff48,(undefined4 *)((int)pvVar2 + 0x28));
        pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
        if (pvVar4 != (void *)0x0) {
          if ((*(byte *)((int)pvVar2 + 0x38) & 0x10) != 0) {
            iVar1 = *(int *)(param_1 + 0x2c);
            local_94 = &stack0xffffff48;
            local_a4 = 0x90000109;
            FUN_004f26d0(&stack0xffffff48,&local_a4);
            pvVar2 = FUN_00403d30((void *)(iVar1 + 0x2c));
            if ((pvVar2 != (void *)0x0) && ((*(byte *)((int)pvVar2 + 0x30) & 1) != 0)) {
              puVar6 = FUN_00403040(pvVar4,local_70);
              bVar7 = *puVar6 != local_a4;
              FUN_00619730();
              if (bVar7) {
                local_a0 = 1;
              }
            }
          }
          if ((local_a0 == 0) &&
             (((*(uint *)((int)pvVar4 + 0x30) & 1) == 0 ||
              ((*(uint *)((int)pvVar4 + 0x30) & 0x40000) == 0)))) {
            puVar6 = FUN_004bd7d0(pvVar4,local_6c);
            local_4._0_1_ = 8;
            pvVar2 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),puVar6);
            local_4._0_1_ = 3;
            FUN_00619730();
            if (pvVar2 == (void *)0x0) goto LAB_004c9b4f;
            puVar3 = FUN_00403040(pvVar2,local_54);
            local_4._0_1_ = 9;
            puVar3 = FUN_0041a400(*(void **)(param_1 + 0x2c),puVar3,(undefined4 *)(param_1 + 0x54),0
                                  ,0,0x40001,2,0,0,(undefined4 *)&DAT_00000002);
            local_4._0_1_ = 3;
            FUN_00619730();
            FUN_00435790(local_8c,(int)puVar3);
            puVar3 = FUN_004357b0(local_8c,local_58);
            local_4._0_1_ = 10;
            FUN_004f26d0(&local_a4,puVar3);
            local_4._0_1_ = 3;
            FUN_00619730();
            FUN_005f58b0(local_8c);
            local_1c = 0x90;
            local_18 = 0x98;
            if ((local_a4 >> 0x18 < 0x90) || (0x97 < local_a4 >> 0x18)) {
              bVar7 = false;
            }
            else {
              bVar7 = true;
            }
            FUN_00619730();
            if (bVar7) {
              local_a0 = 1;
            }
            if (local_a0 == 0) {
              if ((*(uint *)((int)pvVar2 + 0x24) & 0x40000000) != 0) {
                puVar3 = FUN_00403040(pvVar2,local_68);
                local_4._0_1_ = 0xb;
                puVar3 = FUN_0041a400(*(void **)(param_1 + 0x2c),puVar3,
                                      (undefined4 *)(param_1 + 0x54),0x80000000,0,1,2,0,0,
                                      (undefined4 *)&DAT_00000002);
                local_4._0_1_ = 3;
                FUN_00619730();
                FUN_00435790(local_8c,(int)puVar3);
                puVar3 = FUN_004357b0(local_8c,local_50);
                local_4._0_1_ = 0xc;
                FUN_004f26d0(&local_a4,puVar3);
                local_4._0_1_ = 3;
                FUN_00619730();
                FUN_005f58b0(local_8c);
                local_2c = 0x90;
                local_28 = 0x98;
                if ((local_a4 >> 0x18 < 0x90) || (0x97 < local_a4 >> 0x18)) {
                  bVar7 = false;
                }
                else {
                  bVar7 = true;
                }
                FUN_00619730();
                if (bVar7) {
                  local_a0 = 1;
                }
              }
              if (local_a0 == 0) {
                puVar6 = FUN_00403040(pvVar2,local_74);
                local_4._0_1_ = 0xd;
                puVar3 = FUN_0041b2b0(*(void **)(param_1 + 0x2c),local_48,puVar6,0,0,0x40001,2,0,0);
                local_4._0_1_ = 0xe;
                FUN_004f26d0(&local_a4,puVar3);
                local_4._0_1_ = 0xd;
                FUN_00619730();
                local_4._0_1_ = 3;
                FUN_00619730();
                local_3c = 0x90;
                local_38 = 0x98;
                if ((local_a4 >> 0x18 < 0x90) || (0x97 < local_a4 >> 0x18)) {
                  bVar7 = false;
                }
                else {
                  bVar7 = true;
                }
                FUN_00619730();
                if (bVar7) {
                  local_a0 = 1;
                }
                if (local_a0 == 0) {
                  puVar6 = FUN_00403040(pvVar2,local_64);
                  local_4._0_1_ = 0xf;
                  puVar3 = FUN_0041b2b0(*(void **)(param_1 + 0x2c),local_4c,puVar6,0,0,1,2,0,0);
                  local_4._0_1_ = 0x10;
                  FUN_004f26d0(&local_a4,puVar3);
                  local_4._0_1_ = 0xf;
                  FUN_00619730();
                  local_4._0_1_ = 3;
                  FUN_00619730();
                  local_34 = 0x90;
                  local_30 = 0x98;
                  if ((local_a4 >> 0x18 < 0x90) || (0x97 < local_a4 >> 0x18)) {
                    bVar7 = false;
                  }
                  else {
                    bVar7 = true;
                  }
                  FUN_00619730();
                  if (bVar7) {
                    local_a0 = 1;
                  }
                  goto LAB_004c9b4f;
                }
              }
            }
          }
          else {
LAB_004c9b4f:
            if (local_a0 == 0) goto LAB_004c9c10;
          }
          local_24 = 0x90;
          local_20 = 0x98;
          if ((local_a4 >> 0x18 < 0x90) || (0x97 < local_a4 >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          FUN_00619730();
          if ((bVar7) && (local_98 = (int *)FUN_004f5cd0(0x201), local_98 != (int *)0x0)) {
            local_94 = (undefined1 *)FUN_00618b70(0x20);
            local_4._0_1_ = 0x11;
            if (local_94 == (undefined1 *)0x0) {
              pvVar2 = (void *)0x0;
            }
            else {
              pvVar2 = FUN_004f5b10(local_94,(undefined4 *)(param_1 + 0x54),0);
            }
            local_4._0_1_ = 3;
            if (pvVar2 != (void *)0x0) {
              FUN_004f57b0(local_7c,pvVar2);
              piVar5 = local_98;
              local_98[8] = *(int *)(param_1 + 0x30);
              (**(code **)(*local_98 + 0x24))();
              (**(code **)(*piVar5 + 0x2c))(&stack0xffffff58);
            }
          }
        }
      }
      else if ((*(uint *)((int)pvVar2 + 0x38) & 0x4000) != 0) {
        pvVar4 = FUN_0041ac80(*(void **)(param_1 + 0x2c),puVar6,0x4000,0x803800,0,(void *)0x1);
        FUN_00435790(local_8c,(int)pvVar4);
        puVar3 = FUN_004357b0(local_8c,local_90);
        local_4._0_1_ = 5;
        FUN_004f26d0(&local_9c,puVar3);
        while( true ) {
          local_4._0_1_ = 3;
          FUN_00619730();
          local_14 = 0x14;
          local_10 = 0x1c;
          if ((local_9c >> 0x18 < 0x14) || (0x1b < local_9c >> 0x18)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          FUN_00619730();
          if (!bVar7) break;
          local_94 = (undefined1 *)FUN_00618b70(0x20);
          local_4._0_1_ = 6;
          if (local_94 == (undefined1 *)0x0) {
            pvVar4 = (void *)0x0;
          }
          else {
            pvVar4 = FUN_004f5b10(local_94,&local_9c,0);
          }
          local_4._0_1_ = 3;
          if (pvVar4 != (void *)0x0) {
            FUN_004f57b0(local_7c,pvVar4);
          }
          puVar3 = FUN_004357b0(local_8c,local_5c);
          local_4._0_1_ = 7;
          FUN_004f26d0(&local_9c,puVar3);
        }
        FUN_005f58b0(local_8c);
        piVar5 = (int *)FUN_004f5cd0(0x270);
        local_98 = piVar5;
        if (piVar5 != (int *)0x0) {
          piVar5[8] = *(int *)(param_1 + 0x30);
          (**(code **)(*piVar5 + 0x24))();
          (**(code **)(*piVar5 + 0x2c))((int)pvVar2 + 0x28);
        }
        local_a0 = 1;
      }
    }
LAB_004c9c10:
    puVar3 = FUN_004357b0((void *)(param_1 + 0x40),local_60);
    local_4._0_1_ = 0x12;
    FUN_004f26d0((void *)(param_1 + 0x54),puVar3);
  } while( true );
}

