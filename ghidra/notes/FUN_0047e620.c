
undefined4 __fastcall FUN_0047e620(int *param_1)

{
  uint *this;
  uint uVar1;
  bool bVar2;
  bool bVar3;
  short sVar4;
  void *pvVar5;
  int *piVar6;
  uint *puVar7;
  undefined4 *puVar8;
  void *pvVar9;
  int iVar10;
  uint local_6c;
  uint local_68;
  uint *local_64;
  undefined4 local_60;
  uint local_5c;
  undefined1 local_58 [4];
  undefined1 local_54 [4];
  undefined1 local_50 [4];
  undefined1 local_4c [4];
  int local_48;
  undefined1 *local_44;
  undefined4 local_40;
  undefined4 local_3c [2];
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633ce6;
  pvStack_c = ExceptionList;
  local_60 = 0;
  ExceptionList = &pvStack_c;
  FUN_004f5510(local_3c);
  local_4 = 0;
  FUN_004ece30(&local_6c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_68);
  local_4._0_1_ = 2;
  FUN_004355d0(local_1c);
  local_4._0_1_ = 3;
  param_1[8] = param_1[8] & 0xffffffef;
  local_48 = FUN_004bd000((int)param_1);
  this = (uint *)(param_1 + 0xd);
  local_44 = (undefined1 *)0x90;
  local_40 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    puVar7 = (uint *)(param_1 + 0xc);
    local_44 = (undefined1 *)0x30;
    local_40 = 0x40;
    if (((uint)param_1[0xc] >> 0x18 < 0x30) || (0x3f < (uint)param_1[0xc] >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_64 = puVar7;
    FUN_00619730();
    if (bVar2) {
      pvVar5 = FUN_004f5940((void *)param_1[0x19],puVar7);
      FUN_004ece80(puVar7);
      if (pvVar5 != (void *)0x0) {
        FUN_004f26d0(&local_6c,(undefined4 *)((int)pvVar5 + 0x38));
        bVar2 = false;
        while( true ) {
          local_44 = (undefined1 *)0x90;
          local_40 = 0x98;
          if (((0x8f < local_6c >> 0x18) && (local_6c >> 0x18 < 0x98)) || (bVar2)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          FUN_00619730();
          if (!bVar3) break;
          piVar6 = FUN_004f2d10(param_1[9],&local_6c);
          if (piVar6 != (int *)0x0) {
            if ((void *)piVar6[7] == (void *)0x0) {
              FUN_004ece80(&local_6c);
              bVar2 = true;
            }
            else {
              puVar7 = FUN_004025b0((void *)piVar6[7],&local_5c);
              local_4._0_1_ = 4;
              FUN_004f26d0(&local_6c,puVar7);
              local_4._0_1_ = 3;
              FUN_00619730();
            }
          }
        }
        if (((*this == local_6c) &&
            (uVar1 = *(uint *)((int)pvVar5 + 0x30), (uVar1 & 0x20000000) != 0)) &&
           (((uVar1 & 2) != 0 && (((uVar1 & 0x40) != 0 && ((uVar1 | 0x20) == 0)))))) {
          puVar8 = FUN_00403040(pvVar5,&local_5c);
          local_4._0_1_ = 5;
          FUN_004f26d0(local_64,puVar8);
          local_4._0_1_ = 3;
          FUN_00619730();
        }
      }
    }
  }
  local_44 = (undefined1 *)0x30;
  local_40 = 0x40;
  if (((uint)param_1[0xc] >> 0x18 < 0x30) || (0x3f < (uint)param_1[0xc] >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  local_64 = (uint *)(param_1 + 0xc);
  FUN_00619730();
  if (!bVar2) {
    FUN_004ece80(this);
    FUN_004ece80(param_1 + 0xc);
    pvVar5 = FUN_00419a00((void *)param_1[0x1a],0,0x4000,0,0,0,0,0x44,(void *)0x1);
    FUN_00435790(param_1 + 0x15,(int)pvVar5);
    puVar8 = FUN_004357b0(param_1 + 0x15,&local_5c);
    local_4._0_1_ = 6;
    FUN_004f26d0(&local_68,puVar8);
    while( true ) {
      local_4._0_1_ = 3;
      FUN_00619730();
      local_34 = 0x80;
      local_30 = 0x90;
      if ((local_68 >> 0x18 < 0x80) || (0x8f < local_68 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (!bVar2) break;
      pvVar5 = FUN_0041a340((void *)param_1[0x1a],&local_68,0,0x4000,0,0,0,0,0x1d,(void *)0x1);
      FUN_00435790(local_1c,(int)pvVar5);
      puVar8 = FUN_004357b0(local_1c,&local_5c);
      local_4._0_1_ = 7;
      FUN_004f26d0(this,puVar8);
      while( true ) {
        local_4._0_1_ = 3;
        FUN_00619730();
        local_2c = 0x90;
        local_28 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) break;
        local_44 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 8;
        if (local_44 == (undefined1 *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecc70(local_44,this);
        }
        local_4._0_1_ = 3;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(local_3c,pvVar5);
        }
        puVar8 = FUN_004357b0(local_1c,local_58);
        local_4._0_1_ = 9;
        FUN_004f26d0(this,puVar8);
      }
      FUN_005f58b0(local_1c);
      pvVar5 = FUN_0041a340((void *)param_1[0x1a],&local_68,0,0x8000,0,0,0,0,0x1d,&DAT_00000002);
      FUN_00435790(local_1c,(int)pvVar5);
      puVar8 = FUN_004357b0(local_1c,local_54);
      local_4._0_1_ = 10;
      FUN_004f26d0(this,puVar8);
      while( true ) {
        local_4._0_1_ = 3;
        FUN_00619730();
        local_24 = 0x90;
        local_20 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) break;
        local_44 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 0xb;
        if (local_44 == (undefined1 *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecc70(local_44,this);
        }
        local_4._0_1_ = 3;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(local_3c,pvVar5);
        }
        puVar8 = FUN_004357b0(local_1c,local_50);
        local_4._0_1_ = 0xc;
        FUN_004f26d0(this,puVar8);
      }
      FUN_005f58b0(local_1c);
      puVar8 = FUN_004357b0(param_1 + 0x15,local_4c);
      local_4._0_1_ = 0xd;
      FUN_004f26d0(&local_68,puVar8);
    }
    FUN_005f58b0(param_1 + 0x15);
    sVar4 = FUN_005f50e0((int)local_3c);
    if (sVar4 != 0) {
      bVar2 = false;
      pvVar5 = (void *)thunk_FUN_005f5060((int)local_3c);
      while ((pvVar5 != (void *)0x0 && (!bVar2))) {
        puVar8 = FUN_00403040(pvVar5,local_4c);
        iVar10 = param_1[0x1a];
        local_44 = &stack0xffffff7c;
        local_4._0_1_ = 0xe;
        FUN_004f26d0(&stack0xffffff7c,puVar8);
        pvVar9 = FUN_00403d30((void *)(iVar10 + 0x2c));
        local_4._0_1_ = 3;
        FUN_00619730();
        bVar2 = false;
        if ((pvVar9 != (void *)0x0) &&
           ((iVar10 = (**(code **)(*param_1 + 0x44))(), local_48 == iVar10 ||
            (*(int *)((int)pvVar9 + 0xbc) / 2 <= local_48)))) {
          puVar8 = FUN_00403040(pvVar5,local_50);
          local_4._0_1_ = 0xf;
          FUN_004f26d0(this,puVar8);
          local_4._0_1_ = 3;
          FUN_00619730();
          bVar2 = true;
        }
        pvVar5 = *(void **)((int)pvVar5 + 0x10);
      }
      local_24 = 0x90;
      local_20 = 0x98;
      if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        FUN_0047ecf0((int)param_1);
      }
    }
  }
  local_24 = 0x30;
  local_20 = 0x40;
  if ((*local_64 >> 0x18 < 0x30) || (0x3f < *local_64 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    local_60 = 1;
    param_1[8] = param_1[8] | 0x10;
  }
  local_4._0_1_ = 2;
  FUN_00435610(local_1c);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f55c0(local_3c);
  ExceptionList = pvStack_c;
  return local_60;
}

