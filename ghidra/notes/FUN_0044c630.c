
void __fastcall FUN_0044c630(void *param_1)

{
  undefined1 uVar1;
  undefined4 uVar2;
  uint *puVar3;
  HDC pHVar4;
  HDC hdc;
  void *pvVar5;
  int iVar6;
  int iVar7;
  uint *puVar8;
  HGDIOBJ pvVar9;
  char *pcVar10;
  int *piVar11;
  void *this;
  undefined *puVar12;
  undefined2 extraout_var;
  short sVar13;
  short sVar14;
  uint uVar15;
  short sVar16;
  uint uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 *puVar20;
  int iVar21;
  undefined4 local_a8;
  undefined4 uStack_a4;
  uint *local_a0;
  HGDIOBJ pvStack_9c;
  undefined4 local_98;
  undefined4 *local_94;
  undefined4 local_90;
  undefined4 local_8c;
  undefined4 local_88 [6];
  int iStack_70;
  int iStack_68;
  undefined4 uStack_5c;
  undefined4 *local_54;
  undefined4 *local_50;
  undefined4 auStack_4c [3];
  undefined4 local_40 [11];
  undefined4 uStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f80d;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00601880(local_88,(char *)0x0,0,0,0,0,0,0);
  local_4 = 0;
  FUN_00601880(local_40,(char *)0x0,0,0,0,0,0,0);
  local_98 = *(void **)((int)param_1 + 0x1c);
  local_4._0_1_ = 1;
  uVar2 = FUN_006037f0(7);
  local_90 = uVar2;
  local_a8 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 2;
  if (local_a8 == (undefined4 *)0x0) {
    local_50 = (undefined4 *)0x0;
  }
  else {
    local_50 = (undefined4 *)
               FUN_005fbd20(local_a8,uVar2,
                            CONCAT22(extraout_var,
                                     *(undefined2 *)(*(int *)((int)param_1 + 0x174) + 0x2e)),10);
  }
  local_4._0_1_ = 1;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 1;
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    local_a8 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 3;
    if (local_a8 == (undefined4 *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = (uint *)FUN_005fbd20(local_a8,uVar2,0x285f,10);
    }
    local_4._0_1_ = 1;
    local_a0 = puVar3;
    local_a8 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,4);
    if (local_a8 == (undefined4 *)0x0) {
LAB_0044c7b6:
      local_94 = (undefined4 *)0x0;
    }
    else {
      local_94 = (undefined4 *)FUN_005fbd20(local_a8,uVar2,0x2a44,10);
    }
  }
  else {
    local_4._0_1_ = uVar1;
    local_a8 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 5;
    if (local_a8 == (undefined4 *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = (uint *)FUN_005fbd20(local_a8,uVar2,0x2860,10);
    }
    local_4._0_1_ = 1;
    local_a0 = puVar3;
    local_a8 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,6);
    if (local_a8 == (undefined4 *)0x0) goto LAB_0044c7b6;
    local_94 = (undefined4 *)FUN_005fbd20(local_a8,uVar2,0x2a45,10);
  }
  local_4._0_1_ = 1;
  local_a8 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 7;
  if (local_a8 == (undefined4 *)0x0) {
    local_54 = (undefined4 *)0x0;
  }
  else {
    local_54 = (undefined4 *)FUN_005fbd20(local_a8,uVar2,0x2a09,10);
  }
  local_4._0_1_ = 1;
  local_a8 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 8;
  if (local_a8 == (undefined4 *)0x0) {
    local_8c = (undefined4 *)0x0;
  }
  else {
    local_8c = (undefined4 *)FUN_005fbd20(local_a8,uVar2,0x2a0a,10);
  }
  local_4._0_1_ = 1;
  local_a8 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 9;
  if (local_a8 == (undefined4 *)0x0) {
    local_a8 = (undefined4 *)0x0;
  }
  else {
    local_a8 = (undefined4 *)FUN_005fbd20(local_a8,uVar2,0x2a0b,10);
  }
  local_4._0_1_ = 1;
  pHVar4 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar4);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar4);
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 10;
  uStack_a4 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar8 = (uint *)0x0;
  }
  else {
    puVar8 = puVar3;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)puVar3);
    iVar7 = FUN_005fc0e0((int *)puVar3);
    puVar8 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar8,pHVar4);
  }
  local_4._0_1_ = 1;
  FUN_005fcc30(local_50,puVar8,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar3,puVar8,0,0);
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    iVar6 = 0x19c;
  }
  else {
    iVar6 = 0x19f;
  }
  FUN_005fd0f0(local_94,puVar8,iVar6,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puVar8);
  pvStack_9c = SelectObject(hdc,pvVar9);
  iVar6 = *(int *)(*(int *)((int)param_1 + 0x174) + 0x6c);
  if (iVar6 == 1) {
    uStack_a4 = (void *)0x20000ff;
  }
  else if (iVar6 == 2) {
    uStack_a4 = (void *)0x200ff00;
  }
  else {
    uStack_a4 = (void *)0x2ffff00;
  }
  FUN_00601c90(local_40,(int)uStack_a4,(HWND)0x0);
  FUN_00601b30(local_40,0xc,0xd);
  FUN_00601c60(local_40,0xb);
  FUN_00403e90(local_40,1);
  local_40[0] = 400;
  uStack_14 = 1;
  pcVar10 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0x44);
  FUN_00601aa0(local_40,pcVar10);
  FUN_00601b80(local_40,hdc);
  FUN_00601ce0(local_40,hdc);
  FUN_00601c60(local_88,6);
  local_88[0] = 0x15e;
  uStack_5c = 1;
  FUN_00403e90(local_88,0x110);
  pcVar10 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x174) + 0x50);
  FUN_00601aa0(local_88,pcVar10);
  FUN_00601c90(local_88,0,(HWND)0x0);
  FUN_00601b30(local_88,0x1a,0xe2);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  FUN_00601c90(local_88,(int)uStack_a4,(HWND)0x0);
  FUN_00601b30(local_88,0x19,0xe1);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  FUN_006075b0(param_1,(int *)puVar8,1);
  SelectObject(hdc,pvStack_9c);
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xb;
  pvStack_9c = pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = local_a0;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)local_a0);
    iVar7 = FUN_005fc0e0((int *)local_a0);
    puVar3 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar3,pHVar4);
  }
  local_4._0_1_ = 1;
  FUN_005fcc30(local_a8,puVar3,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_a0,puVar3,0,0);
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    iVar6 = 0x19c;
  }
  else {
    iVar6 = 0x19f;
  }
  FUN_005fd0f0(local_94,puVar3,iVar6,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puVar3);
  pvStack_9c = SelectObject(hdc,pvVar9);
  FUN_00601ce0(local_40,hdc);
  FUN_005f2f50(auStack_4c);
  local_4._0_1_ = 0xc;
  FUN_0060b9d0(auStack_4c,0x7013,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
  FUN_00601c90(local_88,(int)uStack_a4,(HWND)0x0);
  pcVar10 = (char *)FUN_00583c40((int)auStack_4c);
  FUN_00601aa0(local_88,pcVar10);
  FUN_00601b30(local_88,0x19,0xe1);
  FUN_00601b80(local_88,hdc);
  FUN_00601b30(local_88,0x1d,(iStack_70 - iStack_68) + 0xb1);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  SelectObject(hdc,pvStack_9c);
  FUN_00607610(param_1,(int *)puVar3,5);
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xd;
  uStack_a4 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = local_a0;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)local_a0);
    iVar7 = FUN_005fc0e0((int *)local_a0);
    puVar3 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar3,pHVar4);
  }
  local_4._0_1_ = 0xc;
  FUN_005fcc30(local_54,puVar3,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_a0,puVar3,0,0);
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    iVar6 = 0x19c;
  }
  else {
    iVar6 = 0x19f;
  }
  FUN_005fd0f0(local_94,puVar3,iVar6,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puVar3);
  pvStack_9c = SelectObject(hdc,pvVar9);
  FUN_00601c90(local_88,0xffffff,(HWND)0x0);
  FUN_00601c60(local_88,4);
  FUN_00403e90(local_88,1);
  local_88[0] = 0xa5;
  FUN_00601b30(local_88,0x26,0x75);
  uStack_a4 = (void *)CONCAT22(DAT_0065d424,0x1932);
  FUN_006019a0(local_88,&uStack_a4);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  local_88[0] = 0xa5;
  FUN_00601b30(local_88,0xcc,0x75);
  uStack_a4 = (void *)CONCAT22(DAT_0065d424,0x1933);
  FUN_006019a0(local_88,&uStack_a4);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  FUN_00601ce0(local_40,hdc);
  SelectObject(hdc,pvStack_9c);
  FUN_00607610(param_1,(int *)puVar3,3);
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xe;
  uStack_a4 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = local_a0;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)local_a0);
    iVar7 = FUN_005fc0e0((int *)local_a0);
    puVar3 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar3,pHVar4);
  }
  local_4 = CONCAT31(local_4._1_3_,0xc);
  FUN_005fcc30(local_8c,puVar3,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_a0,puVar3,0,0);
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    iVar6 = 0x19c;
  }
  else {
    iVar6 = 0x19f;
  }
  FUN_005fd0f0(local_94,puVar3,iVar6,0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puVar3);
  pvStack_9c = SelectObject(hdc,pvVar9);
  FUN_00601c90(local_88,0x2ffffff,(HWND)0x0);
  local_88[0] = 0x6e;
  FUN_00601b30(local_88,0x26,0x75);
  uStack_a4 = (void *)CONCAT22(DAT_0065d424,0x1934);
  FUN_006019a0(local_88,&uStack_a4);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  local_88[0] = 0x6e;
  FUN_00601b30(local_88,0xa5,0x75);
  uStack_a4 = (void *)CONCAT22(DAT_0065d424,0x1935);
  FUN_006019a0(local_88,&uStack_a4);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  local_88[0] = 0x6e;
  FUN_00601b30(local_88,0x113,0x75);
  uStack_a4 = (void *)CONCAT22(DAT_0065d424,0x1936);
  FUN_006019a0(local_88,&uStack_a4);
  FUN_00601b80(local_88,hdc);
  FUN_00601ce0(local_88,hdc);
  FUN_00601ce0(local_40,hdc);
  SelectObject(hdc,pvStack_9c);
  FUN_00607610(param_1,(int *)puVar3,4);
  DeleteDC(hdc);
  puVar3 = local_a0;
  if (local_a0 != (uint *)0x0) {
    FUN_005fbfa0(local_a0);
    FUN_00618b60((undefined *)puVar3);
  }
  puVar20 = local_50;
  if (local_50 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_50);
    FUN_00618b60((undefined *)puVar20);
  }
  puVar20 = local_54;
  if (local_54 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_54);
    FUN_00618b60((undefined *)puVar20);
  }
  puVar20 = local_8c;
  if (local_8c != (undefined4 *)0x0) {
    FUN_005fbfa0(local_8c);
    FUN_00618b60((undefined *)puVar20);
  }
  puVar20 = local_a8;
  if (local_a8 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_a8);
    FUN_00618b60((undefined *)puVar20);
  }
  puVar20 = local_94;
  if (local_94 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_94);
    FUN_00618b60((undefined *)puVar20);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar16 = 0x1a7;
    sVar13 = 0x19;
    sVar14 = 0x20;
    uStack_a4 = (void *)0x1f;
    pvStack_9c = (HGDIOBJ)0x2882;
    local_a0 = (uint *)0x2883;
  }
  else {
    sVar16 = 0x1aa;
    sVar13 = 0x15;
    sVar14 = 0x2c;
    uStack_a4 = (void *)0x29;
    pvStack_9c = (HGDIOBJ)0x2888;
    local_a0 = (uint *)0x2889;
  }
  local_a8 = (undefined4 *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xf;
  if (local_a8 == (undefined4 *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar3 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00602150(local_a8,local_90,(int)sVar16,(int)sVar13,(int)sVar14,
                          (int)(short)uStack_a4,param_1,10,(uint)pvStack_9c,(uint)local_a0,puVar3,
                          iVar6,iVar7);
  }
  local_4._0_1_ = 0xc;
  *(void **)((int)param_1 + 0x164) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    FUN_00602840(*(void **)((int)param_1 + 0x164),0x260);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar16 = 0x1a2;
    sVar13 = 0x58;
    sVar14 = 0x29;
    uStack_a4 = (void *)0xcb;
  }
  else {
    sVar16 = 0x1aa;
    sVar13 = 0x59;
    sVar14 = 0x2c;
    uStack_a4 = (void *)0xda;
  }
  local_a8 = (undefined4 *)FUN_00618b70(0x9c);
  local_4._0_1_ = 0x10;
  if (local_a8 == (undefined4 *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    puVar3 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_0060d590(local_a8,local_98,(int)sVar16,(int)sVar13,(int)sVar14,
                          (int)(short)uStack_a4,100,param_1,puVar3);
  }
  local_4._0_1_ = 0xc;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 0xc;
  *(void **)((int)param_1 + 0x15c) = pvVar5;
  if (pvVar5 == (void *)0x0) goto LAB_0044d56f;
  FUN_005ffce0(pvVar5,0);
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar13 = 0x29;
    uVar15 = 0x29e8;
    local_a0 = (uint *)0x29e9;
  }
  else {
    sVar13 = 0x2c;
    uVar15 = 0x29f2;
    local_a0 = (uint *)0x29f3;
  }
  local_a8 = (undefined4 *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x11;
  if (local_a8 == (undefined4 *)0x0) {
    piVar11 = (int *)0x0;
  }
  else {
    piVar11 = (int *)FUN_00602150(local_a8,local_90,0,0,(int)sVar13,0x29,
                                  *(int *)((int)param_1 + 0x15c),0x65,uVar15,(uint)local_a0,
                                  *(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
  }
  local_4._0_1_ = 0xc;
  if (piVar11 != (int *)0x0) {
    FUN_005ffce0(piVar11,0);
    FUN_00602840(piVar11,0x260);
    local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1944);
    FUN_00600a40(param_1,piVar11,&local_a8);
    FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar11,1,0);
  }
  iVar6 = *(int *)(*(int *)((int)param_1 + 0x118) + 0x9c);
  if (iVar6 == 1) {
    sVar13 = 0x35;
    uStack_a4 = (void *)0x29;
  }
  else {
    sVar13 = 0x3b;
    uStack_a4 = (void *)0x2c;
  }
  if (*(int *)(*(int *)((int)param_1 + 0x174) + 0x6c) == 1) {
    if (iVar6 == 1) {
      uVar17 = 0x29ea;
      uVar15 = 0x29eb;
    }
    else {
      uVar17 = 0x29f4;
      uVar15 = 0x29f5;
    }
  }
  else if (iVar6 == 1) {
    uVar17 = 0x29ec;
    uVar15 = 0x29ed;
  }
  else {
    uVar17 = 0x29f6;
    uVar15 = 0x29f7;
  }
  local_a8 = (undefined4 *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x12;
  if (local_a8 == (undefined4 *)0x0) {
    piVar11 = (int *)0x0;
  }
  else {
    piVar11 = (int *)FUN_00602150(local_a8,local_90,0,(int)sVar13,(int)(short)uStack_a4,0x29,
                                  *(int *)((int)param_1 + 0x15c),0x66,uVar17,uVar15,
                                  *(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
  }
  local_4._0_1_ = 0xc;
  if (piVar11 != (int *)0x0) {
    FUN_005ffce0(piVar11,0);
    FUN_00602840(piVar11,0x260);
    iVar6 = *(int *)(*(int *)((int)param_1 + 0x174) + 0x6c);
    if (iVar6 == 1) {
      puVar20 = &local_8c;
      local_8c = (undefined4 *)CONCAT22(DAT_0065d424,0x1937);
LAB_0044d330:
      FUN_00600a40(param_1,piVar11,puVar20);
    }
    else if (iVar6 == 2) {
      local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1938);
      puVar20 = &local_a8;
      goto LAB_0044d330;
    }
    FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar11,0,1);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar13 = 0x6c;
    uStack_a4 = (void *)0x29;
    iVar6 = *(int *)(*(int *)((int)param_1 + 0x174) + 0x68);
    if (iVar6 == 1) {
      uVar17 = 0x29ea;
      uVar15 = 0x29eb;
    }
    else if (iVar6 == 2) {
      uVar17 = 0x29ec;
      uVar15 = 0x29ed;
    }
    else {
      uVar17 = 0x29f0;
      uVar15 = 0x29f1;
    }
  }
  else {
    sVar13 = 0x76;
    uStack_a4 = (void *)0x2c;
    iVar6 = *(int *)(*(int *)((int)param_1 + 0x174) + 0x68);
    if (iVar6 == 1) {
      uVar17 = 0x29f4;
      uVar15 = 0x29f5;
    }
    else if (iVar6 == 2) {
      uVar17 = 0x29f6;
      uVar15 = 0x29f7;
    }
    else {
      uVar17 = 0x29fa;
      uVar15 = 0x29fb;
    }
  }
  local_a8 = (undefined4 *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x13;
  if (local_a8 == (undefined4 *)0x0) {
    piVar11 = (int *)0x0;
  }
  else {
    piVar11 = (int *)FUN_00602150(local_a8,local_90,0,(int)sVar13,(int)(short)uStack_a4,0x29,
                                  *(int *)((int)param_1 + 0x15c),0x67,uVar17,uVar15,
                                  *(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0,0);
  }
  local_4._0_1_ = 0xc;
  if (piVar11 != (int *)0x0) {
    FUN_005ffce0(piVar11,0);
    FUN_00602840(piVar11,0x260);
    local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1943);
    FUN_00600a40(param_1,piVar11,&local_a8);
    FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar11,0,1);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar13 = 0xa2;
    sVar14 = 0x29;
    pvStack_9c = (HGDIOBJ)0x2a27;
    local_a0 = (uint *)0x2a28;
  }
  else {
    sVar13 = 0xb1;
    sVar14 = 0x2c;
    pvStack_9c = (HGDIOBJ)0x2918;
    local_a0 = (uint *)0x2919;
  }
  local_a8 = (undefined4 *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x14;
  if (local_a8 == (undefined4 *)0x0) {
    piVar11 = (int *)0x0;
  }
  else {
    piVar11 = (int *)FUN_00602150(local_a8,local_90,0,(int)sVar13,(int)sVar14,0x29,
                                  *(int *)((int)param_1 + 0x15c),0x68,(uint)pvStack_9c,
                                  (uint)local_a0,*(uint **)(*(int *)((int)param_1 + 0x15c) + 0x98),0
                                  ,0);
  }
  local_4._0_1_ = 0xc;
  uVar1 = (undefined1)local_4;
  if (piVar11 != (int *)0x0) {
    FUN_005ffce0(piVar11,0);
    FUN_00602840(piVar11,0x260);
    local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1939);
    FUN_00600a40(param_1,piVar11,&local_a8);
    FUN_0060d700(*(void **)((int)param_1 + 0x15c),piVar11,1,0);
    uVar1 = (undefined1)local_4;
  }
LAB_0044d56f:
  local_4._0_1_ = uVar1;
  pvVar5 = (void *)FUN_00618b70(0x9c);
  local_4._0_1_ = 0x15;
  local_a8 = (undefined4 *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    puVar3 = (uint *)FUN_006076b0(param_1,3);
    pvVar5 = FUN_0060d590(pvVar5,local_98,0x24,0x3b,0x169,0x29,0x96,param_1,puVar3);
  }
  local_4._0_1_ = 0xc;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 0xc;
  *(void **)((int)param_1 + 0x160) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    *(uint *)((int)pvVar5 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x160),0);
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar15 = 0x286c;
      uVar17 = 0x286b;
    }
    else {
      uVar15 = 0x2878;
      uVar17 = 0x2877;
    }
    local_a8 = (undefined4 *)FUN_00618b70(0x114);
    uVar2 = local_90;
    local_4._0_1_ = 0x16;
    if (local_a8 == (undefined4 *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(local_a8,local_90,0,0,0x31,0x29,*(int *)((int)param_1 + 0x160),
                                    0x97,uVar15,uVar17,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4._0_1_ = 0xc;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1928);
      FUN_00600a40(param_1,piVar11,&local_a8);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar11,
                   (uint)(*(int *)(*(int *)((int)param_1 + 0x174) + 100) == 0),0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar15 = 0x2872;
      local_a0 = (uint *)0x2871;
    }
    else {
      uVar15 = 0x287c;
      local_a0 = (uint *)0x287b;
    }
    local_a8 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x17;
    if (local_a8 == (undefined4 *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(local_a8,uVar2,0x3e,0,0x31,0x29,*(int *)((int)param_1 + 0x160),
                                    0x98,uVar15,(uint)local_a0,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4._0_1_ = 0xc;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1929);
      FUN_00600a40(param_1,piVar11,&local_a8);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar11,0,1);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar15 = 0x2868;
      local_a0 = (uint *)0x2867;
    }
    else {
      uVar15 = 0x2874;
      local_a0 = (uint *)0x2873;
    }
    local_a8 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x18;
    if (local_a8 == (undefined4 *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(local_a8,uVar2,0x7c,0,0x31,0x29,*(int *)((int)param_1 + 0x160),
                                    0x99,uVar15,(uint)local_a0,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4._0_1_ = 0xc;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1835);
      FUN_00600a40(param_1,piVar11,&local_a8);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar11,0,1);
    }
    local_a8 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x19;
    if (local_a8 == (undefined4 *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(local_a8,uVar2,0xba,0,0x31,0x29,*(int *)((int)param_1 + 0x160),
                                    0x9a,0x2866,0x2865,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4._0_1_ = 0xc;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1946);
      FUN_00600a40(param_1,piVar11,&local_a8);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar11,0,1);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar15 = 0x2870;
      local_a0 = (uint *)0x286f;
    }
    else {
      uVar15 = 0x287a;
      local_a0 = (uint *)0x2879;
    }
    local_a8 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x1a;
    if (local_a8 == (undefined4 *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(local_a8,uVar2,0xf8,0,0x31,0x29,*(int *)((int)param_1 + 0x160),
                                    0x9b,uVar15,(uint)local_a0,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4._0_1_ = 0xc;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1930);
      FUN_00600a40(param_1,piVar11,&local_a8);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar11,
                   *(int *)(*(int *)((int)param_1 + 0x174) + 100),0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      uVar15 = 0x286a;
      local_a0 = (uint *)0x2869;
    }
    else {
      uVar15 = 0x2876;
      local_a0 = (uint *)0x2875;
    }
    local_a8 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x1b;
    if (local_a8 == (undefined4 *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(local_a8,uVar2,0x138,0,0x31,0x29,*(int *)((int)param_1 + 0x160),
                                    0x9c,uVar15,(uint)local_a0,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4._0_1_ = 0xc;
    uVar1 = (undefined1)local_4;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      local_a8 = (undefined4 *)CONCAT22(DAT_0065d424,0x1931);
      FUN_00600a40(param_1,piVar11,&local_a8);
      FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar11,0,1);
      uVar1 = (undefined1)local_4;
    }
  }
  local_4._0_1_ = uVar1;
  local_a8 = (undefined4 *)FUN_00618b70(0x10);
  local_4._0_1_ = 0x1c;
  if (local_a8 == (undefined4 *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_0060a790(local_a8,4);
  }
  local_4._0_1_ = 0xc;
  *(void **)((int)param_1 + 0x124) = pvVar5;
  pvVar5 = (void *)FUN_00618b70(0x118);
  local_4._0_1_ = 0x1d;
  local_a8 = (undefined4 *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar19 = 0x46;
    uVar18 = 0xa5;
    uVar2 = 6;
    puVar3 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00607ea0(pvVar5,local_98,0x25,0x88,0x15b,0xa6,param_1,puVar3,uVar2,uVar18,uVar19);
  }
  local_4._0_1_ = 0xc;
  *(void **)((int)param_1 + 0x120) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    *(undefined4 *)((int)pvVar5 + 0xf4) = 0xb0000;
    *(uint *)(*(int *)((int)param_1 + 0x120) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 0x120) + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x120),0);
    iVar6 = *(int *)((int)param_1 + 0x120);
    FUN_0060a490((void *)(iVar6 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar6,7,0x29fc,
                 *(undefined4 *)(iVar6 + 0x3c),0,*(undefined4 *)(iVar6 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x120) + 0xe4) = 4;
    FUN_00608280(*(void **)((int)param_1 + 0x120),*(undefined4 *)((int)param_1 + 0x124));
    FUN_00609950(*(void **)((int)param_1 + 0x120),0xffffff);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    uVar15 = 0x2a0f;
    uVar17 = 0x2a0d;
  }
  else {
    uVar15 = 0x2a10;
    uVar17 = 0x2a0e;
  }
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x1e;
  local_98 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    iVar7 = 1;
    iVar6 = 0;
    puVar3 = (uint *)FUN_006076b0(param_1,5);
    pvVar5 = FUN_00602150(pvVar5,local_90,0xe2,0xb6,0xa9,0x60,param_1,0xc,uVar17,uVar15,puVar3,iVar6
                          ,iVar7);
  }
  local_4._0_1_ = 0xc;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 0xc;
  *(void **)((int)param_1 + 0x168) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    puVar3 = (uint *)FUN_00603550(pvVar5,0);
    pvVar5 = (void *)FUN_00603550(*(void **)((int)param_1 + 0x168),1);
    this = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x1f;
    local_98 = this;
    if (this == (void *)0x0) {
      puVar8 = (uint *)0x0;
    }
    else {
      puVar8 = puVar3;
      iVar6 = FUN_005fc0f0((int *)puVar3);
      iVar7 = FUN_005fc0e0((int *)puVar3);
      puVar8 = (uint *)FUN_005fbda0(this,iVar7,iVar6,puVar8);
    }
    local_4._0_1_ = 0xc;
    FUN_005fcc30(puVar3,puVar8,0,0,0,0,(undefined4 *)0x0,0);
    FUN_005fd0f0(pvVar5,puVar8,0,0);
    FUN_00603350(*(void **)((int)param_1 + 0x168),1,(int)puVar8);
    *(uint *)(*(int *)((int)param_1 + 0x168) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 0x168) + 0x50) & 0xefffffff;
    local_98 = (void *)CONCAT22(DAT_0065d424,0x1942);
    FUN_00602520(*(void **)((int)param_1 + 0x168),&local_98);
    FUN_00603010(*(void **)((int)param_1 + 0x168),0,0xffffff);
    FUN_006027e0(*(void **)((int)param_1 + 0x168),0,
                 *(int *)((int)*(void **)((int)param_1 + 0x168) + 0x34) / 2 + -0xc);
    FUN_005ffce0(*(void **)((int)param_1 + 0x168),0);
    (**(code **)(**(int **)((int)param_1 + 0x168) + 8))();
    uVar1 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar1;
  iVar6 = *(int *)(*(int *)((int)param_1 + 0x118) + 0x9c);
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x20;
  local_98 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    iVar21 = 1;
    iVar7 = 0;
    puVar3 = (uint *)FUN_006076b0(param_1,5);
    pvVar5 = FUN_00602150(pvVar5,local_90,0x1d,0xb6,0xa9,0x60,param_1,0xb,0x2a0c,
                          (iVar6 != 1) + 0x2a0f,puVar3,iVar7,iVar21);
  }
  local_4._0_1_ = 0xc;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 0xc;
  *(void **)((int)param_1 + 0x16c) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    piVar11 = FUN_004f2d10(*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c),
                           (uint *)(*(int *)((int)param_1 + 0x174) + 0x5c));
    puVar3 = (uint *)FUN_00603550(*(void **)((int)param_1 + 0x16c),0);
    local_98 = (void *)FUN_00603550(*(void **)((int)param_1 + 0x16c),1);
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x21;
    local_a8 = (undefined4 *)pvVar5;
    if (pvVar5 == (void *)0x0) {
      puVar8 = (uint *)0x0;
    }
    else {
      puVar8 = puVar3;
      iVar6 = FUN_005fc0f0((int *)puVar3);
      iVar7 = FUN_005fc0e0((int *)puVar3);
      puVar8 = (uint *)FUN_005fbda0(pvVar5,iVar7,iVar6,puVar8);
    }
    local_4._0_1_ = 0xc;
    FUN_005fcc30(puVar3,puVar8,0,0,0,0,(undefined4 *)0x0,0);
    FUN_005fd0f0(local_98,puVar8,0,0);
    FUN_00603350(*(void **)((int)param_1 + 0x16c),1,(int)puVar8);
    *(uint *)(*(int *)((int)param_1 + 0x16c) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 0x168) + 0x50) & 0xefffffff;
    if (piVar11 == (int *)0x0) {
      local_98 = (void *)CONCAT22(DAT_0065d424,0x1945);
      FUN_00602520(*(void **)((int)param_1 + 0x16c),&local_98);
    }
    else {
      puVar12 = FUN_004f62d0((int)piVar11);
      pcVar10 = (char *)FUN_00583c40((int)puVar12);
      FUN_00602650(*(void **)((int)param_1 + 0x16c),pcVar10);
    }
    FUN_00603010(*(void **)((int)param_1 + 0x16c),0,0xffffff);
    FUN_006027e0(*(void **)((int)param_1 + 0x16c),0,
                 *(int *)((int)*(void **)((int)param_1 + 0x16c) + 0x34) / 2 + -0xc);
    FUN_005ffce0(*(void **)((int)param_1 + 0x16c),0);
    (**(code **)(**(int **)((int)param_1 + 0x16c) + 8))();
    uVar1 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar1;
  iVar6 = *(int *)((int)param_1 + 0x118);
  FUN_00606980(param_1,*(int *)(iVar6 + 0xcc),*(int *)(iVar6 + 0xd0),*(int *)(iVar6 + 0xd4),
               *(int *)(iVar6 + 0xd8));
  FUN_0060d7e0(*(void **)((int)param_1 + 0x15c),0x65,1);
  local_4._0_1_ = 1;
  *(uint *)((int)param_1 + 0x180) =
       (-(uint)(*(int *)(*(int *)((int)param_1 + 0x174) + 100) != 0) & 4) + 0x97;
  FUN_005f2ff0(auStack_4c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00601990((int)local_40);
  local_4 = 0xffffffff;
  FUN_00601990((int)local_88);
  ExceptionList = pvStack_c;
  return;
}

