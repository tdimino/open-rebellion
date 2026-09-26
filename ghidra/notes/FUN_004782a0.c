
int * __fastcall FUN_004782a0(void *param_1)

{
  bool bVar1;
  bool bVar2;
  undefined4 *puVar3;
  void *pvVar4;
  uint *puVar5;
  void *pvVar6;
  undefined1 *this;
  int iVar7;
  int *piVar8;
  uint local_44;
  uint local_40;
  int local_3c;
  uint local_38;
  int *local_34;
  undefined1 local_30 [4];
  undefined1 local_2c [4];
  undefined1 local_28 [4];
  undefined1 local_24 [4];
  undefined1 *local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063320a;
  pvStack_c = ExceptionList;
  local_40 = 0;
  local_34 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_44);
  local_4 = 0;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 1;
  FUN_005f4950(&local_38,0);
  local_4._0_1_ = 2;
  local_3c = 0;
  bVar2 = false;
  puVar3 = FUN_0041a600(*(void **)((int)param_1 + 0x58),(undefined4 *)((int)param_1 + 0x34),0x8005,2
                        ,2);
  FUN_00435790((void *)((int)param_1 + 0x48),(int)puVar3);
  puVar3 = FUN_004357b0((void *)((int)param_1 + 0x48),local_30);
  local_4._0_1_ = 3;
  FUN_004f26d0(&local_44,puVar3);
  do {
    local_4._0_1_ = 2;
    FUN_00619730();
    do {
      local_14 = 8;
      local_10 = 0x10;
      if (((local_44 >> 0x18 < 8) || (0xf < local_44 >> 0x18)) || (local_3c != 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        FUN_005f58b0((void *)((int)param_1 + 0x48));
        iVar7 = thunk_FUN_005f5060((int)local_1c);
        if (iVar7 != 0) {
          FUN_004ece30(&local_3c);
          local_4._0_1_ = 9;
          puVar3 = FUN_00479220(param_1,local_24,&local_44);
          local_4._0_1_ = 10;
          FUN_004f26d0(&local_3c,puVar3);
          local_4 = CONCAT31(local_4._1_3_,9);
          FUN_00619730();
          piVar8 = (int *)FUN_004f5cd0(0x270);
          local_34 = piVar8;
          if (piVar8 != (int *)0x0) {
            piVar8[8] = *(int *)((int)param_1 + 0x20);
            (**(code **)(*piVar8 + 0x24))();
            (**(code **)(*piVar8 + 0x2c))(&local_40);
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
      iVar7 = *(int *)((int)param_1 + 0x58);
      local_20 = &stack0xffffffa4;
      FUN_004f26d0(&stack0xffffffa4,&local_44);
      pvVar4 = FUN_00419e40((void *)(iVar7 + 0x78));
      if (pvVar4 != (void *)0x0) {
        pvVar4 = (void *)thunk_FUN_005f5060((int)pvVar4 + 0x48);
        while ((pvVar4 != (void *)0x0 && (!bVar2))) {
          puVar5 = FUN_00403040(pvVar4,local_30);
          local_4._0_1_ = 4;
          pvVar6 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x58) + 0x58),puVar5);
          local_4._0_1_ = 2;
          FUN_00619730();
          if ((pvVar6 != (void *)0x0) &&
             ((local_38 = *(uint *)((int)pvVar6 + 0x24), (local_38 & 0x20000) != 0 &&
              ((local_38 & 0xc03800) == 0)))) {
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
              if (0 < *(int *)((int)pvVar6 + 0x60)) {
                bVar1 = true;
              }
            }
            if ((*(int *)((int)param_1 + 0x80) < 0) && (!bVar1)) {
              bVar2 = false;
              if (0 < *(int *)((int)pvVar6 + 0x48)) {
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
                goto LAB_004784d0;
              }
            }
            else {
LAB_004784d0:
              if (!bVar1) goto LAB_00478577;
            }
            local_3c = 1;
            this = (undefined1 *)FUN_00618b70(0x20);
            local_4._0_1_ = 5;
            local_20 = this;
            if (this == (undefined1 *)0x0) {
              pvVar6 = (void *)0x0;
            }
            else {
              puVar3 = FUN_00403040(pvVar4,local_2c);
              local_40 = local_40 | 1;
              local_4 = CONCAT31(local_4._1_3_,6);
              pvVar6 = FUN_004f5b10(this,puVar3,0);
            }
            local_4 = 5;
            if ((local_40 & 1) != 0) {
              local_40 = local_40 & 0xfffffffe;
              FUN_00619730();
            }
            local_4._0_1_ = 2;
            if (pvVar6 != (void *)0x0) {
              FUN_004f57b0(local_1c,pvVar6);
              puVar5 = FUN_00403040(pvVar4,local_28);
              local_4._0_1_ = 7;
              FUN_00478e00(param_1,puVar5);
              local_4._0_1_ = 2;
              FUN_00619730();
            }
          }
LAB_00478577:
          pvVar4 = *(void **)((int)pvVar4 + 0x10);
        }
      }
    } while (local_3c != 0);
    puVar3 = FUN_004357b0((void *)((int)param_1 + 0x48),local_24);
    local_4._0_1_ = 8;
    FUN_004f26d0(&local_44,puVar3);
  } while( true );
}

