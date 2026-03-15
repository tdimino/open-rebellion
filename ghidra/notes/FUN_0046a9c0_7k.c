
void __fastcall FUN_0046a9c0(void *param_1)

{
  bool bVar1;
  uint *puVar2;
  void *pvVar3;
  uint *puVar4;
  void *pvVar5;
  int iVar6;
  int iVar7;
  uint *puVar8;
  HDC pHVar9;
  HDC pHVar10;
  HGDIOBJ pvVar11;
  undefined4 *puVar12;
  int *piVar13;
  char *pcVar14;
  undefined *puVar15;
  undefined1 uVar16;
  uint uVar17;
  uint uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  undefined2 uVar21;
  undefined2 uVar22;
  undefined2 uVar23;
  undefined2 uVar24;
  undefined4 uVar25;
  undefined4 uStack_c8;
  uint *local_c4;
  uint *puStack_c0;
  uint local_bc;
  HDC pHStack_b8;
  HDC pHStack_b4;
  uint *local_b0;
  undefined4 uStack_ac;
  uint uStack_a8;
  void *pvStack_a4;
  uint *local_a0;
  int iStack_9c;
  uint uStack_98;
  undefined1 auStack_94 [40];
  undefined4 uStack_6c;
  undefined4 auStack_64 [2];
  undefined4 auStack_5c [2];
  undefined4 uStack_54;
  int aiStack_50 [3];
  undefined1 auStack_44 [4];
  undefined1 local_40 [48];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631f71;
  pvStack_c = ExceptionList;
  local_bc = *(uint *)((int)param_1 + 0x1c);
  ExceptionList = &pvStack_c;
  puVar4 = (uint *)FUN_006037f0(7);
  local_b0 = puVar4;
  FUN_00601880(local_40,(char *)0x0,0,0,0,0,0,0);
  local_4 = 0;
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    local_c4 = (uint *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,1);
    if (local_c4 != (uint *)0x0) {
      puVar4 = (uint *)FUN_005fbd20(local_c4,puVar4,0x2a31,2);
      goto LAB_0046aa84;
    }
  }
  else {
    local_c4 = (uint *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,2);
    if (local_c4 != (uint *)0x0) {
      puVar4 = (uint *)FUN_005fbd20(local_c4,puVar4,0x2a32,2);
      goto LAB_0046aa84;
    }
  }
  puVar4 = (uint *)0x0;
LAB_0046aa84:
  local_4 = local_4 & 0xffffff00;
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 3;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar8 = (uint *)0x0;
  }
  else {
    puVar8 = puVar4;
    iVar6 = FUN_005fc0f0((int *)puVar4);
    iVar7 = FUN_005fc0e0((int *)puVar4);
    puVar8 = (uint *)FUN_005fbda0(pvVar5,iVar7 + 0x1e,iVar6,puVar8);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  local_a0 = puVar8;
  FUN_005fcc30(puVar4,puVar8,0,0,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(puVar4,puVar8,0x1e,0,0,0,(undefined4 *)0x0,0);
  if (puVar4 != (uint *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  pHVar9 = GetDC(*(HWND *)((int)param_1 + 0x18));
  pHVar10 = CreateCompatibleDC(pHVar9);
  pHStack_b4 = pHVar10;
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar9);
  FUN_00601880(auStack_94,(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 4;
  local_c4 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 5;
  if (local_c4 == (uint *)0x0) {
    puVar4 = (uint *)0x0;
  }
  else {
    puVar4 = (uint *)FUN_005fbd20(local_c4,local_b0,0x2b5c,2);
  }
  local_4._0_1_ = 4;
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 6;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar8 = (uint *)0x0;
  }
  else {
    puVar8 = puVar4;
    iVar6 = FUN_005fc0f0((int *)puVar4);
    iVar7 = FUN_005fc0e0((int *)puVar4);
    puVar8 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar8,pHVar10);
  }
  local_4 = CONCAT31(local_4._1_3_,4);
  FUN_005fcc30(puVar4,puVar8,0,0,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(local_a0,puVar8,2,2,0,0,(undefined4 *)0xff,0x10);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)puVar8);
  pvVar11 = SelectObject(pHStack_b4,pvVar11);
  FUN_00601c90(auStack_94,0,(HWND)0x0);
  FUN_00601c60(auStack_94,5);
  FUN_00601b30(auStack_94,5,2);
  FUN_00403e90(auStack_94,0);
  uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8507);
  FUN_006019a0(auStack_94,&uStack_c8);
  FUN_00601b80(auStack_94,pHStack_b4);
  FUN_00601ce0(auStack_94,pHStack_b4);
  FUN_00601c90(auStack_94,0xffffff,(HWND)0x0);
  FUN_00601b30(auStack_94,0x25,0xc3);
  uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8504);
  FUN_006019a0(auStack_94,&uStack_c8);
  FUN_00601b80(auStack_94,pHStack_b4);
  FUN_00601ce0(auStack_94,pHStack_b4);
  SelectObject(pHStack_b4,pvVar11);
  FUN_006075b0(param_1,(int *)puVar8,0x32);
  if (puVar4 != (uint *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  local_c4 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 7;
  if (local_c4 == (uint *)0x0) {
    puVar4 = (uint *)0x0;
  }
  else {
    puVar4 = (uint *)FUN_005fbd20(local_c4,local_b0,0x2b5d,2);
  }
  local_4._0_1_ = 4;
  pvVar5 = (void *)FUN_00618b70(0x28);
  pHVar9 = pHStack_b4;
  local_4._0_1_ = 8;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar8 = (uint *)0x0;
  }
  else {
    puVar8 = puVar4;
    pHVar10 = pHStack_b4;
    iVar6 = FUN_005fc0f0((int *)puVar4);
    iVar7 = FUN_005fc0e0((int *)puVar4);
    puVar8 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar8,pHVar10);
  }
  local_4 = CONCAT31(local_4._1_3_,4);
  FUN_005fcc30(puVar4,puVar8,0,0,0,0,(undefined4 *)0x0,0);
  if (puVar4 != (uint *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  FUN_005fcc30(local_a0,puVar8,2,2,0,0,(undefined4 *)0xff,0x10);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)puVar8);
  uStack_c8 = SelectObject(pHVar9,pvVar11);
  FUN_00601c60(auStack_94,5);
  FUN_00601b30(auStack_94,5,2);
  FUN_00403e90(auStack_94,0);
  uStack_ac = (void *)CONCAT22(DAT_0065d424,0x8507);
  FUN_006019a0(auStack_94,&uStack_ac);
  FUN_00601b80(auStack_94,pHVar9);
  FUN_00601ce0(auStack_94,pHVar9);
  uVar17 = 0x2b71;
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) != 1) {
    uVar17 = 0x2b73;
  }
  local_c4 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 9;
  if (local_c4 == (uint *)0x0) {
    puVar12 = (undefined4 *)0x0;
  }
  else {
    puVar12 = (undefined4 *)FUN_005fbd20(local_c4,local_b0,uVar17,10);
  }
  local_4 = CONCAT31(local_4._1_3_,4);
  FUN_005fcc30(puVar12,puVar8,8,0x41,0,0,(undefined4 *)0x0,0);
  if (puVar12 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar12);
    FUN_00618b60((undefined *)puVar12);
  }
  uVar17 = 0x2b72;
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) != 1) {
    uVar17 = 0x2b74;
  }
  local_c4 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 10;
  if (local_c4 == (uint *)0x0) {
    puVar12 = (undefined4 *)0x0;
  }
  else {
    puVar12 = (undefined4 *)FUN_005fbd20(local_c4,local_b0,uVar17,10);
  }
  local_4 = CONCAT31(local_4._1_3_,4);
  FUN_005fcc30(puVar12,puVar8,0x88,0x41,0,0,(undefined4 *)0x0,0);
  if (puVar12 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar12);
    FUN_00618b60((undefined *)puVar12);
  }
  SelectObject(pHVar9,uStack_c8);
  FUN_00607610(param_1,(int *)puVar8,0x33);
  puVar4 = local_a0;
  if (local_a0 != (uint *)0x0) {
    FUN_005fbfa0(local_a0);
    FUN_00618b60((undefined *)puVar4);
  }
  iVar6 = *(int *)((int)param_1 + 0x114);
  FUN_00606980(param_1,*(int *)(iVar6 + 0xcc),*(int *)(iVar6 + 0xd0),*(int *)(iVar6 + 0xd4),
               *(int *)(iVar6 + 0xd8));
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xb;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
    puVar4 = local_b0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar8 = (uint *)FUN_006073d0(param_1);
    puVar4 = local_b0;
    pvVar5 = FUN_00602150(pvVar5,local_b0,0xf2,3,0xe,0xe,param_1,100,0x277c,0x277d,puVar8,iVar6,
                          iVar7);
  }
  local_4._0_1_ = 4;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar5);
  }
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xc;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar8 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00602150(pvVar5,puVar4,0x65,0xae,0x41,0x12,param_1,0x68,0x296e,0x296f,puVar8,iVar6,
                          iVar7);
  }
  local_4._0_1_ = 4;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8510);
    FUN_00600a40(param_1,pvVar5,&uStack_c8);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar5);
  }
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xd;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar8 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00602150(pvVar5,puVar4,0xaa,0x140,0x40,0x21,param_1,0x65,0x2964,0x2965,puVar8,iVar6
                          ,iVar7);
  }
  local_4._0_1_ = 4;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8503);
    FUN_00600a40(param_1,pvVar5,&uStack_c8);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar5);
  }
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xe;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar8 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00602150(pvVar5,puVar4,0x66,0x140,0x40,0x21,param_1,0x66,0x2962,0x2963,puVar8,iVar6
                          ,iVar7);
  }
  local_4._0_1_ = 4;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8502);
    FUN_00600a40(param_1,pvVar5,&uStack_c8);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar5);
  }
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xf;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar8 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00602150(pvVar5,puVar4,0x21,0x140,0x40,0x21,param_1,0x67,0x2960,0x2961,puVar8,iVar6
                          ,iVar7);
  }
  local_4._0_1_ = 4;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8031);
    FUN_00600a40(param_1,pvVar5,&uStack_c8);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar5);
  }
  pvVar5 = (void *)FUN_00618b70(0x9c);
  local_4._0_1_ = 0x10;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_0060d590(pvVar5,local_bc,7,0x14,0xfd,0x21,0x96,param_1,puVar4);
  }
  local_4._0_1_ = 4;
  uVar16 = (undefined1)local_4;
  local_4._0_1_ = 4;
  *(void **)((int)param_1 + 0x11c) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      uVar17 = 0x2b5f;
      uVar18 = 0x2b60;
    }
    else {
      uVar17 = 0x2b61;
      uVar18 = 0x2b62;
    }
    local_c4 = (uint *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x11;
    if (local_c4 == (uint *)0x0) {
      piVar13 = (int *)0x0;
    }
    else {
      piVar13 = (int *)FUN_00602150(local_c4,local_b0,0,0,0x74,0x21,*(int *)((int)param_1 + 0x11c),
                                    0x97,uVar17,uVar18,
                                    *(uint **)(*(int *)((int)param_1 + 0x11c) + 0x98),0,1);
    }
    local_4._0_1_ = 4;
    if (piVar13 != (int *)0x0) {
      FUN_005ffce0(piVar13,0);
      FUN_0060d700(*(void **)((int)param_1 + 0x11c),piVar13,1,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      uVar18 = 0x2b63;
      uVar17 = 0x2b64;
    }
    else {
      uVar18 = 0x2b65;
      uVar17 = 0x2b66;
    }
    local_c4 = (uint *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x12;
    if (local_c4 == (uint *)0x0) {
      piVar13 = (int *)0x0;
    }
    else {
      piVar13 = (int *)FUN_00602150(local_c4,local_b0,0x82,0,0x74,0x21,
                                    *(int *)((int)param_1 + 0x11c),0x98,uVar18,uVar17,
                                    *(uint **)(*(int *)((int)param_1 + 0x11c) + 0x98),0,1);
    }
    local_4._0_1_ = 4;
    uVar16 = (undefined1)local_4;
    if (piVar13 != (int *)0x0) {
      FUN_005ffce0(piVar13,0);
      FUN_0060d700(*(void **)((int)param_1 + 0x11c),piVar13,0,0);
      uVar16 = (undefined1)local_4;
    }
  }
  local_4._0_1_ = uVar16;
  local_c4 = (uint *)FUN_00618b70(0x10);
  local_4._0_1_ = 0x13;
  if (local_c4 == (uint *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_0060a790(local_c4,1);
  }
  local_4._0_1_ = 4;
  *(void **)((int)param_1 + 0x124) = pvVar5;
  pvVar5 = (void *)FUN_00618b70(0xf8);
  local_4._0_1_ = 0x14;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
    puVar4 = local_b0;
  }
  else {
    uVar25 = 4;
    uVar24 = 0x299a;
    uVar23 = 7;
    uVar22 = 0;
    uVar21 = 0;
    iVar6 = FUN_006073d0(param_1);
    puVar4 = local_b0;
    pvVar5 = FUN_0060bed0(pvVar5,local_b0,(int)param_1,0x23,0x3e,200,0x71,1,
                          *(int **)((int)param_1 + 0x124),iVar6,uVar21,uVar22,uVar23,uVar24,uVar25);
  }
  local_4._0_1_ = 4;
  uVar16 = (undefined1)local_4;
  local_4._0_1_ = 4;
  *(void **)((int)param_1 + 0x128) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    *(uint *)((int)pvVar5 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x128),0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0x98) = 0x111;
    (**(code **)(**(int **)((int)param_1 + 0x128) + 0x18))(4);
    *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0xb4) = 0;
    local_c4 = (uint *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x15;
    if (local_c4 == (uint *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_c4,puVar4,0x2966,2);
    }
    local_4._0_1_ = 4;
    *(void **)(*(int *)((int)param_1 + 0x128) + 0xf4) = pvVar5;
    *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0xa4) = 0x71;
    *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0x94) = 0xffffff;
    FUN_0060ca00(*(void **)((int)param_1 + 0x128),*(int **)((int)param_1 + 0x124));
    uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8520);
    FUN_00600a40(param_1,*(void **)((int)param_1 + 0x128),&uStack_c8);
    uVar16 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar16;
  local_c4 = (uint *)FUN_00618b70(0x10);
  local_4._0_1_ = 0x16;
  if (local_c4 == (uint *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_0060a790(local_c4,4);
  }
  local_4._0_1_ = 4;
  *(void **)((int)param_1 + 300) = pvVar5;
  pvVar5 = (void *)FUN_00618b70(0x118);
  local_4._0_1_ = 0x17;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar20 = 0x3b;
    uVar19 = 0x6b;
    uVar25 = 200;
    puVar8 = (uint *)FUN_006076b0(param_1,0x33);
    pvVar5 = FUN_00607ea0(pvVar5,puVar4,8,0x5d,0x6c,0xd5,param_1,puVar8,uVar25,uVar19,uVar20);
  }
  local_4._0_1_ = 4;
  *(void **)((int)param_1 + 0x130) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    *(uint *)((int)pvVar5 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x130),0);
    iVar6 = *(int *)((int)param_1 + 0x130);
    FUN_0060a490((void *)(iVar6 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar6,7,0x29fc,
                 *(undefined4 *)(iVar6 + 0x3c),0,*(undefined4 *)(iVar6 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x130) + 0xe4) = 0x11;
    (**(code **)(**(int **)((int)param_1 + 0x130) + 0x18))(4);
    *(uint *)(*(int *)((int)param_1 + 0x130) + 0xf4) =
         *(uint *)(*(int *)((int)param_1 + 0x130) + 0xf4) | 0x20000;
    FUN_00608280(*(void **)((int)param_1 + 0x130),*(undefined4 *)((int)param_1 + 300));
  }
  local_c4 = (uint *)FUN_00618b70(0x10);
  local_4._0_1_ = 0x18;
  if (local_c4 == (uint *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_0060a790(local_c4,4);
  }
  local_4._0_1_ = 4;
  *(void **)((int)param_1 + 0x134) = pvVar5;
  pvVar5 = (void *)FUN_00618b70(0x118);
  local_4._0_1_ = 0x19;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar20 = 0x3b;
    uVar19 = 0x6b;
    uVar25 = 0xc9;
    puVar8 = (uint *)FUN_006076b0(param_1,0x33);
    pvVar5 = FUN_00607ea0(pvVar5,puVar4,0x88,0x5d,0x6c,0xd5,param_1,puVar8,uVar25,uVar19,uVar20);
  }
  local_4._0_1_ = 4;
  *(void **)((int)param_1 + 0x138) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    *(uint *)((int)pvVar5 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x138),0);
    iVar6 = *(int *)((int)param_1 + 0x138);
    FUN_0060a490((void *)(iVar6 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar6,7,0x29fc,
                 *(undefined4 *)(iVar6 + 0x3c),0,*(undefined4 *)(iVar6 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x138) + 0xe4) = 0x11;
    (**(code **)(**(int **)((int)param_1 + 0x138) + 0x18))(4);
    *(uint *)(*(int *)((int)param_1 + 0x138) + 0xf4) =
         *(uint *)(*(int *)((int)param_1 + 0x138) + 0xf4) | 0x20000;
    FUN_00608280(*(void **)((int)param_1 + 0x138),*(undefined4 *)((int)param_1 + 0x134));
  }
  pvVar5 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x1a;
  local_c4 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    piVar13 = (int *)0x0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar8 = (uint *)FUN_006076b0(param_1,0x33);
    piVar13 = (int *)FUN_00602150(pvVar5,puVar4,0x78,0x88,0x10,0x10,param_1,0xca,0x2b6d,0x2b6e,
                                  puVar8,iVar6,iVar7);
  }
  local_4._0_1_ = 4;
  if (piVar13 != (int *)0x0) {
    piVar13[0x14] = piVar13[0x14] & 0xefffffff;
    FUN_005ffce0(piVar13,0);
    (**(code **)(*piVar13 + 8))();
    uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8505);
    FUN_00600a40(param_1,piVar13,&uStack_c8);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar13);
  }
  piVar13 = (int *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x1b;
  local_c4 = (uint *)piVar13;
  if (piVar13 == (int *)0x0) {
    piVar13 = (int *)0x0;
  }
  else {
    iVar7 = 0;
    iVar6 = 0;
    puVar8 = (uint *)FUN_006076b0(param_1,0x33);
    piVar13 = (int *)FUN_00602150(piVar13,puVar4,0x78,0xdd,0x10,0x10,param_1,0xcb,0x2b6f,0x2b70,
                                  puVar8,iVar6,iVar7);
  }
  local_4._0_1_ = 4;
  if (piVar13 != (int *)0x0) {
    piVar13[0x14] = piVar13[0x14] & 0xefffffff;
    FUN_005ffce0(piVar13,0);
    (**(code **)(*piVar13 + 8))();
    uStack_c8 = (HGDIOBJ)CONCAT22(DAT_0065d424,0x8506);
    FUN_00600a40(param_1,piVar13,&uStack_c8);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)piVar13);
  }
  FUN_00526730(aiStack_50);
  local_4._0_1_ = 0x1c;
  FUN_004f5380(*(void **)((int)param_1 + 0x120),aiStack_50);
  pvVar5 = (void *)thunk_FUN_005f5060((int)aiStack_50);
  uStack_ac = pvVar5;
  FUN_0042dbe0(&uStack_98);
  local_4._0_1_ = 0x1d;
  while (pvVar5 != (void *)0x0) {
    puVar12 = (undefined4 *)FUN_0042ec10(pvVar5,&local_bc);
    local_4._0_1_ = 0x1e;
    FUN_004f26d0(&uStack_98,puVar12);
    local_4._0_1_ = 0x1d;
    FUN_00619730();
    uStack_c8 = (HGDIOBJ)FUN_0051cab0(&uStack_98);
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      uVar17 = *(uint *)((int)uStack_c8 + 0x30) & 0xfff;
    }
    else {
      uVar17 = (*(uint *)((int)uStack_c8 + 0x30) & 0xfff) + 0x1000;
    }
    local_c4 = (uint *)FUN_00618b70(0x28);
    puVar4 = (uint *)0x0;
    local_4._0_1_ = 0x1f;
    if (local_c4 == (uint *)0x0) {
      puVar8 = (uint *)0x0;
    }
    else {
      puVar8 = (uint *)FUN_005fbd20(local_c4,*(undefined4 *)((int)param_1 + 0x13c),uVar17,2);
    }
    local_4._0_1_ = 0x1d;
    local_c4 = (uint *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x20;
    if (local_c4 == (uint *)0x0) {
      puStack_c0 = (uint *)0x0;
    }
    else {
      puStack_c0 = (uint *)FUN_005fbda0(local_c4,200,0x71,puVar8);
    }
    local_4._0_1_ = 0x1d;
    local_c4 = (uint *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x21;
    if (local_c4 != (uint *)0x0) {
      puVar4 = (uint *)FUN_005fbda0(local_c4,200,0x71,puVar8);
    }
    local_4 = CONCAT31(local_4._1_3_,0x1d);
    iVar6 = FUN_005fc0e0((int *)puVar8);
    iVar7 = (200 - iVar6) / 2;
    iVar6 = FUN_005fc0f0((int *)puVar8);
    iStack_9c = (0x71 - iVar6) / 2;
    uVar16 = (undefined1)puVar8[2];
    iVar6 = FUN_005fc0f0((int *)puStack_c0);
    uVar17 = FUN_005fc0e0((int *)puStack_c0);
    FUN_005fcfe0(puStack_c0,0,0,uVar17,iVar6,uVar16);
    uVar16 = (undefined1)puVar8[2];
    iVar6 = FUN_005fc0f0((int *)puVar4);
    uVar17 = FUN_005fc0e0((int *)puVar4);
    FUN_005fcfe0(puVar4,0,0,uVar17,iVar6,uVar16);
    FUN_005fd0f0(puVar8,puStack_c0,iVar7,iStack_9c);
    FUN_005fd0f0(puVar8,puVar4,iVar7,iStack_9c);
    if (puVar8 != (uint *)0x0) {
      FUN_005fbfa0(puVar8);
      FUN_00618b60((undefined *)puVar8);
    }
    pvVar5 = (void *)FUN_00618b70(0x68);
    local_4._0_1_ = 0x22;
    local_c4 = (uint *)pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      iVar6 = FUN_0046cab0((int)uStack_c8);
      pvVar5 = FUN_0060ba00(pvVar5,iVar6);
    }
    local_4._0_1_ = 0x1d;
    FUN_0060be60(pvVar5,(int)puStack_c0,(int)puVar4);
    pvVar3 = uStack_ac;
    pcVar14 = (char *)FUN_00583c40((int)uStack_ac + 0x24);
    FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar14);
    *(undefined4 *)((int)pvVar5 + 0x58) = 1;
    *(undefined4 *)((int)pvVar5 + 0x60) = 0x808080;
    local_c4 = (uint *)FUN_00618b70(4);
    local_4._0_1_ = 0x23;
    if (local_c4 == (uint *)0x0) {
      uVar25 = 0;
    }
    else {
      uVar25 = FUN_004f26d0(local_c4,&uStack_98);
    }
    *(undefined4 *)((int)pvVar5 + 0x54) = uVar25;
    local_4._0_1_ = 0x1d;
    FUN_005f59f0(*(void **)((int)param_1 + 0x124),(int)pvVar5);
    pvVar5 = *(void **)((int)pvVar3 + 0x10);
    uStack_ac = pvVar5;
  }
  iVar6 = (**(code **)(**(int **)((int)param_1 + 0x124) + 8))();
  if (iVar6 != 0) {
    iVar6 = FUN_0060bd00(iVar6);
    *(undefined1 *)(*(int *)((int)param_1 + 0x128) + 0xac) = *(undefined1 *)(iVar6 + 8);
  }
  FUN_0060ca00(*(void **)((int)param_1 + 0x128),*(int **)((int)param_1 + 0x124));
  (**(code **)(**(int **)((int)param_1 + 0x128) + 4))(5);
  FUN_004ece30(&local_bc);
  puStack_8._0_1_ = 0x24;
  iVar6 = (**(code **)(**(int **)((int)param_1 + 0x120) + 0x28))();
  FUN_004f4390(auStack_64,iVar6);
  puStack_8._0_1_ = 0x25;
  pvVar5 = (void *)thunk_FUN_005f5060((int)auStack_64);
  uVar16 = 0;
  uStack_a8 = uStack_a8 & 0xffffff00;
  pvStack_a4 = pvVar5;
  if (pvVar5 != (void *)0x0) {
    do {
      puVar12 = (undefined4 *)FUN_00403040(pvVar5,&uStack_c8);
      puStack_8._0_1_ = 0x26;
      FUN_004f26d0(&local_bc,puVar12);
      puStack_8._0_1_ = 0x25;
      FUN_00619730();
      local_c4 = (uint *)FUN_004f2d10(*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c),&local_bc);
      if (local_c4 != (uint *)0x0) {
        puStack_c0 = (uint *)FUN_00618b70(0x74);
        puStack_8._0_1_ = 0x27;
        if (puStack_c0 == (uint *)0x0) {
          puStack_c0 = (uint *)0x0;
        }
        else {
          puStack_c0 = (uint *)FUN_004421d0(puStack_c0,&local_bc);
        }
        puStack_8._0_1_ = 0x25;
        puVar4 = FUN_0042c3b0(*(undefined4 *)((int)param_1 + 0x13c),&local_bc,(int *)local_c4,0,1);
        iVar6 = FUN_005fc0e0((int *)puVar4);
        iVar7 = (0x6b - iVar6) / 2;
        iVar6 = FUN_005fc0f0((int *)puVar4);
        iVar6 = (0x3b - iVar6) / 2;
        pvVar5 = (void *)FUN_00618b70(0x28);
        puStack_8._0_1_ = 0x28;
        if (pvVar5 == (void *)0x0) {
          local_b0 = (uint *)0x0;
        }
        else {
          local_b0 = (uint *)FUN_005fbda0(pvVar5,0x6b,0x3b,puVar4);
        }
        puStack_8._0_1_ = 0x25;
        pvVar5 = (void *)FUN_00618b70(0x28);
        puStack_8._0_1_ = 0x29;
        if (pvVar5 == (void *)0x0) {
          puVar8 = (uint *)0x0;
        }
        else {
          puVar8 = (uint *)FUN_005fbda0(pvVar5,0x6b,0x3b,puVar4);
        }
        puStack_8._0_1_ = 0x25;
        uStack_a8 = CONCAT31(uStack_a8._1_3_,(char)puVar4[2]);
        FUN_005fcfe0(local_b0,0,0,0x6b,0x3b,(char)puVar4[2]);
        FUN_005fcfe0(puVar8,0,0,0x6b,0x3b,(char)uStack_a8);
        FUN_005fd0f0(puVar4,local_b0,iVar7,iVar6);
        FUN_005fd0f0(puVar4,puVar8,iVar7,iVar6);
        puVar2 = puStack_c0;
        FUN_0060be60(puStack_c0,(int)local_b0,(int)puVar8);
        puVar15 = FUN_004f62d0((int)local_c4);
        pcVar14 = (char *)FUN_00583c40((int)puVar15);
        FUN_005f35e0(puVar2 + 5,pcVar14);
        iVar7 = FUN_005fc0f0((int *)puVar4);
        puVar2[0xd] = iVar7 + iVar6;
        FUN_005f59f0(*(void **)((int)param_1 + 300),(int)puVar2);
        pvStack_a4 = *(void **)((int)pvStack_a4 + 0x10);
        pvVar5 = pvStack_a4;
        if (puVar4 != (uint *)0x0) {
          FUN_005fbfa0(puVar4);
          FUN_00618b60((undefined *)puVar4);
          pvVar5 = pvStack_a4;
        }
      }
    } while (pvVar5 != (void *)0x0);
    uVar16 = (undefined1)uStack_a8;
  }
  (**(code **)(**(int **)((int)param_1 + 300) + 8))();
  *(undefined1 *)(*(int *)((int)param_1 + 0x130) + 0xd4) = uVar16;
  FUN_0060a280(*(void **)((int)param_1 + 0x130));
  FUN_004f4390(auStack_5c,*(int *)((int)param_1 + 0x120) + 0x58);
  puStack_8._0_1_ = 0x2a;
  uStack_c8 = (HGDIOBJ)thunk_FUN_005f5060((int)auStack_5c);
  if (uStack_c8 != (void *)0x0) {
    do {
      do {
        puVar12 = (undefined4 *)FUN_00403040(uStack_c8,&pHStack_b8);
        puStack_8._0_1_ = 0x2b;
        FUN_004f26d0(&local_bc,puVar12);
        puStack_8._0_1_ = 0x2a;
        FUN_00619730();
        local_c4 = (uint *)FUN_004f2d10(*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c),&local_bc);
      } while (local_c4 == (uint *)0x0);
      puStack_c0 = (uint *)FUN_00618b70(0x74);
      puStack_8._0_1_ = 0x2c;
      if (puStack_c0 == (uint *)0x0) {
        puStack_c0 = (uint *)0x0;
      }
      else {
        puStack_c0 = (uint *)FUN_004421d0(puStack_c0,&local_bc);
      }
      puStack_8._0_1_ = 0x2a;
      puVar4 = FUN_0042c3b0(*(undefined4 *)((int)param_1 + 0x13c),&local_bc,(int *)local_c4,0,1);
      iVar6 = FUN_005fc0e0((int *)puVar4);
      iVar7 = (0x6b - iVar6) / 2;
      iVar6 = FUN_005fc0f0((int *)puVar4);
      iVar6 = (0x3b - iVar6) / 2;
      pvVar5 = (void *)FUN_00618b70(0x28);
      puStack_8._0_1_ = 0x2d;
      if (pvVar5 == (void *)0x0) {
        local_b0 = (uint *)0x0;
      }
      else {
        local_b0 = (uint *)FUN_005fbda0(pvVar5,0x6b,0x3b,puVar4);
      }
      puStack_8._0_1_ = 0x2a;
      pvVar5 = (void *)FUN_00618b70(0x28);
      puStack_8._0_1_ = 0x2e;
      if (pvVar5 == (void *)0x0) {
        puVar8 = (uint *)0x0;
      }
      else {
        puVar8 = (uint *)FUN_005fbda0(pvVar5,0x6b,0x3b,puVar4);
      }
      puStack_8._0_1_ = 0x2a;
      uStack_a8 = CONCAT31(uStack_a8._1_3_,(char)puVar4[2]);
      FUN_005fcfe0(local_b0,0,0,0x6b,0x3b,(char)puVar4[2]);
      FUN_005fcfe0(puVar8,0,0,0x6b,0x3b,(char)uStack_a8);
      FUN_005fd0f0(puVar4,local_b0,iVar7,iVar6);
      FUN_005fd0f0(puVar4,puVar8,iVar7,iVar6);
      puVar2 = puStack_c0;
      FUN_0060be60(puStack_c0,(int)local_b0,(int)puVar8);
      puVar15 = FUN_004f62d0((int)local_c4);
      pcVar14 = (char *)FUN_00583c40((int)puVar15);
      FUN_005f35e0(puVar2 + 5,pcVar14);
      iVar7 = FUN_005fc0f0((int *)puVar4);
      puVar2[0xd] = iVar7 + iVar6;
      FUN_005f59f0(*(void **)((int)param_1 + 0x134),(int)puVar2);
      pvStack_a4 = *(void **)((int)pvStack_a4 + 0x10);
      if (puVar4 != (uint *)0x0) {
        FUN_005fbfa0(puVar4);
        FUN_00618b60((undefined *)puVar4);
      }
    } while( true );
  }
  (**(code **)(**(int **)((int)param_1 + 0x134) + 8))();
  *(undefined1 *)(*(int *)((int)param_1 + 0x138) + 0xd4) = uVar16;
  FUN_0060a280(*(void **)((int)param_1 + 0x138));
  puVar4 = (uint *)FUN_006076b0(param_1,0x32);
  puStack_c0 = puVar4;
  puVar12 = (undefined4 *)FUN_0053ca30(*(void **)((int)param_1 + 0x120),&uStack_c8);
  puStack_8._0_1_ = 0x2f;
  FUN_004f26d0(&local_bc,puVar12);
  puStack_8._0_1_ = 0x2a;
  FUN_00619730();
  puVar8 = (uint *)FUN_004f2d10(*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c),&local_bc);
  local_c4 = puVar8;
  if (puVar8 != (uint *)0x0) {
    local_b0 = (uint *)0x90;
    uStack_ac = (void *)0x98;
    if ((local_bc >> 0x18 < 0x90) || (0x97 < local_bc >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      pvVar5 = (void *)FUN_00618b70(0x28);
      puStack_8._0_1_ = 0x30;
      uStack_c8 = pvVar5;
      if (pvVar5 == (void *)0x0) {
        puVar8 = (uint *)0x0;
        puStack_8._0_1_ = 0x2a;
      }
      else {
        uVar16 = 2;
        iVar6 = FUN_00509610((int)puVar8);
        uVar17 = FUN_0045c970(iVar6);
        puVar8 = (uint *)FUN_005fbd20(pvVar5,pHStack_b4,uVar17,uVar16);
        puStack_8._0_1_ = 0x2a;
      }
    }
    else {
      puVar8 = FUN_0042c3b0(*(undefined4 *)((int)param_1 + 0x13c),&local_bc,(int *)puVar8,1,1);
    }
  }
  if ((puVar8 != (uint *)0x0) && (puVar4 != (uint *)0x0)) {
    pvVar11 = (HGDIOBJ)FUN_005fc130((int)puStack_c0);
    pvVar11 = SelectObject(pHStack_b8,pvVar11);
    iVar6 = FUN_005fc0e0((int *)puVar8);
    iVar7 = FUN_005fc0f0((int *)puVar8);
    FUN_005fd0f0(puVar8,puStack_c0,(0xa5 - iVar6) / 2 + 0x33,(0x4f - iVar7) / 2 + 0xd3);
    FUN_00601c90(&uStack_98,0xffffff,(HWND)0x0);
    FUN_00601b30(&uStack_98,0x25,0x126);
    FUN_00601c60(&uStack_98,5);
    puVar15 = FUN_004f62d0((int)local_c4);
    pcVar14 = (char *)FUN_00583c40((int)puVar15);
    FUN_00601aa0(&uStack_98,pcVar14);
    FUN_00403e90(&uStack_98,1);
    uStack_6c = 1;
    FUN_00601b80(&uStack_98,pHStack_b8);
    uStack_98 = 0xb9;
    FUN_00601ce0(&uStack_98,pHStack_b8);
    SelectObject(pHStack_b8,pvVar11);
  }
  if (puVar8 != (uint *)0x0) {
    FUN_005fbfa0(puVar8);
    FUN_00618b60((undefined *)puVar8);
  }
  DeleteDC(pHStack_b8);
  PostMessageA(*(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18),0x467,
               *(WPARAM *)((int)param_1 + 0x18),0);
  puStack_8._0_1_ = 0x25;
  FUN_004f4380(auStack_5c);
  puStack_8._0_1_ = 0x24;
  FUN_004f4380(auStack_64);
  puStack_8._0_1_ = 0x1d;
  FUN_00619730();
  puStack_8._0_1_ = 0x1c;
  FUN_00619730();
  puStack_8._0_1_ = 4;
  FUN_00526770(&uStack_54);
  puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
  FUN_00601990((int)&uStack_98);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00601990((int)auStack_44);
  ExceptionList = pvStack_10;
  return;
}

