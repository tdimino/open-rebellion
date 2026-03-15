
void __fastcall FUN_00459e30(int *param_1)

{
  undefined4 uVar1;
  HDC hdc;
  HDC pHVar2;
  int iVar3;
  void *pvVar4;
  undefined *puVar5;
  char *pcVar6;
  int *piVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  undefined4 *puVar11;
  int *piVar12;
  HPALETTE pHVar13;
  uint *puVar14;
  longlong lVar15;
  undefined4 uVar16;
  uint uVar17;
  uint uVar18;
  int iVar19;
  undefined1 uVar20;
  int iVar21;
  int *local_a4;
  undefined4 *puStack_a0;
  void *pvStack_9c;
  uint *puStack_98;
  int iStack_94;
  HPALETTE pHStack_90;
  int iStack_8c;
  undefined **ppuStack_88;
  undefined4 uStack_84;
  int iStack_80;
  int iStack_7c;
  uint uStack_78;
  undefined2 uStack_70;
  undefined2 uStack_6e;
  undefined2 uStack_6c;
  undefined2 uStack_6a;
  int iStack_68;
  undefined2 uStack_64;
  undefined2 uStack_62;
  int iStack_60;
  undefined4 uStack_5c;
  tagRECT tStack_58;
  undefined4 local_48 [3];
  undefined4 uStack_3c;
  undefined4 auStack_38 [7];
  int iStack_1c;
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630895;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_18);
  local_4 = 0;
  FUN_005f2f50(local_48);
  local_4._0_1_ = 1;
  local_a4 = (int *)FUN_006037f0(7);
  uVar1 = FUN_006037f0(2);
  hdc = GetDC((HWND)param_1[6]);
  pHStack_90 = SelectPalette(hdc,DAT_006be5a0,0);
  pHVar2 = CreateCompatibleDC(hdc);
  param_1[0x58] = (int)pHVar2;
  pvStack_9c = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 2;
  if (pvStack_9c == (void *)0x0) {
    puStack_98 = (uint *)0x0;
  }
  else {
    puStack_98 = (uint *)FUN_005fbea0(pvStack_9c,param_1[0xc],param_1[0xd],
                                      *(uint **)(param_1[0x65] + 0x45c),hdc);
  }
  puVar14 = puStack_98;
  local_4._0_1_ = 1;
  FUN_006075b0(param_1,(int *)puStack_98,1);
  iVar3 = FUN_005fddd0(puVar14,&LAB_004ac550);
  param_1[0x57] = iVar3;
  DAT_006be5ac = uVar1;
  pvStack_9c = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 3;
  if (pvStack_9c == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_0060db00(pvStack_9c,param_1[0x65]);
  }
  local_4._0_1_ = 1;
  param_1[0x66] = (int)pvVar4;
  pvStack_9c = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 4;
  if (pvStack_9c == (void *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005fbea0(pvStack_9c,param_1[0xe],param_1[0xf],(uint *)param_1[0x29],hdc);
  }
  local_4._0_1_ = 1;
  param_1[0x67] = (int)pvVar4;
  SelectPalette(hdc,pHStack_90,0);
  ReleaseDC((HWND)param_1[6],hdc);
  *(undefined4 *)(param_1[0x49] + 4) = 0x14;
  FUN_00601b30((void *)param_1[0x49],2,2);
  FUN_00403e90((void *)param_1[0x49],0x25);
  puVar5 = FUN_004f62d0(param_1[0x51]);
  pcVar6 = (char *)FUN_00583c40((int)puVar5);
  FUN_00601aa0((void *)param_1[0x49],pcVar6);
  FUN_00601c60((void *)param_1[0x49],4);
  FUN_00601c90((void *)param_1[0x49],0x200f0f0,(HWND)0x0);
  pvStack_9c = (void *)FUN_00618b70(0x28);
  piVar12 = local_a4;
  local_4._0_1_ = 5;
  if (pvStack_9c == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_005fbd20(pvStack_9c,local_a4,0x277c,2);
  }
  local_4._0_1_ = 1;
  uVar20 = (undefined1)local_4;
  local_4._0_1_ = 1;
  if (piVar7 != (int *)0x0) {
    pvVar4 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 6;
    pvStack_9c = pvVar4;
    if (pvVar4 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      iVar3 = param_1[0xc];
      iVar21 = 1;
      iVar19 = 0;
      uVar18 = 0x277d;
      uVar17 = 0x277c;
      uVar16 = 0x138;
      piVar12 = param_1;
      puVar14 = puStack_98;
      iVar8 = FUN_005fc0f0(piVar7);
      iVar9 = FUN_005fc0e0(piVar7);
      uVar1 = 2;
      iVar10 = FUN_005fc0e0(piVar7);
      pvVar4 = FUN_00602150(pvVar4,local_a4,(iVar3 - iVar10) + -3,uVar1,iVar9,iVar8,piVar12,uVar16,
                            uVar17,uVar18,puVar14,iVar19,iVar21);
    }
    local_4._0_1_ = 1;
    param_1[99] = (int)pvVar4;
    FUN_005ffce0(pvVar4,0);
    FUN_005fbfa0(piVar7);
    FUN_00618b60((undefined *)piVar7);
    piVar12 = local_a4;
    uVar20 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar20;
  pvStack_9c = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 7;
  if (pvStack_9c == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_005fbd20(pvStack_9c,piVar12,0x27e2,2);
  }
  local_4._0_1_ = 1;
  uVar20 = (undefined1)local_4;
  local_4._0_1_ = 1;
  if (piVar7 != (int *)0x0) {
    pvVar4 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 8;
    pvStack_9c = pvVar4;
    if (pvVar4 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      iVar21 = 1;
      iVar19 = 0;
      iVar3 = *(int *)(param_1[99] + 0x28);
      uVar18 = 0x27e3;
      uVar17 = 0x27e2;
      uVar16 = 0x139;
      piVar12 = param_1;
      puVar14 = puStack_98;
      iVar8 = FUN_005fc0f0(piVar7);
      iVar9 = FUN_005fc0e0(piVar7);
      uVar1 = 2;
      iVar10 = FUN_005fc0e0(piVar7);
      pvVar4 = FUN_00602150(pvVar4,local_a4,iVar3 - iVar10,uVar1,iVar9,iVar8,piVar12,uVar16,uVar17,
                            uVar18,puVar14,iVar19,iVar21);
    }
    local_4._0_1_ = 1;
    param_1[100] = (int)pvVar4;
    FUN_005ffce0(pvVar4,0);
    FUN_005fbfa0(piVar7);
    FUN_00618b60((undefined *)piVar7);
    piVar12 = local_a4;
    uVar20 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar20;
  FUN_00607740(param_1,piVar12,0x2779,0x277a,0x277b,(int *)0x2778,(int *)0x2775,(int *)0x2774,0x2777
               ,(int *)0x2776);
  puVar11 = (undefined4 *)FUN_00526560((void *)param_1[0x51],&pvStack_9c);
  uStack_5c = *puVar11;
  pvVar4 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 9;
  pvStack_9c = pvVar4;
  if (pvVar4 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    uVar20 = 10;
    uVar17 = FUN_0045c970(1);
    piVar7 = (int *)FUN_005fbd20(pvVar4,piVar12,uVar17,uVar20);
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  iStack_60 = FUN_005fc0e0(piVar7);
  iStack_68 = FUN_005fc0f0(piVar7);
  if (piVar7 != (int *)0x0) {
    FUN_005fbfa0(piVar7);
    FUN_00618b60((undefined *)piVar7);
  }
  pvStack_9c = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 10;
  if (pvStack_9c == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    piVar12 = (int *)FUN_005fbd20(pvStack_9c,piVar12,0x2a13,10);
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  iVar3 = FUN_005fc0e0(piVar12);
  puStack_98 = (uint *)(iVar3 + 1);
  iStack_94 = FUN_005fc0f0(piVar12);
  iStack_94 = iStack_94 + 1;
  if (piVar12 != (int *)0x0) {
    FUN_005fbfa0(piVar12);
    FUN_00618b60((undefined *)piVar12);
  }
  FUN_00512f90(auStack_38,param_1[0x51],4);
  local_4 = CONCAT31(local_4._1_3_,0xb);
  FUN_00513120((int)auStack_38);
  while (iStack_1c != 0) {
    pHVar13 = (HPALETTE)FUN_0052bed0((int)auStack_38);
    pHStack_90 = pHVar13;
    FUN_004025b0(pHVar13,(uint *)&local_a4);
    local_4._0_1_ = 0xc;
    puStack_a0 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 0xd;
    if (puStack_a0 == (undefined4 *)0x0) {
      puVar11 = (undefined4 *)0x0;
    }
    else {
      puVar11 = FUN_004acd10(puStack_a0);
    }
    local_4 = CONCAT31(local_4._1_3_,0xc);
    FUN_004f26d0(puVar11 + 6,&local_a4);
    if (puVar11 != (undefined4 *)0x0) {
      FUN_004acba0(param_1 + 0x45,puVar11,0);
    }
    puStack_a0 = (undefined4 *)FUN_00618b70(0x74);
    local_4._0_1_ = 0xe;
    if (puStack_a0 == (undefined4 *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_004421d0(puStack_a0,(uint *)&local_a4);
    }
    local_4._0_1_ = 0xc;
    if (pvVar4 != (void *)0x0) {
      puStack_a0 = (undefined4 *)FUN_00509620(pHVar13,&uStack_3c);
      puStack_a0 = (undefined4 *)*puStack_a0;
      iStack_8c = (int)DAT_00659ef8;
      lVar15 = __ftol();
      iVar3 = (int)lVar15;
      puStack_a0 = (undefined4 *)(int)DAT_00659efa;
      lVar15 = __ftol();
      iVar8 = (int)lVar15;
      SetRect(&tStack_58,iVar3,iVar8,iStack_60 + iVar3,iStack_68 + iVar8);
      *(int *)((int)pvVar4 + 0x28) = iVar3;
      *(int *)((int)pvVar4 + 0x2c) = iVar8;
      *(LONG *)((int)pvVar4 + 0x40) = tStack_58.left;
      *(LONG *)((int)pvVar4 + 0x44) = tStack_58.top;
      *(LONG *)((int)pvVar4 + 0x48) = tStack_58.right;
      *(LONG *)((int)pvVar4 + 0x4c) = tStack_58.bottom;
      puVar5 = FUN_004f62d0((int)pHStack_90);
      pcVar6 = (char *)FUN_00583c40((int)puVar5);
      FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar6);
      iVar3 = iStack_68;
      *(int *)((int)pvVar4 + 0x34) = iStack_68 + 0xb;
      FUN_005f59f0(param_1 + 0x59,(int)pvVar4);
      puStack_a0 = (undefined4 *)FUN_00618b70(200);
      local_4._0_1_ = 0xf;
      if (puStack_a0 == (undefined4 *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_004acec0(puStack_a0,tStack_58.left,tStack_58.bottom + 2,
                              tStack_58.right - tStack_58.left,3,(int)param_1,0x2ffffff,0x2ff0000,
                              (uint *)0x0,0,0);
      }
      local_4._0_1_ = 0xc;
      if (pvVar4 != (void *)0x0) {
        *(uint *)((int)pvVar4 + 0x50) = *(uint *)((int)pvVar4 + 0x50) & 0xefffffff;
        FUN_0060e3b0(pvVar4,0x808080);
        FUN_005ffce0(pvVar4,0);
        *(uint *)((int)pvVar4 + 0x24) = ((uint)local_a4 & 0x3ff) << 6 | 1;
        FUN_005f4f10(param_1 + 0x1b,(int)pvVar4);
        uStack_70 = 0x1822;
        uStack_6e = DAT_0065d424;
        FUN_00600a40(param_1,pvVar4,&uStack_70);
      }
      puStack_a0 = (undefined4 *)FUN_00618b70(200);
      local_4._0_1_ = 0x10;
      if (puStack_a0 == (undefined4 *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_004acec0(puStack_a0,*(undefined4 *)((int)pvVar4 + 0x28),
                              *(int *)((int)pvVar4 + 0x2c) + 1 + *(int *)((int)pvVar4 + 0x34),
                              *(int *)((int)pvVar4 + 0x30),3,(int)param_1,0x200ffff,0x20f5cf9,
                              (uint *)0x0,0,0);
      }
      local_4._0_1_ = 0xc;
      if (pvVar4 != (void *)0x0) {
        *(uint *)((int)pvVar4 + 0x50) = *(uint *)((int)pvVar4 + 0x50) & 0xefffffff;
        FUN_0060e3b0(pvVar4,0x808080);
        FUN_005ffce0(pvVar4,0);
        *(uint *)((int)pvVar4 + 0x24) = ((uint)local_a4 & 0x3ff) << 6 | 2;
        FUN_005f4f10(param_1 + 0x1b,(int)pvVar4);
        uStack_6c = 0x1823;
        uStack_6a = DAT_0065d424;
        FUN_00600a40(param_1,pvVar4,&uStack_6c);
      }
      puStack_a0 = (undefined4 *)FUN_00618b70(200);
      local_4._0_1_ = 0x11;
      if (puStack_a0 == (undefined4 *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_004acec0(puStack_a0,*(undefined4 *)((int)pvVar4 + 0x28),
                              *(int *)((int)pvVar4 + 0x34) + 1 + *(int *)((int)pvVar4 + 0x2c),
                              *(int *)((int)pvVar4 + 0x30),3,(int)param_1,
                              (-(uint)(*(int *)(param_1[0x65] + 0x9c) != 1) & 0xfe01) + 0x20000ff,
                              (-(uint)(*(int *)(param_1[0x65] + 0x9c) != 1) & 0xffff01ff) +
                              0x200ff00,(uint *)0x0,0,0);
      }
      local_4._0_1_ = 0xc;
      if (pvVar4 != (void *)0x0) {
        *(uint *)((int)pvVar4 + 0x50) = *(uint *)((int)pvVar4 + 0x50) & 0xefffffff;
        *(uint *)((int)pvVar4 + 0x24) = ((uint)local_a4 & 0x3ff) << 6 | 3;
        FUN_005ffce0(pvVar4,0);
        uStack_64 = 0x1821;
        uStack_62 = DAT_0065d424;
        FUN_00600a40(param_1,pvVar4,&uStack_64);
        FUN_005f4f10(param_1 + 0x1b,(int)pvVar4);
      }
      iVar8 = iStack_60 / 2 + tStack_58.left;
      iVar3 = iVar3 / 2 + tStack_58.top;
      puStack_a0 = (undefined4 *)FUN_00618b70(0x74);
      local_4._0_1_ = 0x12;
      if (puStack_a0 == (undefined4 *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_00442130(puStack_a0,(uint)local_a4 & 0xffff | 0x40000,&local_a4);
      }
      local_4._0_1_ = 0xc;
      if (pvVar4 != (void *)0x0) {
        iStack_80 = (int)puStack_98 + (iVar8 - (int)puStack_98);
        iStack_7c = iStack_94 + (iVar3 - iStack_94);
        *(int *)((int)pvVar4 + 0x40) = iVar8 - (int)puStack_98;
        *(int *)((int)pvVar4 + 0x44) = iVar3 - iStack_94;
        *(int *)((int)pvVar4 + 0x48) = iStack_80;
        *(int *)((int)pvVar4 + 0x4c) = iStack_7c;
        uVar17 = FUN_005f4960((undefined4 *)((int)pvVar4 + 0x3c));
        *(uint *)((int)pvVar4 + 0x3c) = uVar17 & 0xfffffffd;
        pcVar6 = (char *)FUN_00583c40((int)local_48);
        FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar6);
        FUN_005f59f0(param_1 + 0x5d,(int)pvVar4);
      }
      puStack_a0 = (undefined4 *)FUN_00618b70(0x74);
      local_4._0_1_ = 0x13;
      if (puStack_a0 == (undefined4 *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_00442130(puStack_a0,(uint)local_a4 & 0xffff | 0x80000,&local_a4);
      }
      local_4._0_1_ = 0xc;
      if (pvVar4 != (void *)0x0) {
        iStack_80 = (int)puStack_98 + (iVar8 - (int)puStack_98);
        iStack_7c = iStack_94 + iVar3 + 1;
        *(int *)((int)pvVar4 + 0x40) = iVar8 - (int)puStack_98;
        *(int *)((int)pvVar4 + 0x44) = iVar3 + 1;
        *(int *)((int)pvVar4 + 0x48) = iStack_80;
        *(int *)((int)pvVar4 + 0x4c) = iStack_7c;
        uVar17 = FUN_005f4960((undefined4 *)((int)pvVar4 + 0x3c));
        *(uint *)((int)pvVar4 + 0x3c) = uVar17 & 0xfffffffd;
        pcVar6 = (char *)FUN_00583c40((int)local_48);
        FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar6);
        FUN_005f59f0(param_1 + 0x5d,(int)pvVar4);
      }
      puStack_a0 = (undefined4 *)FUN_00618b70(0x74);
      local_4._0_1_ = 0x14;
      if (puStack_a0 == (undefined4 *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_00442130(puStack_a0,(uint)local_a4 & 0xffff | 0x100000,&local_a4);
      }
      local_4._0_1_ = 0xc;
      if (pvVar4 != (void *)0x0) {
        iStack_80 = (int)puStack_98 + iVar8 + 1;
        iStack_7c = iStack_94 + (iVar3 - iStack_94);
        *(int *)((int)pvVar4 + 0x40) = iVar8 + 1;
        *(int *)((int)pvVar4 + 0x44) = iVar3 - iStack_94;
        *(int *)((int)pvVar4 + 0x48) = iStack_80;
        *(int *)((int)pvVar4 + 0x4c) = iStack_7c;
        uVar17 = FUN_005f4960((undefined4 *)((int)pvVar4 + 0x3c));
        *(uint *)((int)pvVar4 + 0x3c) = uVar17 & 0xfffffffd;
        pcVar6 = (char *)FUN_00583c40((int)local_48);
        FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar6);
        FUN_005f59f0(param_1 + 0x5d,(int)pvVar4);
      }
      puStack_a0 = (undefined4 *)FUN_00618b70(0x74);
      local_4._0_1_ = 0x15;
      if (puStack_a0 == (undefined4 *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_00442130(puStack_a0,(uint)local_a4 & 0xffff | 0x400000,&local_a4);
      }
      local_4._0_1_ = 0xc;
      if (pvVar4 != (void *)0x0) {
        *(int *)((int)pvVar4 + 0x40) = iVar8 + 1;
        *(int *)((int)pvVar4 + 0x44) = iVar3 + 1;
        *(int *)((int)pvVar4 + 0x48) = (int)puStack_98 + iVar8 + 1;
        *(int *)((int)pvVar4 + 0x4c) = iStack_94 + iVar3 + 1;
        uVar17 = FUN_005f4960((uint *)((int)pvVar4 + 0x3c));
        *(uint *)((int)pvVar4 + 0x3c) = uVar17 & 0xfffffffd;
        pcVar6 = (char *)FUN_00583c40((int)local_48);
        FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar6);
        FUN_005f59f0(param_1 + 0x5d,(int)pvVar4);
      }
      pHVar13 = pHStack_90;
      puVar14 = FUN_004025b0(pHStack_90,(uint *)&pvStack_9c);
      local_4._0_1_ = 0x16;
      (**(code **)(*param_1 + 100))(puVar14,pHVar13);
      local_4._0_1_ = 0xc;
      FUN_00619730();
    }
    local_4 = CONCAT31(local_4._1_3_,0xb);
    FUN_00619730();
    FUN_005130d0((int)auStack_38);
  }
  if ((*(byte *)(param_1[0x65] + 0xc0) & 4) == 0) {
    FUN_00610b20(&ppuStack_88,0,0,0,0,0,0);
    uStack_78 = uStack_78 & 0xfffffff9 | 1;
    local_4 = CONCAT31(local_4._1_3_,0x17);
    uStack_84 = 0x25c;
    iStack_80 = 7;
    FUN_00610c30((int)&ppuStack_88);
    ppuStack_88 = &PTR_LAB_00658518;
  }
  local_4._0_1_ = 1;
  FUN_00513010(auStack_38);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_48);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = pvStack_c;
  return;
}

