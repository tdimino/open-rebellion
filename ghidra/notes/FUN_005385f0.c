
uint __thiscall FUN_005385f0(void *this,void *param_1)

{
  undefined1 uVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  uint *puVar5;
  int iVar6;
  undefined3 extraout_var;
  void *pvVar7;
  int iVar8;
  uint *puVar9;
  undefined4 *puVar10;
  undefined3 extraout_var_00;
  uint local_98;
  void *local_94;
  uint local_90;
  void *local_8c;
  undefined4 local_88;
  undefined4 local_84;
  int local_80;
  uint local_7c;
  void *local_78;
  uint local_74;
  undefined4 local_70;
  undefined4 local_6c;
  undefined4 local_68;
  int local_64;
  undefined4 local_60;
  int local_5c [4];
  undefined4 local_4c [9];
  undefined4 local_28 [7];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645f4e;
  pvStack_c = ExceptionList;
  local_88 = 1;
  local_84 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  local_78 = this;
  FUN_00520580(param_1,&local_88);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00541e70(local_5c);
  local_4 = 1;
  FUN_0051fcb0(this,local_5c);
  uVar3 = FUN_005202d0(this,param_1);
  local_7c = 0;
  if (uVar3 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      uVar3 = FUN_00553b80(*(int *)((int)this + 0x40),&local_7c);
    }
    if ((uVar3 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
      local_8c = (void *)FUN_0041c210(*(int *)((int)this + 0x40));
      while (((local_8c != (void *)0x0 && (uVar3 != 0)) && (*(int *)((int)param_1 + 4) == -1))) {
        FUN_004fa150(local_8c,&local_98);
        local_4._0_1_ = 2;
        pvVar4 = (void *)FUN_00505750(&local_98);
        if (pvVar4 != (void *)0x0) {
          local_94 = (void *)0x0;
          local_88 = 1;
          local_84 = 0xffffffff;
          local_4._0_1_ = 3;
          if (*(int *)((int)param_1 + 4) == -1) {
            local_74 = 1;
            local_70 = 0x22;
            local_4._0_1_ = 4;
            puVar5 = (uint *)FUN_004c27b0(pvVar4,&local_90);
            local_4._0_1_ = 5;
            uVar3 = FUN_00504e60(puVar5,&local_94,&local_74,&local_88);
            local_4._0_1_ = 4;
            FUN_00619730();
            local_4._0_1_ = 3;
            FUN_00619730();
            if (uVar3 != 0) {
              if (local_94 == (void *)0x0) {
                FUN_00520580(param_1,&local_88);
              }
              goto LAB_005387ae;
            }
          }
          else {
LAB_005387ae:
            if (((uVar3 != 0) && (*(int *)((int)param_1 + 4) == -1)) &&
               ((*(uint *)((int)local_94 + 0x24) >> 6 & 3) != local_7c)) {
              puVar5 = (uint *)FUN_00451110(this,&local_80);
              local_64 = 0x14;
              local_60 = 0x1c;
              local_4._0_1_ = 6;
              if ((*puVar5 >> 0x18 < 0x14) || (0x1b < *puVar5 >> 0x18)) {
                bVar2 = false;
              }
              else {
                bVar2 = true;
              }
              FUN_00619730();
              local_4._0_1_ = 3;
              FUN_00619730();
              if (!bVar2) {
                local_6c = 1;
                local_68 = 0x28;
                local_4._0_1_ = 7;
                FUN_00520580(param_1,&local_6c);
                local_4._0_1_ = 3;
                FUN_00619730();
              }
            }
          }
          local_4._0_1_ = 2;
          FUN_00619730();
        }
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        local_8c = *(void **)((int)local_8c + 0x10);
      }
    }
  }
  FUN_00551060(local_4c);
  local_4 = CONCAT31(local_4._1_3_,8);
  if (uVar3 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      local_90 = 3;
      uVar3 = FUN_00551190(local_4c,local_5c,local_7c,&local_90);
    }
    if ((uVar3 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
      if (local_7c == 1) {
        iVar6 = 1;
      }
      else if (local_7c == 2) {
        iVar6 = 2;
      }
      else {
        iVar6 = 0;
      }
      pvVar4 = (void *)FUN_00506ea0(iVar6);
      uVar3 = (uint)(pvVar4 != (void *)0x0);
      if (uVar3 != 0) {
        bVar2 = FUN_0052e970(pvVar4,(uint *)((int)this + 0x44),*(int *)((int)this + 0x48),param_1);
        uVar3 = CONCAT31(extraout_var,bVar2);
      }
    }
  }
  iVar6 = 0;
  if ((uVar3 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
    pvVar4 = (void *)FUN_0041c210(*(int *)((int)this + 0x40));
    this = local_78;
    while ((local_78 = this, pvVar4 != (void *)0x0 && (uVar3 != 0))) {
      FUN_004fa150(pvVar4,&local_98);
      local_4._0_1_ = 9;
      pvVar7 = (void *)FUN_00505750(&local_98);
      iVar8 = FUN_00528040(pvVar7,1);
      iVar6 = iVar6 + iVar8;
      puVar5 = (uint *)FUN_004c27b0(pvVar7,&local_90);
      local_4._0_1_ = 10;
      uVar3 = FUN_005513a0(local_4c,puVar5);
      local_4._0_1_ = 9;
      FUN_00619730();
      local_4 = CONCAT31(local_4._1_3_,8);
      FUN_00619730();
      pvVar4 = *(void **)((int)pvVar4 + 0x10);
      this = local_78;
    }
  }
  local_64 = 0;
  local_8c = (void *)0x0;
  if (((uVar3 != 0) && (*(int *)((int)param_1 + 4) == -1)) && (iVar6 != 0)) {
    local_64 = *(int *)((int)this + 0x48) / iVar6;
    local_8c = (void *)(*(int *)((int)this + 0x48) % iVar6);
  }
  if (uVar3 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      for (local_94 = (void *)FUN_0041c210(*(int *)((int)this + 0x40)); local_94 != (void *)0x0;
          local_94 = *(void **)((int)local_94 + 0x10)) {
        if (uVar3 == 0) goto LAB_00538bf2;
        if (*(int *)((int)param_1 + 4) != -1) break;
        FUN_004fa150(local_94,&local_90);
        local_4 = CONCAT31(local_4._1_3_,0xb);
        pvVar4 = (void *)FUN_00505750(&local_90);
        uVar3 = FUN_00552210(local_4c,pvVar4,param_1);
        if (uVar3 != 0) {
          iVar6 = FUN_00528040(pvVar4,1);
          pvVar7 = (void *)iVar6;
          if ((int)local_8c <= iVar6) {
            pvVar7 = local_8c;
          }
          local_80 = iVar6 * local_64 + (int)pvVar7;
          local_8c = (void *)((int)local_8c - (int)pvVar7);
        }
        for (iVar6 = 0; ((uVar3 != 0 && (*(int *)((int)param_1 + 4) == -1)) && (iVar6 < local_80));
            iVar6 = iVar6 + 1) {
          FUN_004ece30(&local_98);
          local_4._0_1_ = 0xc;
          FUN_0054fa40(local_28);
          local_4._0_1_ = 0xd;
          uVar1 = (undefined1)local_4;
          local_4._0_1_ = 0xd;
          if (uVar3 != 0) {
            if (*(int *)((int)param_1 + 4) == -1) {
              puVar5 = (uint *)FUN_004c27b0(pvVar4,&local_74);
              local_4._0_1_ = 0xe;
              puVar9 = (uint *)FUN_00451110(local_78,&local_6c);
              local_4._0_1_ = 0xf;
              uVar3 = FUN_00551a20(local_4c,puVar9,puVar5,param_1,local_28,&local_98);
              local_4._0_1_ = 0xe;
              FUN_00619730();
              local_4._0_1_ = 0xd;
              FUN_00619730();
              uVar1 = (undefined1)local_4;
            }
            local_4._0_1_ = uVar1;
            if ((uVar3 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
              puVar10 = (undefined4 *)FUN_00451110(local_78,&local_88);
              local_4._0_1_ = 0x10;
              bVar2 = FUN_0053e020((void *)((int)local_94 + 0x50),&local_98,puVar10);
              uVar3 = CONCAT31(extraout_var_00,bVar2);
              local_4._0_1_ = 0xd;
              FUN_00619730();
            }
          }
          local_4._0_1_ = 0xc;
          FUN_0054fa90(local_28);
          local_4 = CONCAT31(local_4._1_3_,0xb);
          FUN_00619730();
        }
        local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
        local_4 = CONCAT31(local_4._1_3_,8);
        FUN_00619730();
      }
    }
    if (uVar3 != 0) goto LAB_00538c24;
  }
LAB_00538bf2:
  local_88 = 1;
  local_84 = 1;
  local_4._0_1_ = 0x11;
  FUN_00520580(param_1,&local_88);
  local_4 = CONCAT31(local_4._1_3_,8);
  FUN_00619730();
LAB_00538c24:
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005510b0(local_4c);
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return uVar3;
}

