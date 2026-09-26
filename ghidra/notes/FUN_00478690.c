
int * __fastcall FUN_00478690(void *param_1)

{
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  uint *puVar5;
  int iVar6;
  int *piVar7;
  uint local_48;
  uint local_44;
  undefined1 *local_40;
  int local_3c;
  uint local_38;
  int *local_34;
  undefined1 local_30 [4];
  undefined1 local_2c [4];
  undefined1 local_28 [4];
  undefined1 local_24 [4];
  void *local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063328a;
  pvStack_c = ExceptionList;
  local_44 = 0;
  local_34 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_48);
  local_4 = 0;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 1;
  FUN_005f4950(&local_38,0);
  local_4._0_1_ = 2;
  local_3c = 0;
  bVar2 = false;
  pvVar3 = FUN_0041ab40(*(void **)((int)param_1 + 0x58),*(uint *)((int)param_1 + 0x44),0x8001,
                        0xf0000006,(void *)0x1);
  FUN_00435790((void *)((int)param_1 + 0x48),(int)pvVar3);
  puVar4 = FUN_004357b0((void *)((int)param_1 + 0x48),&local_40);
  local_4._0_1_ = 3;
  FUN_004f26d0(&local_48,puVar4);
  do {
    local_4._0_1_ = 2;
    FUN_00619730();
    do {
      local_14 = 8;
      local_10 = 0x10;
      if (((local_48 >> 0x18 < 8) || (0xf < local_48 >> 0x18)) || (local_3c != 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        FUN_005f58b0((void *)((int)param_1 + 0x48));
        iVar6 = thunk_FUN_005f5060((int)local_1c);
        if (iVar6 != 0) {
          FUN_004ece30(&local_3c);
          local_4._0_1_ = 9;
          puVar4 = FUN_00479220(param_1,local_24,&local_48);
          local_4._0_1_ = 10;
          FUN_004f26d0(&local_3c,puVar4);
          local_4 = CONCAT31(local_4._1_3_,9);
          FUN_00619730();
          piVar7 = (int *)FUN_004f5cd0(0x270);
          local_34 = piVar7;
          if (piVar7 != (int *)0x0) {
            piVar7[8] = *(int *)((int)param_1 + 0x20);
            (**(code **)(*piVar7 + 0x24))();
            (**(code **)(*piVar7 + 0x2c))(&local_40);
          }
          local_4._0_1_ = 2;
          FUN_00619730();
        }
        local_4._0_1_ = 1;
        FUN_00619730();
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_004f4380(local_1c);
        local_4 = 0xffffffff;
        FUN_00619730();
        ExceptionList = pvStack_c;
        return local_34;
      }
      iVar6 = *(int *)((int)param_1 + 0x58);
      local_40 = &stack0xffffffa0;
      FUN_004f26d0(&stack0xffffffa0,&local_48);
      pvVar3 = FUN_00419e40((void *)(iVar6 + 0x78));
      if (pvVar3 != (void *)0x0) {
        local_40 = (undefined1 *)thunk_FUN_005f5060((int)pvVar3 + 0x48);
        while ((local_40 != (undefined1 *)0x0 && (!bVar2))) {
          puVar5 = FUN_00403040(local_40,local_30);
          local_4._0_1_ = 4;
          pvVar3 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x58) + 0x58),puVar5);
          local_4._0_1_ = 2;
          FUN_00619730();
          if ((pvVar3 != (void *)0x0) &&
             (local_38 = *(uint *)((int)pvVar3 + 0x24), (local_38 & 0xc23800) == 0)) {
            bVar1 = false;
            bVar2 = true;
            if (*(int *)((int)param_1 + 0x6c) < 0) {
              bVar2 = false;
              if ((local_38 & 1) != 0) {
                bVar1 = true;
              }
            }
            if ((*(int *)((int)param_1 + 0x70) < 0) && (!bVar1)) {
              bVar2 = false;
              if ((local_38 & 0x102) != 0) {
                bVar1 = true;
              }
            }
            if ((*(int *)((int)param_1 + 0x90) < 0) && (!bVar1)) {
              bVar2 = false;
              if ((local_38 & 2) != 0) {
                bVar1 = true;
              }
            }
            if ((*(int *)((int)param_1 + 0x74) < 0) && (!bVar1)) {
              bVar2 = false;
              if ((local_38 & 0xc) != 0) {
                bVar1 = true;
              }
            }
            if ((*(int *)((int)param_1 + 0x78) < 0) && (!bVar1)) {
              bVar2 = false;
              if (0 < *(int *)((int)pvVar3 + 0x60)) {
                bVar1 = true;
              }
            }
            if ((*(int *)((int)param_1 + 0x80) < 0) && (!bVar1)) {
              bVar2 = false;
              if (0 < *(int *)((int)pvVar3 + 0x48)) {
                bVar1 = true;
              }
            }
            if ((*(int *)((int)param_1 + 0x7c) < 0) && (!bVar1)) {
              bVar2 = false;
              if ((local_38 & 0x80) != 0) {
                bVar1 = true;
              }
            }
            if (*(int *)((int)param_1 + 0x84) < 0) {
              if (!bVar1) {
                bVar2 = false;
                if ((local_38 & 0x400) != 0) {
                  bVar1 = true;
                }
                goto LAB_004788c1;
              }
            }
            else {
LAB_004788c1:
              if (!bVar1) goto LAB_00478980;
            }
            if ((local_38 & 0x1000000) == 0) {
              local_3c = 1;
              pvVar3 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 5;
              local_20 = pvVar3;
              if (pvVar3 == (void *)0x0) {
                pvVar3 = (void *)0x0;
              }
              else {
                puVar4 = FUN_00403040(local_40,local_2c);
                local_44 = local_44 | 1;
                local_4 = CONCAT31(local_4._1_3_,6);
                pvVar3 = FUN_004f5b10(pvVar3,puVar4,0);
              }
              local_4 = 5;
              if ((local_44 & 1) != 0) {
                local_44 = local_44 & 0xfffffffe;
                FUN_00619730();
              }
              local_4._0_1_ = 2;
              if (pvVar3 != (void *)0x0) {
                FUN_004f57b0(local_1c,pvVar3);
                puVar5 = FUN_00403040(local_40,local_28);
                local_4._0_1_ = 7;
                FUN_00478e00(param_1,puVar5);
                local_4._0_1_ = 2;
                FUN_00619730();
              }
            }
            else {
              *(uint *)((int)pvVar3 + 0x24) = *(uint *)((int)pvVar3 + 0x24) | 0x2000000;
            }
          }
LAB_00478980:
          local_40 = *(undefined1 **)(local_40 + 0x10);
        }
      }
    } while (local_3c != 0);
    puVar4 = FUN_004357b0((void *)((int)param_1 + 0x48),local_24);
    local_4._0_1_ = 8;
    FUN_004f26d0(&local_48,puVar4);
  } while( true );
}

