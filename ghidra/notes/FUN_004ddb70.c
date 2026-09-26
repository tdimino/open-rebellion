
int * __fastcall FUN_004ddb70(int param_1)

{
  uint *this;
  bool bVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  void *pvVar7;
  int *piVar8;
  uint *puVar9;
  undefined1 *puVar10;
  int local_5c;
  int *local_50;
  int iStack_4c;
  int local_48;
  undefined1 auStack_44 [4];
  uint uStack_40;
  undefined1 *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d682;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_50 = (int *)0x0;
  local_5c = 0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  puVar9 = (uint *)(param_1 + 0x50);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  FUN_004ece80(puVar9);
  this = (uint *)(param_1 + 0x54);
  FUN_004ece80(this);
  uVar3 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x2c) + 4));
  if ((uVar3 & 0x20) != 0) {
    pvVar4 = FUN_00419e90(*(void **)(param_1 + 0x2c),0,0,0x2000000,0,0,0,&DAT_00000002);
    pvVar7 = (void *)(param_1 + 0x40);
    FUN_00435790(pvVar7,(int)pvVar4);
    puVar5 = FUN_004357b0(pvVar7,&local_48);
    local_4._0_1_ = 1;
    FUN_004f26d0(this,puVar5);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar7);
    pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,0x20001,2,0,0,0x10,&DAT_00000002);
    FUN_00435790(pvVar7,(int)pvVar4);
    puVar5 = FUN_004357b0(pvVar7,&local_48);
    local_4._0_1_ = 2;
    FUN_004f26d0(this,puVar5);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar7);
    local_3c = (undefined1 *)0x90;
    local_38 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      iVar6 = FUN_0042f310((void *)(*(int *)(param_1 + 0x2c) + 0xd8),(int *)this);
      if (iVar6 != 0) {
        local_5c = FUN_00476b70(iVar6);
      }
      if (0 < local_5c) {
        puVar5 = FUN_00419b80(*(void **)(param_1 + 0x2c),this,0x80000,0,0x1000,0,0,0,1);
        FUN_00435790(pvVar7,(int)puVar5);
        puVar5 = FUN_004357b0(pvVar7,&local_48);
        local_4._0_1_ = 3;
        FUN_004f26d0(puVar9,puVar5);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_005f58b0(pvVar7);
        pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar9,0x10000,0,0x8001,2,0,0x30000,0x13,
                              &DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar4);
        puVar5 = FUN_004357b0(pvVar7,&local_48);
        local_4._0_1_ = 4;
        FUN_004f26d0(puVar9,puVar5);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_005f58b0(pvVar7);
        local_3c = (undefined1 *)0x90;
        local_38 = 0x98;
        if ((*puVar9 >> 0x18 < 0x90) || (0x97 < *puVar9 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) {
          puVar5 = FUN_00419b80(*(void **)(param_1 + 0x2c),this,0x280000,0,0,0,0,0,1);
          FUN_00435790(pvVar7,(int)puVar5);
          puVar5 = FUN_004357b0(pvVar7,&local_48);
          local_4._0_1_ = 5;
          FUN_004f26d0(puVar9,puVar5);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar7);
          pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar9,0x18000,0,1,2,0,0x30000,0x13,
                                &DAT_00000002);
          FUN_00435790(pvVar7,(int)pvVar4);
          puVar5 = FUN_004357b0(pvVar7,&local_48);
          local_4._0_1_ = 6;
          FUN_004f26d0(puVar9,puVar5);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar7);
        }
      }
      local_3c = (undefined1 *)0x90;
      local_38 = 0x98;
      if ((*puVar9 >> 0x18 < 0x90) || (0x97 < *puVar9 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        iVar6 = *(int *)(param_1 + 0x2c);
        local_3c = &stack0xffffff90;
        FUN_004f26d0(&stack0xffffff90,puVar9);
        pvVar7 = FUN_00403d30((void *)(iVar6 + 0x2c));
        piVar8 = FUN_004f3220(*(int *)(param_1 + 0x30),puVar9);
        if ((piVar8 != (int *)0x0) && (pvVar7 != (void *)0x0)) {
          local_48 = 0;
          if (*(int *)((int)pvVar7 + 0x94) < local_5c) {
            local_5c = *(int *)((int)pvVar7 + 0x94);
          }
          FUN_00504c40(auStack_2c,piVar8,1);
          local_4._0_1_ = 7;
          do {
            if (local_5c < 1) break;
            FUN_00513120((int)auStack_2c);
            pvVar7 = (void *)0x0;
            iStack_4c = 1000000;
            FUN_00513120((int)auStack_2c);
            while (iStack_10 != 0) {
              puVar10 = auStack_44;
              pvVar4 = (void *)FUN_0052bed0((int)auStack_2c);
              puVar9 = FUN_00402d80(pvVar4,puVar10);
              local_4._0_1_ = 8;
              iVar6 = FUN_0042e190((void *)(*(int *)(param_1 + 0x2c) + 0x138),puVar9);
              local_4._0_1_ = 7;
              FUN_00619730();
              if ((*(uint *)(iVar6 + 0x18) & 0x2000) != 0) {
                piVar8 = (int *)FUN_0052bed0((int)auStack_2c);
                iVar6 = (**(code **)(*piVar8 + 0x1e0))();
                if (iVar6 < iStack_4c) {
                  puVar9 = &uStack_40;
                  pvVar4 = (void *)FUN_0052bed0((int)auStack_2c);
                  puVar9 = FUN_004025b0(pvVar4,puVar9);
                  local_4._0_1_ = 9;
                  pvVar4 = FUN_004f5940(local_34,puVar9);
                  local_4._0_1_ = 7;
                  FUN_00619730();
                  if (pvVar4 == (void *)0x0) {
                    piVar8 = (int *)FUN_0052bed0((int)auStack_2c);
                    iStack_4c = (**(code **)(*piVar8 + 0x1e0))();
                    pvVar7 = (void *)FUN_0052bed0((int)auStack_2c);
                  }
                }
              }
              FUN_005130d0((int)auStack_2c);
            }
            if (pvVar7 == (void *)0x0) {
              local_48 = 1;
            }
            else {
              pvVar4 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 10;
              if (pvVar4 == (void *)0x0) {
                pvVar7 = (void *)0x0;
              }
              else {
                puVar9 = FUN_004025b0(pvVar7,(uint *)&local_3c);
                bVar2 = true;
                local_4 = CONCAT31(local_4._1_3_,0xb);
                pvVar7 = FUN_004f5b10(pvVar4,puVar9,0);
              }
              local_4 = 10;
              if (bVar2) {
                bVar2 = false;
                FUN_00619730();
              }
              local_4._0_1_ = 7;
              if (pvVar7 != (void *)0x0) {
                FUN_004f57b0(local_34,pvVar7);
              }
              local_5c = local_5c + -1;
            }
          } while (local_48 == 0);
          local_4._0_1_ = 0;
          FUN_00504d40(auStack_2c);
        }
      }
    }
  }
  iVar6 = thunk_FUN_005f5060((int)local_34);
  if ((iVar6 != 0) && (local_50 = (int *)FUN_004f5cd0(0x201), local_50 != (int *)0x0)) {
    local_50[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*local_50 + 0x24))();
    (**(code **)(*local_50 + 0x2c))(this);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return local_50;
}

