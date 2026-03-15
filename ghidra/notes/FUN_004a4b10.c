
void __fastcall FUN_004a4b10(int *param_1)

{
  int *piVar1;
  undefined4 uVar2;
  uint *puVar3;
  void *pvVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  undefined *puVar8;
  char *pcVar9;
  HPEN pHVar10;
  uint *puVar11;
  uint uVar12;
  int *piVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  uint uVar16;
  uint *puVar17;
  int iVar18;
  HDC pHVar19;
  int iVar20;
  int iVar21;
  undefined4 uStack_38;
  HDC pHStack_34;
  uint *local_30;
  int *local_2c;
  void *pvStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  uint uStack_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637d6a;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar1 = FUN_004a25c0(param_1);
  puVar11 = (uint *)0x0;
  if (piVar1 != (int *)0x0) {
    local_2c = piVar1;
    uVar2 = FUN_006037f0(7);
    local_30 = (uint *)FUN_00618b70(0x28);
    local_4 = 0;
    if (local_30 != (uint *)0x0) {
      puVar11 = (uint *)FUN_005fbd20(local_30,uVar2,0x2a12,2);
    }
    local_4 = 0xffffffff;
    param_1[0x5b] = (uint)piVar1[9] >> 6 & 3;
    puVar3 = FUN_004025b0(piVar1,(uint *)&local_30);
    local_4 = 1;
    FUN_004f26d0(param_1 + 0x51,puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_00607ce0(param_1,2);
    FUN_006071a0(param_1);
    pHStack_34 = GetDC((HWND)param_1[6]);
    param_1[0x5d] = *(int *)(param_1[0x52] + 0x9c);
    param_1[0x5c] = (*(int *)(param_1[0x52] + 0x9c) != 1) + 1;
    local_30 = (uint *)FUN_00618b70(0x10);
    local_4 = 2;
    if (local_30 == (uint *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_0060a790(local_30,4);
    }
    local_4 = 0xffffffff;
    param_1[0x62] = (int)pvVar4;
    local_30 = (uint *)FUN_00618b70(0x10);
    local_4 = 3;
    if (local_30 == (uint *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_0060a790(local_30,4);
    }
    local_4 = 0xffffffff;
    param_1[99] = (int)pvVar4;
    puVar3 = (uint *)FUN_00618b70(0x28);
    local_4 = 4;
    local_30 = puVar3;
    if (puVar3 == (uint *)0x0) {
      uStack_38 = (uint *)0x0;
    }
    else {
      puVar17 = puVar11;
      pHVar19 = pHStack_34;
      iVar5 = FUN_005fc0f0((int *)puVar11);
      iVar6 = FUN_005fc0e0((int *)puVar11);
      uStack_38 = (uint *)FUN_005fbea0(puVar3,iVar6,iVar5,puVar17,pHVar19);
    }
    puVar3 = uStack_38;
    local_4 = 0xffffffff;
    FUN_005fcc30(puVar11,uStack_38,0,0,0,0,(undefined4 *)0x0,0);
    FUN_00607610(param_1,(int *)puVar3,1);
    if (puVar11 != (uint *)0x0) {
      FUN_005fbfa0(puVar11);
      FUN_00618b60((undefined *)puVar11);
    }
    FUN_006075e0(param_1,1);
    local_30 = (uint *)param_1[0x29];
    uVar12 = -(uint)(param_1[0x5c] != 1) & 0x32;
    pvStack_28 = (void *)FUN_00618b70(0x28);
    local_4 = 5;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_005fbd20(pvStack_28,uVar2,uVar12 + 0x28a7,2);
    }
    local_4 = 0xffffffff;
    param_1[0x60] = (int)pvVar4;
    pvStack_28 = (void *)FUN_00618b70(0x28);
    local_4 = 6;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_005fbea0(pvStack_28,0x7d,0x31,(uint *)param_1[0x60],pHStack_34);
    }
    local_4 = 0xffffffff;
    param_1[0x5a] = (int)pvVar4;
    pvStack_28 = (void *)FUN_00618b70(0x28);
    local_4 = 7;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_005fbd20(pvStack_28,uVar2,0x28d9 - uVar12,2);
    }
    local_4 = 0xffffffff;
    param_1[0x61] = (int)pvVar4;
    pvStack_28 = (void *)FUN_00618b70(0x9c);
    local_4 = 8;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_0060d590(pvStack_28,param_1[0x6c],99,0x60,0x83,0x1c,0x6a,param_1,uStack_38);
    }
    param_1[0x56] = (int)pvVar4;
    *(uint *)((int)pvVar4 + 0x50) = *(uint *)((int)pvVar4 + 0x50) & 0xefffffff;
    local_4 = 0xffffffff;
    FUN_005ffce0((void *)param_1[0x56],0);
    pvStack_28 = (void *)FUN_00618b70(0x9c);
    local_4 = 9;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_0060d590(pvStack_28,param_1[0x6c],99,0x60,0x83,0x1c,0x6b,param_1,uStack_38);
    }
    param_1[0x57] = (int)pvVar4;
    local_4 = 0xffffffff;
    *(uint *)((int)pvVar4 + 0x50) = *(uint *)(param_1[0x56] + 0x50) & 0xefffffff;
    FUN_005ffce0((void *)param_1[0x57],0);
    pvStack_28 = (void *)FUN_00618b70(0x34);
    local_4 = 10;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_00601880(pvStack_28,(char *)0x0,100,0x1d,0x80,0x19,0,0x13);
    }
    local_4 = 0xffffffff;
    param_1[100] = (int)pvVar4;
    FUN_00601c60(pvVar4,5);
    FUN_00601c90((void *)param_1[100],0xffffff,(HWND)0x0);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0xb;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,1,0,0x1f,0x1c,param_1[0x56],0x66,uVar12 + 0x28a9
                                   ,uVar12 + 0x28b0,*(uint **)(param_1[0x56] + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,uVar12 + 0x28b0);
    FUN_005ffce0(piVar1,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3823);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    FUN_0060d700((void *)param_1[0x56],piVar1,0,0);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0xc;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,1,0,0x1f,0x1c,param_1[0x57],0x66,0x28db - uVar12
                                   ,0x28e2 - uVar12,*(uint **)(param_1[0x57] + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,0x28e2 - uVar12);
    FUN_005ffce0(piVar1,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3823);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    FUN_0060d700((void *)param_1[0x57],piVar1,0,0);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0xd;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,0x21,0,0x1f,0x1c,param_1[0x56],0x67,
                                   uVar12 + 0x28aa,uVar12 + 0x28b1,*(uint **)(param_1[0x56] + 0x98),
                                   0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,uVar12 + 0x28b1);
    FUN_00603150(piVar1,2,uVar12 + 0x28ad);
    FUN_005ffce0(piVar1,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3820);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    FUN_0060d700((void *)param_1[0x56],piVar1,0,0);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0xe;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,0x21,0,0x1f,0x1c,param_1[0x57],0x67,
                                   0x28dc - uVar12,0x28e3 - uVar12,*(uint **)(param_1[0x57] + 0x98),
                                   0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,0x28e3 - uVar12);
    FUN_00603150(piVar1,2,0x28df - uVar12);
    FUN_005ffce0(piVar1,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3820);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    FUN_0060d700((void *)param_1[0x57],piVar1,0,0);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0xf;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,0x41,0,0x1f,0x1c,param_1[0x56],0x68,
                                   uVar12 + 0x28ab,uVar12 + 0x28b2,*(uint **)(param_1[0x56] + 0x98),
                                   0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,uVar12 + 0x28b2);
    FUN_00603150(piVar1,2,uVar12 + 0x28ae);
    FUN_005ffce0(piVar1,0);
    FUN_0060d700((void *)param_1[0x56],piVar1,0,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3821);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0x10;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,0x41,0,0x1f,0x1c,param_1[0x57],0x68,
                                   0x28dd - uVar12,0x28e4 - uVar12,*(uint **)(param_1[0x57] + 0x98),
                                   0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,0x28e4 - uVar12);
    FUN_00603150(piVar1,2,0x28e0 - uVar12);
    FUN_005ffce0(piVar1,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3821);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    FUN_0060d700((void *)param_1[0x57],piVar1,0,0);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0x11;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,0x61,0,0x1f,0x1c,param_1[0x56],0x69,
                                   uVar12 + 0x28ac,uVar12 + 0x28b3,*(uint **)(param_1[0x56] + 0x98),
                                   0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,uVar12 + 0x28b3);
    FUN_00603150(piVar1,2,uVar12 + 0x28af);
    FUN_005ffce0(piVar1,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3822);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    FUN_0060d700((void *)param_1[0x56],piVar1,0,0);
    pvStack_28 = (void *)FUN_00618b70(0x114);
    local_4 = 0x12;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_00602150(pvStack_28,uVar2,0x61,0,0x1f,0x1c,param_1[0x57],0x69,
                                   0x28de - uVar12,0x28e5 - uVar12,*(uint **)(param_1[0x57] + 0x98),
                                   0,0);
    }
    local_4 = 0xffffffff;
    FUN_00603150(piVar1,4,0x28e5 - uVar12);
    FUN_00603150(piVar1,2,0x28e1 - uVar12);
    FUN_005ffce0(piVar1,0);
    uStack_38 = (uint *)CONCAT22(DAT_0065d424,0x3822);
    FUN_00600a40(param_1,piVar1,&uStack_38);
    FUN_0060d700((void *)param_1[0x57],piVar1,0,0);
    pvStack_28 = (void *)FUN_00618b70(0x28);
    local_4 = 0x13;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_005fbd20(pvStack_28,uVar2,0x27e1,2);
    }
    local_4 = 0xffffffff;
    iVar5 = 2;
    if (piVar1 != (int *)0x0) {
      pvVar4 = (void *)FUN_00618b70(0x114);
      local_4 = 0x14;
      pvStack_28 = pvVar4;
      if (pvVar4 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        puVar11 = (uint *)param_1[0x29];
        iVar20 = 1;
        iVar18 = 0;
        uVar16 = 0x27e0;
        uVar12 = 0x27e1;
        uVar14 = 0xca;
        piVar13 = param_1;
        iVar6 = FUN_005fc0f0(piVar1);
        iVar7 = FUN_005fc0e0(piVar1);
        pvVar4 = FUN_00602150(pvVar4,uVar2,3,3,iVar7,iVar6,piVar13,uVar14,uVar12,uVar16,puVar11,
                              iVar18,iVar20);
      }
      local_4 = 0xffffffff;
      if (pvVar4 != (void *)0x0) {
        FUN_006030c0(pvVar4,0x80);
        FUN_005ffce0(pvVar4,0);
        FUN_005f4f10(param_1 + 0x54,(int)pvVar4);
        iVar5 = FUN_005fc0e0(piVar1);
        iVar5 = iVar5 + 5;
      }
      FUN_005fbfa0(piVar1);
      FUN_00618b60((undefined *)piVar1);
    }
    *(undefined4 *)(param_1[0x49] + 4) = 0x10;
    FUN_00601b30((void *)param_1[0x49],iVar5,2);
    puVar8 = FUN_004f62d0((int)local_2c);
    pcVar9 = (char *)FUN_00583c40((int)puVar8);
    FUN_00601aa0((void *)param_1[0x49],pcVar9);
    FUN_00601c60((void *)param_1[0x49],5);
    FUN_00403e90((void *)param_1[0x49],0x24);
    FUN_00601c90((void *)param_1[0x49],0x2000000,(HWND)0x0);
    FUN_00601b80((void *)param_1[0x49],pHStack_34);
    iVar5 = param_1[0x49];
    uStack_24 = *(undefined4 *)(iVar5 + 0x14);
    uStack_1c = *(undefined4 *)(iVar5 + 0x1c);
    param_1[0x4d] = (param_1[0x4b] + *(int *)(iVar5 + 0x20)) - *(int *)(iVar5 + 0x18);
    iVar5 = FUN_006002b0(10);
    param_1[0x69] = iVar5;
    iVar5 = FUN_006002b0(10);
    param_1[0x6a] = iVar5;
    pHVar10 = CreatePen(2,1,0x2ffffff);
    param_1[0x68] = (int)pHVar10;
    pvStack_28 = (void *)FUN_00618b70(0x28);
    local_4 = 0x15;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_005fbd20(pvStack_28,uVar2,0x277c,2);
    }
    local_4 = 0xffffffff;
    local_2c = (int *)FUN_00618b70(0x114);
    local_4 = 0x16;
    if (local_2c == (int *)0x0) {
      uStack_38 = (uint *)0x0;
    }
    else {
      puVar11 = (uint *)param_1[0x29];
      iVar5 = param_1[0xc];
      iVar21 = 1;
      iVar20 = 0;
      uVar16 = 0x277d;
      uVar12 = 0x277c;
      uVar15 = 0x65;
      piVar13 = param_1;
      iVar6 = FUN_005fc0f0(piVar1);
      iVar7 = FUN_005fc0e0(piVar1);
      uVar14 = 3;
      iVar18 = FUN_005fc0e0(piVar1);
      uStack_38 = (uint *)FUN_00602150(local_2c,uVar2,(iVar5 - iVar18) + -3,uVar14,iVar7,iVar6,
                                       piVar13,uVar15,uVar12,uVar16,puVar11,iVar20,iVar21);
    }
    pvVar4 = uStack_38;
    local_4 = 0xffffffff;
    if (uStack_38 != (uint *)0x0) {
      FUN_006030c0(uStack_38,0x80);
      FUN_005ffce0(pvVar4,0);
      FUN_005f4f10(param_1 + 0x54,(int)pvVar4);
    }
    if (piVar1 != (int *)0x0) {
      FUN_005fbfa0(piVar1);
      FUN_00618b60((undefined *)piVar1);
    }
    pvStack_28 = (void *)FUN_00618b70(0x28);
    local_4 = 0x17;
    if (pvStack_28 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)FUN_005fbd20(pvStack_28,uVar2,0x280d,2);
    }
    local_4 = 0xffffffff;
    if ((piVar1 != (int *)0x0) && (uStack_38 != (uint *)0x0)) {
      local_2c = (int *)FUN_00618b70(0x114);
      local_4 = 0x18;
      if (local_2c == (int *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        puVar11 = (uint *)param_1[0x29];
        iVar21 = 1;
        iVar20 = 0;
        iVar5 = *(int *)((int)uStack_38 + 0x28);
        uVar16 = 0x280e;
        uVar12 = 0x280d;
        uVar15 = 100;
        piVar13 = param_1;
        iVar6 = FUN_005fc0f0(piVar1);
        iVar7 = FUN_005fc0e0(piVar1);
        uVar14 = 3;
        iVar18 = FUN_005fc0e0(piVar1);
        pvVar4 = FUN_00602150(local_2c,uVar2,iVar5 - iVar18,uVar14,iVar7,iVar6,piVar13,uVar15,uVar12
                              ,uVar16,puVar11,iVar20,iVar21);
      }
      local_4 = 0xffffffff;
      FUN_006030c0(pvVar4,0x80);
      FUN_005ffce0(pvVar4,0);
      FUN_005f4f10(param_1 + 0x54,(int)pvVar4);
      FUN_005fbfa0(piVar1);
      FUN_00618b60((undefined *)piVar1);
    }
    if (param_1[0x5c] == 1) {
      pvStack_28 = (void *)FUN_00618b70(0x28);
      local_4 = 0x19;
      if (pvStack_28 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_005fbd20(pvStack_28,uVar2,0x27d9,10);
      }
      local_4 = 0xffffffff;
      param_1[0x5e] = (int)pvVar4;
      pvStack_28 = (void *)FUN_00618b70(0x28);
      local_4 = 0x1a;
      if (pvStack_28 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_005fbd20(pvStack_28,uVar2,0x283e,10);
      }
      param_1[0x5f] = (int)pvVar4;
      param_1[0x5b] = 2;
    }
    else {
      param_1[0x5b] = 1;
      pvStack_28 = (void *)FUN_00618b70(0x28);
      local_4 = 0x1b;
      if (pvStack_28 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_005fbd20(pvStack_28,uVar2,0x283b,10);
      }
      local_4 = 0xffffffff;
      param_1[0x5e] = (int)pvVar4;
      pvStack_28 = (void *)FUN_00618b70(0x28);
      local_4 = 0x1c;
      if (pvStack_28 == (void *)0x0) {
        param_1[0x5f] = 0;
      }
      else {
        pvVar4 = FUN_005fbd20(pvStack_28,uVar2,0x27d8,10);
        param_1[0x5f] = (int)pvVar4;
      }
    }
    local_4 = 0xffffffff;
    pvStack_28 = (void *)FUN_00618b70(0x118);
    local_4 = 0x1d;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_00607ea0(pvStack_28,uVar2,4,0x1d,0x5b,0x10a,param_1,local_30,0xc9,0x5b,0x32);
    }
    local_4 = 0xffffffff;
    param_1[0x58] = (int)pvVar4;
    if (pvVar4 != (void *)0x0) {
      FUN_00609290(pvVar4,*(HWND *)(param_1[0x52] + 0x18));
      FUN_005ffce0((void *)param_1[0x58],0);
      FUN_00609940((void *)param_1[0x58],0xffffff);
      FUN_00609950((void *)param_1[0x58],0xffffff);
      FUN_00608280((void *)param_1[0x58],param_1[0x62]);
      FUN_0060a280((void *)param_1[0x58]);
      (**(code **)(*(int *)param_1[0x58] + 0x18))(10);
      *(undefined4 *)(param_1[0x58] + 0xe4) = 0x20;
      *(undefined4 *)(param_1[0x58] + 0xe8) = 8;
      *(undefined4 *)(param_1[0x58] + 0xec) = 3;
      FUN_0060a490((void *)(param_1[0x58] + 0x94),uVar2,param_1[0x58],7,0x29cb,0x10a,0,0x10a,0,
                   (uint *)0x0);
    }
    pvStack_28 = (void *)FUN_00618b70(0x118);
    local_4 = 0x1e;
    if (pvStack_28 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_00607ea0(pvStack_28,uVar2,0x65,0x7f,0x85,0xa4,param_1,local_30,200,0x7d,0x32);
    }
    local_4 = 0xffffffff;
    param_1[0x59] = (int)pvVar4;
    if (pvVar4 != (void *)0x0) {
      FUN_00609290(pvVar4,*(HWND *)(param_1[0x52] + 0x18));
      FUN_005ffce0((void *)param_1[0x59],0);
      FUN_00609940((void *)param_1[0x59],0xffffff);
      FUN_00609950((void *)param_1[0x59],0xffffff);
      FUN_00608280((void *)param_1[0x59],param_1[99]);
      (**(code **)(*(int *)param_1[0x59] + 0x18))(10);
      *(undefined4 *)(param_1[0x59] + 0xe4) = 0x29;
      FUN_0060a490((void *)(param_1[0x59] + 0x94),uVar2,param_1[0x59],7,0x29cb,0xa4,0,0xa4,0,
                   (uint *)0x0);
    }
    FUN_004a3340(param_1);
    ReleaseDC((HWND)param_1[6],pHStack_34);
    if ((*(byte *)(param_1[0x52] + 0xc0) & 4) == 0) {
      FUN_00610b20(&uStack_24,0,0,0,0,0,0);
      uStack_14 = uStack_14 & 0xfffffff9 | 1;
      local_4 = 0x1f;
      uStack_1c = 7;
      uStack_20 = 0x25e;
      FUN_00610c30((int)&uStack_24);
    }
  }
  ExceptionList = local_c;
  return;
}

