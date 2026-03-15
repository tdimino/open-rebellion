
void __thiscall FUN_004714e0(void *this,int param_1,int param_2)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  uint *puVar5;
  void *pvVar6;
  int *piVar7;
  undefined4 uVar8;
  undefined1 *puVar9;
  void *pvVar10;
  undefined4 *puVar11;
  void *this_00;
  uint *puVar12;
  int iVar13;
  short sVar14;
  short sVar15;
  undefined4 *puVar16;
  undefined2 *puVar17;
  uint local_c8;
  undefined4 local_c4;
  undefined4 local_c0;
  undefined2 local_bc;
  undefined2 uStack_ba;
  undefined1 *local_b8;
  undefined4 local_b4;
  uint *local_b0;
  undefined4 local_ac [2];
  uint local_a4;
  uint uStack_a0;
  int local_9c;
  undefined4 local_98 [2];
  undefined4 local_90 [2];
  undefined4 local_88;
  undefined4 local_84;
  undefined4 local_80 [2];
  undefined4 auStack_78 [3];
  undefined2 uStack_6c;
  undefined2 uStack_6a;
  undefined4 local_68;
  undefined4 local_64;
  undefined2 uStack_60;
  undefined2 uStack_5e;
  undefined4 local_5c [7];
  int local_40;
  undefined4 local_3c [7];
  int local_20;
  tagRECT tStack_1c;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006326e0;
  local_c = ExceptionList;
  if ((*(int *)((int)this + 0x180) == param_1) && (param_2 == 0)) {
    return;
  }
  ExceptionList = &local_c;
  FUN_00520570(&local_a4);
  local_4 = 0;
  local_b0 = (uint *)0x0;
  FUN_004f5510(local_ac);
  local_4._0_1_ = 1;
  FUN_004f5510(local_98);
  local_4._0_1_ = 2;
  FUN_004f5510(local_90);
  local_4._0_1_ = 3;
  FUN_004f5510(local_80);
  local_4._0_1_ = 4;
  iVar13 = *(int *)(*(int *)((int)this + 0x15c) + 0x94);
  if (iVar13 == 0) {
    iVar13 = 0;
  }
  else {
    iVar13 = *(int *)(iVar13 + 0x24);
  }
  if (iVar13 == 0x70) {
    FUN_004f5610(local_80,(void *)(*(int *)((int)this + 0x174) + 0x78));
    local_c8 = 1;
  }
  else {
    if (iVar13 != 0x71) goto LAB_00471f88;
    FUN_004f5610(local_80,(void *)(*(int *)((int)this + 0x174) + 0x80));
    local_c8 = 2;
  }
  FUN_00472750((int)this);
  switch(param_1) {
  case 0x65:
    puVar9 = &DAT_00000002;
    sVar14 = 0xa5;
    sVar15 = 0x46;
    local_b8 = (undefined1 *)0x14;
    local_b4 = 0x1c;
    local_4._0_1_ = 5;
    FUN_00520580(&local_a4,&local_b8);
    local_4._0_1_ = 4;
    FUN_00619730();
    break;
  case 0x66:
    puVar9 = &DAT_00000002;
    sVar14 = 0xa5;
    sVar15 = 0x46;
    local_88 = 0x1c;
    local_84 = 0x20;
    local_4._0_1_ = 6;
    FUN_00520580(&local_a4,&local_88);
    local_4._0_1_ = 4;
    FUN_00619730();
    break;
  case 0x67:
    puVar9 = &DAT_00000002;
    sVar14 = 0xa5;
    sVar15 = 0x46;
    local_68 = 0x10;
    local_64 = 0x14;
    local_4._0_1_ = 7;
    FUN_00520580(&local_a4,&local_68);
    local_4._0_1_ = 4;
    FUN_00619730();
    break;
  case 0x68:
    puVar9 = (undefined1 *)0x3;
    sVar14 = 0x6e;
    sVar15 = 0x32;
    local_c4 = 0x30;
    local_c0 = 0x40;
    local_4._0_1_ = 8;
    FUN_00520580(&local_a4,&local_c4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_b0 = (uint *)0x1;
    break;
  default:
    sVar15 = (short)local_b8;
    puVar9 = local_b8;
    sVar14 = sVar15;
  }
  local_c4 = FUN_006076b0(this,(int)puVar9);
  uVar4 = FUN_006073d0(this);
  if (local_c4 != uVar4) {
    puVar5 = (uint *)FUN_006075e0(this,(int)puVar9);
    pvVar6 = *(void **)((int)this + 0x120);
    FUN_00608300(pvVar6,puVar5,*(int *)((int)pvVar6 + 0x28),*(uint *)((int)pvVar6 + 0x2c));
    FUN_006082c0(*(void **)((int)this + 0x120),(int)sVar14,(int)sVar15);
  }
  iVar13 = (local_c8 != 1) + 1;
  local_9c = iVar13;
  for (pvVar6 = (void *)thunk_FUN_005f5060(*(int *)((int)this + 0x174) + 0x60);
      pvVar6 != (void *)0x0; pvVar6 = *(void **)((int)pvVar6 + 0x10)) {
    puVar5 = (uint *)FUN_00403040(pvVar6,&local_c4);
    local_4._0_1_ = 9;
    piVar7 = FUN_004f36a0(iVar13,puVar5);
    local_4._0_1_ = 4;
    FUN_00619730();
    if ((piVar7 != (int *)0x0) && (((uint)piVar7[9] >> 6 & 3) == local_c8)) {
      FUN_00472010(piVar7,local_ac,local_98,param_1);
    }
  }
  FUN_004ffef0(local_3c,*(undefined4 *)((int)this + 0x114),1,local_c8);
  local_4._0_1_ = 10;
  FUN_00513120((int)local_3c);
  while (local_20 != 0) {
    puVar11 = local_98;
    puVar16 = local_ac;
    iVar13 = param_1;
    uVar8 = FUN_0052bed0((int)local_3c);
    FUN_00472010(uVar8,puVar16,puVar11,iVar13);
    FUN_005130d0((int)local_3c);
  }
  if (param_1 == 0x66) {
    FUN_00503a50(local_5c,*(undefined4 *)((int)this + 0x114),1,local_c8);
    local_4._0_1_ = 0xb;
    FUN_00513120((int)local_5c);
    bVar2 = false;
    while (local_40 != 0) {
      iVar13 = FUN_0052bed0((int)local_5c);
      local_b8 = (undefined1 *)(*(uint *)(iVar13 + 0x50) >> 2 & 1);
      if (local_b8 != (undefined1 *)0x0) {
        puVar9 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 0xc;
        local_b8 = puVar9;
        if (puVar9 == (undefined1 *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          puVar5 = &local_c4;
          pvVar6 = (void *)FUN_0052bed0((int)local_5c);
          puVar5 = FUN_004025b0(pvVar6,puVar5);
          bVar2 = true;
          local_4 = CONCAT31(local_4._1_3_,0xd);
          pvVar6 = FUN_004ecc70(puVar9,puVar5);
        }
        local_4 = 0xc;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 0xb;
        FUN_004f57b0(local_ac,pvVar6);
      }
      FUN_005130d0((int)local_5c);
    }
    bVar2 = false;
    for (pvVar6 = (void *)thunk_FUN_005f5060(*(int *)((int)this + 0x174) + 0x68);
        pvVar6 != (void *)0x0; pvVar6 = *(void **)((int)pvVar6 + 0x10)) {
      puVar5 = (uint *)FUN_00403040(pvVar6,&local_c4);
      local_4._0_1_ = 0xe;
      piVar7 = FUN_004f3590(local_9c,puVar5);
      local_4._0_1_ = 0xb;
      FUN_00619730();
      if ((piVar7 != (int *)0x0) && (((uint)piVar7[9] >> 6 & 3) == local_c8)) {
        puVar9 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 0xf;
        local_b8 = puVar9;
        if (puVar9 == (undefined1 *)0x0) {
          pvVar10 = (void *)0x0;
        }
        else {
          puVar5 = FUN_004025b0(piVar7,(uint *)&local_bc);
          bVar2 = true;
          local_4 = CONCAT31(local_4._1_3_,0x10);
          pvVar10 = FUN_004ecc70(puVar9,puVar5);
        }
        local_4 = 0xf;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 0xb;
        FUN_004f57b0(local_ac,pvVar10);
      }
    }
    local_4._0_1_ = 10;
    FUN_00503ad0(local_5c);
  }
  else if ((param_1 == 0x68) &&
          (pvVar6 = (void *)thunk_FUN_005f5060(*(int *)((int)this + 0x174) + 0x70),
          pvVar6 != (void *)0x0)) {
    bVar2 = false;
    do {
      puVar5 = (uint *)FUN_00403040(pvVar6,&local_c4);
      local_4._0_1_ = 0x11;
      piVar7 = FUN_004f2ec0(local_9c,puVar5);
      local_4._0_1_ = 10;
      FUN_00619730();
      if ((piVar7 != (int *)0x0) && (((uint)piVar7[9] >> 6 & 3) == local_c8)) {
        puVar9 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 0x12;
        local_b8 = puVar9;
        if (puVar9 == (undefined1 *)0x0) {
          pvVar10 = (void *)0x0;
        }
        else {
          puVar5 = FUN_004025b0(piVar7,(uint *)&local_bc);
          bVar2 = true;
          local_4 = CONCAT31(local_4._1_3_,0x13);
          pvVar10 = FUN_004ecc70(puVar9,puVar5);
        }
        local_4 = 0x12;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 10;
        FUN_004f57b0(local_ac,pvVar10);
      }
      pvVar6 = *(void **)((int)pvVar6 + 0x10);
    } while (pvVar6 != (void *)0x0);
  }
  bVar2 = false;
  for (pvVar6 = (void *)thunk_FUN_005f5060((int)local_80); pvVar6 != (void *)0x0;
      pvVar6 = *(void **)((int)pvVar6 + 0x10)) {
    puVar5 = (uint *)FUN_00403040(pvVar6,&local_c4);
    if ((*puVar5 >> 0x18 < local_a4) || (uStack_a0 <= *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      puVar9 = (undefined1 *)FUN_00618b70(0x1c);
      local_4._0_1_ = 0x14;
      local_b8 = puVar9;
      if (puVar9 == (undefined1 *)0x0) {
        pvVar10 = (void *)0x0;
      }
      else {
        puVar11 = (undefined4 *)FUN_00403040(pvVar6,&local_bc);
        bVar2 = true;
        local_4 = CONCAT31(local_4._1_3_,0x15);
        pvVar10 = FUN_004ecc70(puVar9,puVar11);
      }
      local_4 = 0x14;
      if (bVar2) {
        bVar2 = false;
        FUN_00619730();
      }
      local_4._0_1_ = 10;
      FUN_004f57b0(local_90,pvVar10);
    }
  }
  pvVar6 = (void *)thunk_FUN_005f5060((int)local_ac);
  pvVar10 = (void *)thunk_FUN_005f5060((int)local_98);
  this_00 = (void *)thunk_FUN_005f5060((int)local_90);
  FUN_005f2f50(auStack_78);
  local_4 = CONCAT31(local_4._1_3_,0x16);
  bVar2 = false;
  bVar3 = false;
  bVar1 = false;
LAB_00471c8c:
  if (pvVar6 != (void *)0x0) goto LAB_00471c90;
  if (bVar2) {
    puVar11 = (undefined4 *)0x0;
  }
  else {
    bVar2 = true;
    iVar13 = thunk_FUN_005f5060((int)local_90);
    if ((iVar13 == 0) && (iVar13 = thunk_FUN_005f5060((int)local_98), iVar13 == 0)) {
      uStack_60 = 0x8526;
      uStack_5e = DAT_0065d424;
      puVar17 = &uStack_60;
    }
    else {
      puVar17 = &uStack_6c;
      uStack_6c = 0x1940;
      uStack_6a = DAT_0065d424;
    }
    FUN_005f3010(auStack_78,puVar17);
    puVar11 = auStack_78;
  }
  FUN_004724a0(this,(int *)0x0,(uint *)0x0,(uint)puVar11);
LAB_00471d6a:
  if (param_1 == 0x68) {
    if (pvVar10 == (void *)0x0) {
      if (bVar1) {
        puVar11 = (undefined4 *)0x0;
      }
      else {
        bVar1 = true;
        local_bc = 0x8526;
        uStack_ba = DAT_0065d424;
        FUN_005f3010(auStack_78,&local_bc);
        puVar11 = auStack_78;
      }
      FUN_004724a0(this,(int *)0x0,(uint *)0x0,(uint)puVar11);
    }
    else {
      puVar12 = (uint *)FUN_00403040(pvVar10,&local_88);
      uVar4 = 0;
      local_4._0_1_ = 0x18;
      puVar5 = local_b0;
      piVar7 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x118) + 0x9c),puVar12);
      FUN_004724a0(this,piVar7,puVar5,uVar4);
      local_4 = CONCAT31(local_4._1_3_,0x16);
      FUN_00619730();
      pvVar10 = *(void **)((int)pvVar10 + 0x10);
      if (pvVar10 == (void *)0x0) {
        bVar1 = true;
      }
    }
  }
  if (this_00 == (void *)0x0) {
    if (bVar3) {
      puVar11 = (undefined4 *)0x0;
    }
    else {
      bVar3 = true;
      local_c4 = CONCAT22(DAT_0065d424,0x1941);
      FUN_005f3010(auStack_78,&local_c4);
      puVar11 = auStack_78;
    }
    FUN_004724a0(this,(int *)0x0,(uint *)0x0,(uint)puVar11);
  }
  else {
    puVar12 = (uint *)FUN_00403040(this_00,&local_b8);
    uVar4 = 0;
    local_4._0_1_ = 0x19;
    puVar5 = local_b0;
    piVar7 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x118) + 0x9c),puVar12);
    FUN_004724a0(this,piVar7,puVar5,uVar4);
    local_4 = CONCAT31(local_4._1_3_,0x16);
    FUN_00619730();
    this_00 = *(void **)((int)this_00 + 0x10);
    if (this_00 != (void *)0x0) goto LAB_00471c8c;
    bVar3 = true;
  }
  if (this_00 != (void *)0x0) goto LAB_00471c8c;
  if (pvVar6 == (void *)0x0) {
    thunk_FUN_005f50a0(local_ac);
    thunk_FUN_005f50a0(local_90);
    piVar7 = (int *)(**(code **)(**(int **)((int)this + 0x124) + 8))();
    iVar13 = piVar7[8];
    goto joined_r0x00471ef4;
  }
LAB_00471c90:
  puVar12 = (uint *)FUN_00403040(pvVar6,&local_68);
  uVar4 = 0;
  local_4._0_1_ = 0x17;
  puVar5 = local_b0;
  piVar7 = FUN_004f2d10(local_9c,puVar12);
  FUN_004724a0(this,piVar7,puVar5,uVar4);
  local_4 = CONCAT31(local_4._1_3_,0x16);
  FUN_00619730();
  pvVar6 = *(void **)((int)pvVar6 + 0x10);
  if (pvVar6 == (void *)0x0) {
    bVar2 = true;
  }
  goto LAB_00471d6a;
joined_r0x00471ef4:
  if (iVar13 != 0) goto LAB_00471f08;
  piVar7 = (int *)(**(code **)(*piVar7 + 0xc))();
  if (piVar7 == (int *)0x0) goto LAB_00471f1b;
  iVar13 = piVar7[8];
  goto joined_r0x00471ef4;
LAB_00471f08:
  if (piVar7 != (int *)0x0) {
    *(undefined1 *)(*(int *)((int)this + 0x120) + 0xd4) = *(undefined1 *)(iVar13 + 8);
  }
LAB_00471f1b:
  FUN_0060a280(*(void **)((int)this + 0x120));
  SetRect(&tStack_1c,0xc,0x28,0x167,0x88);
  InvalidateRect(*(HWND *)((int)this + 0x18),&tStack_1c,0);
  *(int *)((int)this + 0x180) = param_1;
  local_4._0_1_ = 10;
  FUN_005f2ff0(auStack_78);
  local_4._0_1_ = 4;
  FUN_004fff70(local_3c);
LAB_00471f88:
  local_4._0_1_ = 3;
  FUN_004f55c0(local_80);
  local_4._0_1_ = 2;
  FUN_004f55c0(local_90);
  local_4._0_1_ = 1;
  FUN_004f55c0(local_98);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f55c0(local_ac);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

