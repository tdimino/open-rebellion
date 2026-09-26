
int * __fastcall FUN_004c9cb0(int param_1)

{
  uint *this;
  bool bVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  int iVar7;
  int iVar8;
  int *piVar9;
  undefined1 *this_00;
  uint *puVar10;
  uint local_90;
  undefined1 local_8c [4];
  int local_88;
  uint local_84;
  uint local_80;
  void *local_7c;
  uint local_78;
  int *local_74;
  uint local_70;
  undefined1 *local_6c;
  undefined4 local_68;
  undefined1 *puStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 local_58 [2];
  uint auStack_50 [7];
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b1ac;
  pvStack_c = ExceptionList;
  local_78 = 0;
  local_74 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_88 = param_1;
  FUN_004355d0(auStack_50 + 3);
  local_4 = 0;
  FUN_004ece30(&local_90);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_84);
  local_4._0_1_ = 2;
  FUN_004ece30(&local_80);
  local_4._0_1_ = 3;
  FUN_004ece30(&local_70);
  local_4._0_1_ = 4;
  FUN_004f4340(local_58);
  local_4._0_1_ = 5;
  uVar3 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x2c) + 4));
  if ((uVar3 & 0x30) != 0) {
    pvVar4 = FUN_00419e90(*(void **)(param_1 + 0x2c),0,0,0x8000000,0,0,0,&DAT_00000002);
    pvVar6 = (void *)(param_1 + 0x40);
    local_7c = pvVar6;
    FUN_00435790(pvVar6,(int)pvVar4);
    puVar5 = FUN_004357b0(pvVar6,local_8c);
    local_4._0_1_ = 6;
    FUN_004f26d0(&local_84,puVar5);
    local_4._0_1_ = 5;
    FUN_00619730();
    FUN_004f26d0(&local_70,&local_84);
    bVar1 = false;
    while( true ) {
      local_6c = (undefined1 *)0x80;
      local_68 = 0x90;
      if (((local_84 >> 0x18 < 0x80) || (0x8f < local_84 >> 0x18)) || (bVar1)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (!bVar2) break;
      pvVar6 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),&local_84);
      if (((pvVar6 != (void *)0x0) &&
          (iVar7 = FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),
                                *(uint *)((int)pvVar6 + 0x38)), param_1 = local_88, iVar7 != 0)) &&
         (iVar8 = FUN_00476b70(iVar7), param_1 = local_88, 0 < iVar8)) {
        FUN_004f26d0(&local_90,(undefined4 *)(iVar7 + 0x40));
        uStack_60 = 0x90;
        uStack_5c = 0x98;
        if ((local_90 >> 0x18 < 0x90) || (0x97 < local_90 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        param_1 = local_88;
        if (bVar2) {
          puStack_64 = &stack0xffffff5c;
          iVar7 = *(int *)(local_88 + 0x2c);
          FUN_004f26d0(&stack0xffffff5c,&local_90);
          pvVar6 = FUN_00403d30((void *)(iVar7 + 0x2c));
          param_1 = local_88;
          if ((pvVar6 != (void *)0x0) && ((*(byte *)((int)pvVar6 + 0x28) & 2) == 0)) {
            bVar1 = true;
          }
        }
      }
      puVar5 = FUN_004357b0(local_7c,local_8c);
      local_4._0_1_ = 7;
      FUN_004f26d0(&local_84,puVar5);
      local_4._0_1_ = 5;
      FUN_00619730();
    }
    FUN_005f58b0(local_7c);
    if (!bVar1) {
      FUN_004ece80(&local_90);
      pvVar6 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),&local_70);
      if (((pvVar6 != (void *)0x0) &&
          (iVar7 = FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),
                                *(uint *)((int)pvVar6 + 0x38)), iVar7 != 0)) &&
         (iVar8 = FUN_00476b70(iVar7), 0 < iVar8)) {
        FUN_004f26d0(&local_90,(undefined4 *)(iVar7 + 0x40));
      }
    }
    local_6c = (undefined1 *)0x90;
    local_68 = 0x98;
    if ((local_90 >> 0x18 < 0x90) || (0x97 < local_90 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    iVar7 = local_88;
    if (bVar1) {
      pvVar4 = FUN_0041a4c0(*(void **)(local_88 + 0x2c),5,2,0xc,(void *)0x1);
      pvVar6 = local_7c;
      FUN_00435790(local_7c,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar6,local_8c);
      this = (uint *)(iVar7 + 0x54);
      local_4._0_1_ = 8;
      FUN_004f26d0(this,puVar5);
      local_4._0_1_ = 5;
      FUN_00619730();
      local_88 = 0;
      while( true ) {
        uStack_34 = 8;
        uStack_30 = 0x10;
        if (((*this >> 0x18 < 8) || (0xf < *this >> 0x18)) || (local_88 != 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) break;
        iVar8 = *(int *)(iVar7 + 0x2c);
        local_6c = &stack0xffffff5c;
        FUN_004f26d0(&stack0xffffff5c,this);
        pvVar6 = FUN_00419e40((void *)(iVar8 + 0x78));
        if (pvVar6 != (void *)0x0) {
          pvVar6 = FUN_0041ac80(*(void **)(iVar7 + 0x2c),this,0x24000,0x800,9,(void *)0x1);
          FUN_00435790(auStack_50 + 3,(int)pvVar6);
          puVar5 = FUN_004357b0(auStack_50 + 3,local_8c);
          local_4._0_1_ = 9;
          FUN_004f26d0(&local_80,puVar5);
          while( true ) {
            local_4._0_1_ = 5;
            FUN_00619730();
            auStack_50[1] = 0x14;
            auStack_50[2] = 0x1c;
            if ((local_80 >> 0x18 < 0x14) || (0x1b < local_80 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (!bVar1) break;
            piVar9 = FUN_004f2e20(*(int *)(iVar7 + 0x30),&local_80);
            if (piVar9 != (int *)0x0) {
              FUN_00504c40(auStack_2c,piVar9,1);
              local_4 = CONCAT31(local_4._1_3_,10);
              FUN_00513120((int)auStack_2c);
              if (iStack_10 != 0) {
                local_88 = 1;
                do {
                  this_00 = (undefined1 *)FUN_00618b70(0x20);
                  local_4._0_1_ = 0xb;
                  local_6c = this_00;
                  if (this_00 == (undefined1 *)0x0) {
                    pvVar6 = (void *)0x0;
                  }
                  else {
                    puVar10 = auStack_50;
                    pvVar6 = (void *)FUN_0052bed0((int)auStack_2c);
                    puVar10 = FUN_004025b0(pvVar6,puVar10);
                    local_78 = local_78 | 1;
                    local_4 = CONCAT31(local_4._1_3_,0xc);
                    pvVar6 = FUN_004f5b10(this_00,puVar10,0);
                  }
                  local_4 = 0xb;
                  if ((local_78 & 1) != 0) {
                    local_78 = local_78 & 0xfffffffe;
                    FUN_00619730();
                  }
                  local_4 = CONCAT31(local_4._1_3_,10);
                  if (pvVar6 != (void *)0x0) {
                    FUN_004f57b0(local_58,pvVar6);
                  }
                  FUN_005130d0((int)auStack_2c);
                } while (iStack_10 != 0);
              }
              local_4._0_1_ = 5;
              FUN_00504d40(auStack_2c);
            }
            puVar5 = FUN_004357b0(auStack_50 + 3,&uStack_60);
            local_4._0_1_ = 0xd;
            FUN_004f26d0(&local_80,puVar5);
          }
          FUN_005f58b0(auStack_50 + 3);
        }
        puVar5 = FUN_004357b0(local_7c,&puStack_64);
        local_4._0_1_ = 0xe;
        FUN_004f26d0(this,puVar5);
        local_4._0_1_ = 5;
        FUN_00619730();
      }
      FUN_005f58b0(local_7c);
      iVar8 = thunk_FUN_005f5060((int)local_58);
      if ((iVar8 != 0) &&
         (piVar9 = (int *)FUN_004f5cd0(0x201), local_74 = piVar9, piVar9 != (int *)0x0)) {
        piVar9[8] = *(int *)(iVar7 + 0x30);
        (**(code **)(*piVar9 + 0x24))();
        (**(code **)(*piVar9 + 0x2c))(&stack0xffffff6c);
      }
    }
  }
  local_4._0_1_ = 4;
  FUN_004f4380(local_58);
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00435610(auStack_50 + 3);
  ExceptionList = pvStack_c;
  return local_74;
}

