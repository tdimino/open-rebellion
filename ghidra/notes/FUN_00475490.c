
int * __fastcall FUN_00475490(void *param_1)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  undefined4 *puVar4;
  void *pvVar5;
  uint *puVar6;
  void *pvVar7;
  undefined1 *this;
  int *piVar8;
  int iVar9;
  int iVar10;
  uint local_48;
  int *local_44;
  int local_40;
  int local_3c;
  uint local_38;
  undefined1 local_34 [4];
  undefined1 local_30 [4];
  undefined1 local_2c [4];
  undefined1 local_28 [4];
  undefined1 *local_24;
  undefined4 local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632caa;
  pvStack_c = ExceptionList;
  bVar1 = false;
  bVar2 = false;
  local_44 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_48);
  local_4 = 0;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 1;
  FUN_005f4950(&local_38,0);
  uVar3 = *(uint *)((int)param_1 + 0x30) >> 0x18;
  iVar10 = 0;
  local_4._0_1_ = 2;
  local_3c = 0;
  local_40 = 0;
  local_24 = (undefined1 *)0x90;
  local_20 = 0x98;
  if ((0x8f < uVar3) && (uVar3 < 0x98)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    puVar4 = FUN_0041a8a0(*(void **)((int)param_1 + 0x4c),(undefined4 *)((int)param_1 + 0x30),0x8005
                          ,10,2);
    FUN_00435790((void *)((int)param_1 + 0x3c),(int)puVar4);
    puVar4 = FUN_004357b0((void *)((int)param_1 + 0x3c),local_34);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_48,puVar4);
    do {
      local_4._0_1_ = 2;
      FUN_00619730();
      do {
        local_14 = 8;
        local_10 = 0x10;
        if (((local_48 >> 0x18 < 8) || (0xf < local_48 >> 0x18)) || (iVar10 != 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) {
          FUN_005f58b0((void *)((int)param_1 + 0x3c));
          goto LAB_00475752;
        }
        iVar9 = *(int *)((int)param_1 + 0x4c);
        local_24 = &stack0xffffffa0;
        FUN_004f26d0(&stack0xffffffa0,&local_48);
        pvVar5 = FUN_00419e40((void *)(iVar9 + 0x78));
        if (pvVar5 != (void *)0x0) {
          pvVar5 = (void *)thunk_FUN_005f5060((int)pvVar5 + 0x48);
          iVar10 = local_3c;
          while ((local_3c = iVar10, pvVar5 != (void *)0x0 && (local_40 == 0))) {
            puVar6 = FUN_00403040(pvVar5,local_34);
            local_4._0_1_ = 4;
            pvVar7 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x4c) + 0x58),puVar6);
            local_4._0_1_ = 2;
            FUN_00619730();
            if ((pvVar7 != (void *)0x0) &&
               ((local_38 = *(uint *)((int)pvVar7 + 0x24), (local_38 & 0x20000) != 0 &&
                ((local_38 & 0x803800) == 0)))) {
              bVar1 = false;
              local_40 = 1;
              if ((*(int *)((int)param_1 + 0x34) < 0) && (local_40 = 0, (local_38 & 1) != 0)) {
                bVar1 = true;
              }
              if (bVar1) {
                local_3c = 1;
                this = (undefined1 *)FUN_00618b70(0x20);
                local_4._0_1_ = 5;
                local_24 = this;
                if (this == (undefined1 *)0x0) {
                  pvVar7 = (void *)0x0;
                }
                else {
                  puVar4 = FUN_00403040(pvVar5,local_30);
                  bVar2 = true;
                  local_4 = CONCAT31(local_4._1_3_,6);
                  pvVar7 = FUN_004f5b10(this,puVar4,0);
                }
                local_4 = 5;
                if (bVar2) {
                  bVar2 = false;
                  FUN_00619730();
                }
                local_4._0_1_ = 2;
                if (pvVar7 != (void *)0x0) {
                  FUN_004f57b0(local_1c,pvVar7);
                  puVar6 = FUN_00403040(pvVar5,local_2c);
                  local_4._0_1_ = 7;
                  FUN_00475e70(param_1,puVar6);
                  local_4._0_1_ = 2;
                  FUN_00619730();
                }
              }
            }
            pvVar5 = *(void **)((int)pvVar5 + 0x10);
            iVar10 = local_3c;
          }
        }
      } while (iVar10 != 0);
      puVar4 = FUN_004357b0((void *)((int)param_1 + 0x3c),local_28);
      local_4._0_1_ = 8;
      FUN_004f26d0(&local_48,puVar4);
    } while( true );
  }
LAB_00475752:
  iVar10 = thunk_FUN_005f5060((int)local_1c);
  if (iVar10 != 0) {
    iVar10 = (int)param_1 + 0x2c;
    uVar3 = *(uint *)((int)param_1 + 0x2c) >> 0x18;
    local_14 = 8;
    local_10 = 0x10;
    if ((uVar3 < 8) || (0xf < uVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      piVar8 = (int *)FUN_004f5cd0(0x201);
      local_44 = piVar8;
      if (piVar8 == (int *)0x0) goto LAB_00475840;
      piVar8[8] = *(int *)((int)param_1 + 0x20);
      (**(code **)(*piVar8 + 0x24))();
      iVar9 = *piVar8;
    }
    else {
      iVar10 = (int)param_1 + 0x30;
      uVar3 = *(uint *)((int)param_1 + 0x30) >> 0x18;
      local_14 = 0x90;
      local_10 = 0x98;
      if ((uVar3 < 0x90) || (0x97 < uVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if ((!bVar2) || (piVar8 = (int *)FUN_004f5cd0(0x270), local_44 = piVar8, piVar8 == (int *)0x0)
         ) goto LAB_00475840;
      piVar8[8] = *(int *)((int)param_1 + 0x20);
      (**(code **)(*piVar8 + 0x24))();
      iVar9 = *piVar8;
    }
    (**(code **)(iVar9 + 0x2c))(iVar10);
  }
LAB_00475840:
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_44;
}

