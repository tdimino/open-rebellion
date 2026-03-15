
uint __thiscall FUN_0053d430(void *this,int *param_1)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  uint *puVar5;
  uint local_a0;
  undefined4 local_9c;
  uint local_98;
  void *local_94;
  uint local_90;
  void *local_8c;
  undefined4 local_88;
  undefined4 local_84;
  int local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined1 local_74 [4];
  undefined1 local_70 [4];
  int local_6c [2];
  uint local_64 [2];
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [9];
  undefined4 local_28 [7];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00646710;
  pvStack_c = ExceptionList;
  local_a0 = 1;
  local_9c = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  local_8c = this;
  FUN_00520580(param_1,&local_a0);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00541e70(local_6c);
  local_4 = 1;
  FUN_0051fcb0(this,local_6c);
  uVar3 = FUN_005202d0(this,param_1);
  local_90 = 0;
  if ((uVar3 != 0) && (param_1[1] == -1)) {
    uVar3 = FUN_00553b80(*(int *)((int)this + 0x40),&local_90);
  }
  local_94 = (void *)0x0;
  local_7c = 1;
  local_78 = 0xffffffff;
  local_4._0_1_ = 2;
  if (uVar3 != 0) {
    if (param_1[1] == -1) {
      local_a0 = 1;
      local_9c = 0x22;
      local_4._0_1_ = 3;
      uVar3 = FUN_00504e60(local_64,&local_94,&local_a0,&local_7c);
      local_4._0_1_ = 2;
      FUN_00619730();
      if ((uVar3 != 0) && (local_94 == (void *)0x0)) {
        FUN_00520580(param_1,&local_7c);
      }
    }
    if ((uVar3 != 0) && (param_1[1] == -1)) {
      local_98 = 1;
      local_80 = 1;
      pvVar4 = (void *)FUN_0041c210(*(int *)((int)this + 0x40));
      while (((pvVar4 != (void *)0x0 && (param_1[1] == -1)) && (local_98 != 0))) {
        puVar5 = (uint *)FUN_004fa150(pvVar4,local_70);
        local_58 = 0x1c;
        local_5c = 0x14;
        local_4._0_1_ = 5;
        if ((*puVar5 >> 0x18 < 0x14) || (0x1b < *puVar5 >> 0x18)) {
          puVar5 = (uint *)FUN_004fa150(pvVar4,local_74);
          local_54 = 8;
          local_50 = 0x10;
          local_4._0_1_ = 6;
          if ((*puVar5 >> 0x18 < 8) || (0xf < *puVar5 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 5;
          FUN_00619730();
          if (bVar1) goto LAB_0053d644;
          bVar1 = true;
        }
        else {
LAB_0053d644:
          bVar1 = false;
        }
        local_4._0_1_ = 4;
        FUN_00619730();
        local_4._0_1_ = 2;
        FUN_00619730();
        if (bVar1) {
          local_98 = 0;
        }
        puVar5 = (uint *)FUN_004fa150(pvVar4,&local_a0);
        local_88 = 0x10;
        local_84 = 0x14;
        local_4._0_1_ = 7;
        if ((*puVar5 >> 0x18 < 0x10) || (0x13 < *puVar5 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 2;
        FUN_00619730();
        if (!bVar1) {
          local_80 = 0;
        }
        pvVar4 = *(void **)((int)pvVar4 + 0x10);
      }
      bVar1 = false;
      puVar5 = FUN_004025b0(local_94,&local_a0);
      local_88 = 0x90;
      local_84 = 0x98;
      local_4._0_1_ = 8;
      if ((*puVar5 >> 0x18 < 0x90) || (0x97 < *puVar5 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 2;
      FUN_00619730();
      if (bVar2) {
        if (((*(byte *)((int)local_94 + 0x50) & 0x40) == 0) ||
           ((*(byte *)((int)local_94 + 0x88) & 1) != 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
      }
      this = local_8c;
      if ((((*(uint *)((int)local_94 + 0x24) >> 6 & 3) != local_90) && (local_98 == 0)) &&
         ((local_80 == 0 || (!bVar1)))) {
        local_a0 = 1;
        local_9c = 0x28;
        local_4._0_1_ = 9;
        FUN_00520580(param_1,&local_a0);
        local_4._0_1_ = 2;
        FUN_00619730();
        this = local_8c;
      }
    }
  }
  FUN_00551060(local_4c);
  local_4._0_1_ = 10;
  if (uVar3 != 0) {
    if (param_1[1] == -1) {
      local_a0 = 3;
      uVar3 = FUN_00551190(local_4c,local_6c,local_90,&local_a0);
      if (uVar3 == 0) goto LAB_0053d922;
      uVar3 = FUN_005513a0(local_4c,local_64);
    }
    if (uVar3 != 0) {
      if (param_1[1] == -1) {
        pvVar4 = (void *)FUN_0041c210(*(int *)((int)this + 0x40));
        while (((pvVar4 != (void *)0x0 && (uVar3 != 0)) && (param_1[1] == -1))) {
          FUN_004ece30(&local_98);
          local_4._0_1_ = 0xb;
          FUN_0054fa40(local_28);
          local_4._0_1_ = 0xc;
          puVar5 = (uint *)FUN_004fa150(pvVar4,&local_a0);
          local_4._0_1_ = 0xd;
          uVar3 = FUN_00551630(local_4c,puVar5,local_64,param_1,local_28,&local_98);
          local_4 = CONCAT31(local_4._1_3_,0xc);
          FUN_00619730();
          if ((uVar3 != 0) && (param_1[1] == -1)) {
            FUN_004f26d0((void *)((int)pvVar4 + 0x48),&local_98);
          }
          local_4._0_1_ = 0xb;
          FUN_0054fa90(local_28);
          local_4._0_1_ = 10;
          FUN_00619730();
          pvVar4 = *(void **)((int)pvVar4 + 0x10);
        }
      }
      if (uVar3 != 0) goto LAB_0053d954;
    }
  }
LAB_0053d922:
  local_a0 = 1;
  local_9c = 1;
  local_4._0_1_ = 0xe;
  FUN_00520580(param_1,&local_a0);
  local_4._0_1_ = 10;
  FUN_00619730();
LAB_0053d954:
  local_4._0_1_ = 2;
  FUN_005510b0(local_4c);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return uVar3;
}

