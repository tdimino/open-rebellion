
int * __fastcall FUN_00475890(void *param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  undefined4 *puVar6;
  uint *puVar7;
  void *pvVar8;
  undefined1 *this;
  int *piVar9;
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
  puStack_8 = &LAB_00632d1a;
  pvStack_c = ExceptionList;
  iVar10 = 0;
  bVar3 = false;
  local_44 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_48);
  local_4 = 0;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 1;
  FUN_005f4950(&local_38,0);
  local_4._0_1_ = 2;
  uVar4 = *(uint *)((int)param_1 + 0x30) >> 0x18;
  local_3c = 0;
  local_40 = 0;
  local_24 = (undefined1 *)0x90;
  local_20 = 0x98;
  if ((uVar4 < 0x90) || (0x97 < uVar4)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    pvVar5 = FUN_0041ab40(*(void **)((int)param_1 + 0x4c),0,0x8001,0xf000000e,(void *)0x1);
    FUN_00435790((void *)((int)param_1 + 0x3c),(int)pvVar5);
    puVar6 = (undefined4 *)FUN_004357b0((void *)((int)param_1 + 0x3c),local_34);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_48,puVar6);
    do {
      local_4._0_1_ = 2;
      FUN_00619730();
      do {
        local_14 = 8;
        local_10 = 0x10;
        if (((local_48 >> 0x18 < 8) || (0xf < local_48 >> 0x18)) || (iVar10 != 0)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          FUN_005f58b0((void *)((int)param_1 + 0x3c));
          goto LAB_00475b54;
        }
        iVar1 = *(int *)((int)param_1 + 0x4c);
        local_24 = &stack0xffffffa0;
        FUN_004f26d0(&stack0xffffffa0,&local_48);
        pvVar5 = FUN_00419e40((void *)(iVar1 + 0x78));
        if (pvVar5 != (void *)0x0) {
          pvVar5 = (void *)thunk_FUN_005f5060((int)pvVar5 + 0x48);
          iVar10 = local_3c;
          while ((local_3c = iVar10, pvVar5 != (void *)0x0 && (local_40 == 0))) {
            puVar7 = (uint *)FUN_00403040(pvVar5,local_34);
            local_4._0_1_ = 4;
            pvVar8 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x4c) + 0x58),puVar7);
            local_4._0_1_ = 2;
            FUN_00619730();
            if ((pvVar8 != (void *)0x0) &&
               ((local_38 = *(uint *)((int)pvVar8 + 0x24), (local_38 & 0x20000) != 0 &&
                ((local_38 & 0x803800) == 0)))) {
              bVar2 = false;
              local_40 = 1;
              if ((*(int *)((int)param_1 + 0x34) < 0) && (local_40 = 0, (local_38 & 1) != 0)) {
                bVar2 = true;
              }
              if (bVar2) {
                local_3c = 1;
                this = (undefined1 *)FUN_00618b70(0x20);
                local_4._0_1_ = 5;
                local_24 = this;
                if (this == (undefined1 *)0x0) {
                  pvVar8 = (void *)0x0;
                }
                else {
                  puVar6 = (undefined4 *)FUN_00403040(pvVar5,local_30);
                  bVar3 = true;
                  local_4 = CONCAT31(local_4._1_3_,6);
                  pvVar8 = FUN_004f5b10(this,puVar6,0);
                }
                local_4 = 5;
                if (bVar3) {
                  bVar3 = false;
                  FUN_00619730();
                }
                local_4._0_1_ = 2;
                if (pvVar8 != (void *)0x0) {
                  FUN_004f57b0(local_1c,pvVar8);
                  puVar7 = (uint *)FUN_00403040(pvVar5,local_2c);
                  local_4._0_1_ = 7;
                  FUN_00475e70(param_1,puVar7);
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
      puVar6 = (undefined4 *)FUN_004357b0((void *)((int)param_1 + 0x3c),local_28);
      local_4._0_1_ = 8;
      FUN_004f26d0(&local_48,puVar6);
    } while( true );
  }
LAB_00475b54:
  iVar10 = thunk_FUN_005f5060((int)local_1c);
  if (iVar10 != 0) {
    uVar4 = *(uint *)((int)param_1 + 0x2c) >> 0x18;
    local_14 = 8;
    local_10 = 0x10;
    if ((uVar4 < 8) || (0xf < uVar4)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      piVar9 = (int *)FUN_004f5cd0(0x201);
      local_44 = piVar9;
      if (piVar9 == (int *)0x0) goto LAB_00475c2a;
      piVar9[8] = *(int *)((int)param_1 + 0x20);
    }
    else {
      local_14 = 0x90;
      uVar4 = *(uint *)((int)param_1 + 0x30) >> 0x18;
      local_10 = 0x98;
      if ((uVar4 < 0x90) || (0x97 < uVar4)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if ((!bVar3) || (piVar9 = (int *)FUN_004f5cd0(0x270), local_44 = piVar9, piVar9 == (int *)0x0)
         ) goto LAB_00475c2a;
      piVar9[8] = *(int *)((int)param_1 + 0x20);
    }
    local_44 = piVar9;
    (**(code **)(*piVar9 + 0x24))();
    (**(code **)(*piVar9 + 0x2c))((int)param_1 + 0x2c);
  }
LAB_00475c2a:
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_44;
}

