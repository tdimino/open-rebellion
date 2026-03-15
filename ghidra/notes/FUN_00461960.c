
void __fastcall FUN_00461960(void *param_1)

{
  undefined4 uVar1;
  HDC pHVar2;
  HDC hdc;
  void *pvVar3;
  int iVar4;
  int iVar5;
  uint *puVar6;
  HGDIOBJ pvVar7;
  undefined4 *this;
  int *piVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  uint uVar11;
  uint uVar12;
  undefined4 uVar13;
  int iVar14;
  undefined *puStack_30;
  uint *local_28;
  undefined4 *local_24;
  undefined4 *local_20;
  undefined4 *local_1c;
  undefined2 uStack_18;
  undefined2 uStack_16;
  undefined4 uStack_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631363;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  iVar4 = *(int *)((int)param_1 + 0x158);
  FUN_00606980(param_1,*(int *)(iVar4 + 0xcc),*(int *)(iVar4 + 0xd0),*(int *)(iVar4 + 0xd4),
               *(int *)(iVar4 + 0xd8));
  iVar4 = *(int *)(*(int *)((int)param_1 + 0x158) + 0x9c);
  if (iVar4 == 1) {
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (local_10 == (void *)0x0) {
      local_20 = (undefined4 *)0x0;
    }
    else {
      local_20 = (undefined4 *)FUN_005fbd20(local_10,uVar1,0x291e,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (local_10 == (void *)0x0) {
      local_1c = (undefined4 *)0x0;
    }
    else {
      local_1c = (undefined4 *)FUN_005fbd20(local_10,uVar1,0x291c,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (local_10 == (void *)0x0) {
      local_28 = (uint *)0x0;
    }
    else {
      local_28 = (uint *)FUN_005fbd20(local_10,uVar1,0x285f,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (local_10 == (void *)0x0) {
LAB_00461b89:
      local_4 = 0xffffffff;
      local_24 = (undefined4 *)0x0;
    }
    else {
      local_24 = (undefined4 *)FUN_005fbd20(local_10,uVar1,0x295a,10);
      local_4 = 0xffffffff;
    }
  }
  else if (iVar4 == 2) {
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 4;
    if (local_10 == (void *)0x0) {
      local_20 = (undefined4 *)0x0;
    }
    else {
      local_20 = (undefined4 *)FUN_005fbd20(local_10,uVar1,0x291f,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 5;
    if (local_10 == (void *)0x0) {
      local_1c = (undefined4 *)0x0;
    }
    else {
      local_1c = (undefined4 *)FUN_005fbd20(local_10,uVar1,0x291d,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 6;
    if (local_10 == (void *)0x0) {
      local_28 = (uint *)0x0;
    }
    else {
      local_28 = (uint *)FUN_005fbd20(local_10,uVar1,0x2860,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 7;
    if (local_10 == (void *)0x0) goto LAB_00461b89;
    local_24 = (undefined4 *)FUN_005fbd20(local_10,uVar1,0x295e,10);
    local_4 = 0xffffffff;
  }
  pHVar2 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar2);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar2);
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 8;
  local_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    puVar6 = (uint *)0x0;
  }
  else {
    puVar6 = local_28;
    pHVar2 = hdc;
    iVar4 = FUN_005fc0f0((int *)local_28);
    iVar5 = FUN_005fc0e0((int *)local_28);
    puVar6 = (uint *)FUN_005fbea0(pvVar3,iVar5,iVar4,puVar6,pHVar2);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(local_20,puVar6,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_28,puVar6,0,0);
  FUN_005fcc30(local_24,puVar6,0x19c,0,0,0,(undefined4 *)0x0,0);
  pvVar7 = (HGDIOBJ)FUN_005fc130((int)puVar6);
  local_10 = SelectObject(hdc,pvVar7);
  uStack_14 = (void *)FUN_00618b70(0x34);
  local_4 = 9;
  if (uStack_14 == (void *)0x0) {
    this = (undefined4 *)0x0;
  }
  else {
    uStack_16 = DAT_0065d424;
    uStack_18 = 0x1880;
    this = (undefined4 *)FUN_00601700(uStack_14,&uStack_18,0x24,0xe,0,0,0,0);
  }
  local_4 = 0xffffffff;
  FUN_00601c60(this,5);
  FUN_00601c90(this,0x2ffffff,(HWND)0x0);
  *this = 0x15e;
  this[0xb] = 1;
  FUN_00403e90(this,1);
  FUN_00601ce0(this,hdc);
  uStack_14 = (void *)FUN_00618b70(0x34);
  local_4 = 10;
  if (uStack_14 == (void *)0x0) {
    puStack_30 = (undefined *)0x0;
  }
  else {
    uStack_16 = DAT_0065d424;
    uStack_18 = 0x1881;
    puStack_30 = (undefined *)FUN_00601700(uStack_14,&uStack_18,0x24,0x30,0,0,0,0);
  }
  local_4 = 0xffffffff;
  FUN_00601c60(puStack_30,4);
  FUN_00601c90(puStack_30,0x2ffffff,(HWND)0x0);
  FUN_00601ce0(puStack_30,hdc);
  FUN_00607610(param_1,(int *)puVar6,1);
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 0xb;
  uStack_14 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    puVar6 = (uint *)0x0;
  }
  else {
    puVar6 = local_28;
    pHVar2 = hdc;
    iVar4 = FUN_005fc0f0((int *)local_28);
    iVar5 = FUN_005fc0e0((int *)local_28);
    puVar6 = (uint *)FUN_005fbea0(pvVar3,iVar5,iVar4,puVar6,pHVar2);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(local_1c,puVar6,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_28,puVar6,0,0);
  FUN_005fcc30(local_24,puVar6,0x19c,0,0,0,(undefined4 *)0x0,0);
  pvVar7 = (HGDIOBJ)FUN_005fc130((int)puVar6);
  SelectObject(hdc,pvVar7);
  uStack_14 = (void *)CONCAT22(DAT_0065d424,0x1885);
  FUN_006019a0(this,&uStack_14);
  *this = 0x15e;
  this[0xb] = 1;
  FUN_00403e90(this,1);
  FUN_00601ce0(this,hdc);
  uStack_14 = (void *)CONCAT22(DAT_0065d424,0x1886);
  FUN_006019a0(puStack_30,&uStack_14);
  FUN_00601ce0(puStack_30,hdc);
  FUN_00607610(param_1,(int *)puVar6,2);
  FUN_006075e0(param_1,1);
  SelectObject(hdc,local_10);
  DeleteDC(hdc);
  if (local_20 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_20);
    FUN_00618b60((undefined *)local_20);
  }
  if (local_1c != (undefined4 *)0x0) {
    FUN_005fbfa0(local_1c);
    FUN_00618b60((undefined *)local_1c);
  }
  if (local_28 != (uint *)0x0) {
    FUN_005fbfa0(local_28);
    FUN_00618b60((undefined *)local_28);
  }
  if (local_24 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_24);
    FUN_00618b60((undefined *)local_24);
  }
  if (this != (undefined4 *)0x0) {
    FUN_00601990((int)this);
    FUN_00618b60((undefined *)this);
  }
  if (puStack_30 != (undefined *)0x0) {
    FUN_00601990((int)puStack_30);
    FUN_00618b60(puStack_30);
  }
  pvVar3 = (void *)FUN_00618b70(0x9c);
  local_4 = 0xc;
  local_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar9 = *(undefined4 *)((int)param_1 + 0x1c);
    puVar6 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_0060d590(pvVar3,uVar9,0x24,0x4e,0x99,0x29,100,param_1,puVar6);
  }
  *(void **)((int)param_1 + 0x14c) = pvVar3;
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_005ffce0(pvVar3,0);
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0xd;
    if (local_10 == (void *)0x0) {
      piVar8 = (int *)0x0;
    }
    else {
      piVar8 = (int *)FUN_00602150(local_10,uVar1,0,0,0x31,0x29,*(int *)((int)param_1 + 0x14c),1,
                                   0x2904,0x2905,*(uint **)(*(int *)((int)param_1 + 0x14c) + 0x98),0
                                   ,0);
    }
    local_4 = 0xffffffff;
    if (piVar8 != (int *)0x0) {
      FUN_005ffce0(piVar8,0);
      uStack_14 = (void *)CONCAT22(DAT_0065d424,0x1882);
      FUN_00600a40(param_1,piVar8,&uStack_14);
      FUN_0060d700(*(void **)((int)param_1 + 0x14c),piVar8,1,0);
    }
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0xe;
    if (local_10 == (void *)0x0) {
      piVar8 = (int *)0x0;
    }
    else {
      piVar8 = (int *)FUN_00602150(local_10,uVar1,0x34,0,0x31,0x29,*(int *)((int)param_1 + 0x14c),2,
                                   0x2906,0x2907,*(uint **)(*(int *)((int)param_1 + 0x14c) + 0x98),0
                                   ,0);
    }
    local_4 = 0xffffffff;
    if (piVar8 != (int *)0x0) {
      FUN_005ffce0(piVar8,0);
      uStack_14 = (void *)CONCAT22(DAT_0065d424,0x1883);
      FUN_00600a40(param_1,piVar8,&uStack_14);
      FUN_0060d700(*(void **)((int)param_1 + 0x14c),piVar8,0,0);
    }
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0xf;
    if (local_10 == (void *)0x0) {
      piVar8 = (int *)0x0;
    }
    else {
      piVar8 = (int *)FUN_00602150(local_10,uVar1,0x68,0,0x31,0x29,*(int *)((int)param_1 + 0x14c),3,
                                   0x2909,0x290a,*(uint **)(*(int *)((int)param_1 + 0x14c) + 0x98),0
                                   ,0);
    }
    local_4 = 0xffffffff;
    if (piVar8 != (int *)0x0) {
      FUN_005ffce0(piVar8,0);
      uStack_14 = (void *)CONCAT22(DAT_0065d424,0x1884);
      FUN_00600a40(param_1,piVar8,&uStack_14);
      FUN_0060d700(*(void **)((int)param_1 + 0x14c),piVar8,0,0);
    }
  }
  if (*(int *)(*(int *)((int)param_1 + 0x158) + 0x9c) == 1) {
    FUN_00606c90(param_1,0x1a7,0x19,0x2912,0x2913,200,0x1954,0,0,0,0,0,0);
    uVar12 = 0x2917;
    uVar11 = 0x2916;
    uVar10 = 0x5d;
    uVar9 = 0x1a7;
  }
  else {
    FUN_00606c90(param_1,0x1aa,0x15,0x2914,0x2915,200,0x1954,0,0,0,0,0,0);
    uVar12 = 0x2919;
    uVar11 = 0x2918;
    uVar10 = 0x59;
    uVar9 = 0x1aa;
  }
  FUN_00606c90(param_1,uVar9,uVar10,uVar11,uVar12,0xc9,0x1953,0,0,0,0,0,0);
  pvVar3 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),200);
  if (pvVar3 != (void *)0x0) {
    FUN_00602840(pvVar3,0x260);
  }
  pvVar3 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),0xc9);
  if (pvVar3 != (void *)0x0) {
    FUN_00602840(pvVar3,0x260);
  }
  pvVar3 = (void *)FUN_00618b70(0xf4);
  local_4 = 0x10;
  local_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar9 = *(undefined4 *)((int)param_1 + 0x1c);
    uVar11 = 0;
    puVar6 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_00604cf0(pvVar3,uVar9,0x8f,0x2d,0xfa,0x10,(int)param_1,puVar6,uVar11);
  }
  *(void **)((int)param_1 + 0x150) = pvVar3;
  *(undefined4 *)((int)pvVar3 + 0x24) = 0xcd;
  local_4 = 0xffffffff;
  *(undefined4 *)(*(int *)((int)param_1 + 0x150) + 0xe4) = 0xffffff;
  FUN_005ffce0(*(void **)((int)param_1 + 0x150),0);
  FUN_00605070(*(void **)((int)param_1 + 0x150));
  SetCaretPos(0,0);
  local_10 = (void *)FUN_00618b70(0x34);
  local_4 = 0x11;
  if (local_10 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00601880(local_10,(char *)0x0,0,0,0,0,0,0);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x15c) = pvVar3;
  if (pvVar3 != (void *)0x0) {
    FUN_00601b30(pvVar3,0x28,0x77);
    **(undefined4 **)((int)param_1 + 0x15c) = 0x11b;
    *(undefined4 *)(*(int *)((int)param_1 + 0x15c) + 4) = 0x10;
    *(undefined4 *)(*(int *)((int)param_1 + 0x15c) + 0x2c) = 3;
    FUN_00403e90(*(void **)((int)param_1 + 0x15c),0);
    FUN_00601c60(*(void **)((int)param_1 + 0x15c),5);
    FUN_00601c90(*(void **)((int)param_1 + 0x15c),0x2ffffff,(HWND)0x0);
    pHVar2 = GetDC(*(HWND *)((int)param_1 + 0x18));
    FUN_00601b80(*(void **)((int)param_1 + 0x15c),pHVar2);
    ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar2);
  }
  pvVar3 = (void *)FUN_00618b70(0x118);
  local_4 = 0x12;
  local_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar13 = 0x14;
    uVar10 = 0x15e;
    uVar9 = 0xcc;
    puVar6 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_00607ea0(pvVar3,uVar1,0x24,0x8a,0x15e,0xa5,param_1,puVar6,uVar9,uVar10,uVar13);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x154) = pvVar3;
  if (pvVar3 != (void *)0x0) {
    *(undefined4 *)((int)pvVar3 + 0xf4) = 0x30000;
    FUN_005ffce0(*(void **)((int)param_1 + 0x154),0);
    FUN_00609340(*(void **)((int)param_1 + 0x154),2);
    *(undefined4 *)(*(int *)((int)param_1 + 0x154) + 0xe4) = 0x24;
    iVar4 = *(int *)((int)param_1 + 0x154);
    FUN_0060a490((void *)(iVar4 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar4,7,0x299d,
                 *(undefined4 *)(iVar4 + 0x3c),0,*(undefined4 *)(iVar4 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x154) + 0xe8) = 5;
    FUN_00608280(*(void **)((int)param_1 + 0x154),*(undefined4 *)((int)param_1 + 0x168));
  }
  if (*(int *)(*(int *)((int)param_1 + 0x158) + 0x9c) == 1) {
    pvVar3 = (void *)FUN_00618b70(0x9c);
    local_4 = 0x13;
    local_10 = pvVar3;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      uVar9 = *(undefined4 *)((int)param_1 + 0x1c);
      puVar6 = (uint *)FUN_006073d0(param_1);
      pvVar3 = FUN_0060d590(pvVar3,uVar9,0x1a7,0x93,0x20,0x55,0xfa,param_1,puVar6);
    }
    *(void **)((int)param_1 + 0x144) = pvVar3;
  }
  else {
    pvVar3 = (void *)FUN_00618b70(0x9c);
    local_4 = 0x14;
    local_10 = pvVar3;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      uVar9 = *(undefined4 *)((int)param_1 + 0x1c);
      puVar6 = (uint *)FUN_006073d0(param_1);
      pvVar3 = FUN_0060d590(pvVar3,uVar9,0x1aa,0x8f,0x2c,0x88,0xfa,param_1,puVar6);
    }
    *(void **)((int)param_1 + 0x144) = pvVar3;
  }
  local_4 = 0xffffffff;
  if (*(void **)((int)param_1 + 0x144) == (void *)0x0) goto LAB_00462724;
  FUN_005ffce0(*(void **)((int)param_1 + 0x144),0);
  if (*(int *)(*(int *)((int)param_1 + 0x158) + 0x9c) == 1) {
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x15;
    if (local_10 == (void *)0x0) goto LAB_004625e8;
    iVar4 = *(int *)((int)param_1 + 0x144);
    puVar6 = *(uint **)(iVar4 + 0x98);
    uVar12 = 0x2923;
    uVar11 = 0x2922;
    iVar14 = 0x1f;
    iVar5 = 0x20;
LAB_004625d6:
    piVar8 = (int *)FUN_00602150(local_10,uVar1,0,0,iVar5,iVar14,iVar4,2,uVar11,uVar12,puVar6,0,0);
  }
  else {
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x16;
    if (local_10 != (void *)0x0) {
      iVar4 = *(int *)((int)param_1 + 0x144);
      puVar6 = *(uint **)(iVar4 + 0x98);
      uVar12 = 0x2927;
      uVar11 = 0x2926;
      iVar14 = 0x29;
      iVar5 = 0x2c;
      goto LAB_004625d6;
    }
LAB_004625e8:
    piVar8 = (int *)0x0;
  }
  local_4 = 0xffffffff;
  if (piVar8 != (int *)0x0) {
    FUN_005ffce0(piVar8,0);
    FUN_00602840(piVar8,0x260);
    uStack_14 = (void *)CONCAT22(DAT_0065d424,0x1885);
    FUN_00600a40(param_1,piVar8,&uStack_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar8,1,0);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x158) + 0x9c) == 1) {
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x17;
    if (local_10 == (void *)0x0) goto LAB_004626d4;
    iVar4 = *(int *)((int)param_1 + 0x144);
    puVar6 = *(uint **)(iVar4 + 0x98);
    uVar12 = 0x2921;
    uVar11 = 0x2920;
    iVar14 = 0x1f;
    iVar5 = 0x20;
LAB_004626c2:
    piVar8 = (int *)FUN_00602150(local_10,uVar1,0,0x36,iVar5,iVar14,iVar4,1,uVar11,uVar12,puVar6,0,0
                                );
  }
  else {
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x18;
    if (local_10 != (void *)0x0) {
      iVar4 = *(int *)((int)param_1 + 0x144);
      puVar6 = *(uint **)(iVar4 + 0x98);
      uVar12 = 0x2925;
      uVar11 = 0x2924;
      iVar14 = 0x29;
      iVar5 = 0x2c;
      goto LAB_004626c2;
    }
LAB_004626d4:
    piVar8 = (int *)0x0;
  }
  local_4 = 0xffffffff;
  if (piVar8 != (int *)0x0) {
    FUN_005ffce0(piVar8,0);
    FUN_00602840(piVar8,0x260);
    uStack_14 = (void *)CONCAT22(DAT_0065d424,0x1880);
    FUN_00600a40(param_1,piVar8,&uStack_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar8,0,0);
  }
LAB_00462724:
  FUN_0060d7e0(*(void **)((int)param_1 + 0x144),1,0);
  *(undefined4 *)((int)param_1 + 0x148) = 1;
  FUN_00462be0(param_1,1);
  SetFocus(*(HWND *)(*(int *)((int)param_1 + 0x150) + 0x18));
  ExceptionList = pvStack_c;
  return;
}

