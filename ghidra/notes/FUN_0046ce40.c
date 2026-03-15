
void __fastcall FUN_0046ce40(void *param_1)

{
  undefined4 uVar1;
  HDC pHVar2;
  HDC pHVar3;
  HBRUSH pHVar4;
  void *pvVar5;
  int iVar6;
  int iVar7;
  undefined4 *puVar8;
  undefined *this;
  HGDIOBJ pvVar9;
  uint *puVar10;
  int *piVar11;
  undefined4 uVar12;
  uint uVar13;
  uint uVar14;
  undefined4 uVar15;
  uint *puVar16;
  uint *puStack_30;
  uint *puStack_2c;
  undefined4 *puStack_28;
  undefined4 uStack_24;
  uint *puStack_20;
  undefined4 *puStack_1c;
  undefined4 uStack_18;
  undefined4 local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006320f0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  iVar6 = *(int *)((int)param_1 + 0x150);
  local_14 = uVar1;
  FUN_00606980(param_1,*(int *)(iVar6 + 0xcc),*(int *)(iVar6 + 0xd0),*(int *)(iVar6 + 0xd4),
               *(int *)(iVar6 + 0xd8));
  pHVar2 = GetDC(*(HWND *)((int)param_1 + 0x18));
  pHVar3 = CreateCompatibleDC(pHVar2);
  *(HDC *)((int)param_1 + 0x160) = pHVar3;
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar2);
  pHVar4 = CreateSolidBrush(0x20000ff);
  *(HBRUSH *)((int)param_1 + 0x164) = pHVar4;
  pHVar4 = CreateSolidBrush(0x200ff00);
  *(HBRUSH *)((int)param_1 + 0x168) = pHVar4;
  pHVar4 = CreateSolidBrush(0x808080);
  *(HBRUSH *)((int)param_1 + 0x16c) = pHVar4;
  pHVar4 = CreateSolidBrush(0x80);
  *(HBRUSH *)((int)param_1 + 0x170) = pHVar4;
  pHVar4 = CreateSolidBrush(0x8000);
  *(HBRUSH *)((int)param_1 + 0x174) = pHVar4;
  uStack_18 = (undefined4 *)FUN_00618b70(0x28);
  uStack_4 = 0;
  if (uStack_18 == (undefined4 *)0x0) {
    puStack_1c = (undefined4 *)0x0;
  }
  else {
    puStack_1c = (undefined4 *)FUN_005fbd20(uStack_18,uVar1,0x292c,10);
  }
  uStack_4 = 0xffffffff;
  uStack_18 = (undefined4 *)FUN_00618b70(0x28);
  uStack_4 = 1;
  if (uStack_18 == (undefined4 *)0x0) {
    uStack_18 = (undefined4 *)0x0;
  }
  else {
    uStack_18 = (undefined4 *)FUN_005fbd20(uStack_18,uVar1,0x292d,10);
  }
  uStack_4 = 0xffffffff;
  iVar6 = *(int *)(*(int *)((int)param_1 + 0x150) + 0x9c);
  if (iVar6 == 1) {
    puStack_20 = (uint *)FUN_00618b70(0x28);
    uStack_4 = 2;
    if (puStack_20 == (uint *)0x0) {
      puStack_2c = (uint *)0x0;
    }
    else {
      puStack_2c = (uint *)FUN_005fbd20(puStack_20,uVar1,0x285f,10);
    }
    uStack_4 = 0xffffffff;
    puStack_20 = (uint *)FUN_00618b70(0x28);
    uStack_4 = 3;
    if (puStack_20 == (uint *)0x0) {
      puStack_28 = (undefined4 *)0x0;
    }
    else {
      puStack_28 = (undefined4 *)FUN_005fbd20(puStack_20,uVar1,0x2958,10);
    }
  }
  else if (iVar6 == 2) {
    puStack_20 = (uint *)FUN_00618b70(0x28);
    uStack_4 = 4;
    if (puStack_20 == (uint *)0x0) {
      puStack_2c = (uint *)0x0;
    }
    else {
      puStack_2c = (uint *)FUN_005fbd20(puStack_20,uVar1,0x2860,10);
    }
    uStack_4 = 0xffffffff;
    puStack_20 = (uint *)FUN_00618b70(0x28);
    uStack_4 = 5;
    if (puStack_20 == (uint *)0x0) {
      puStack_28 = (undefined4 *)0x0;
    }
    else {
      puStack_28 = (undefined4 *)FUN_005fbd20(puStack_20,uVar1,0x295c,10);
    }
  }
  uStack_4 = 0xffffffff;
  pvVar5 = (void *)FUN_00618b70(0x28);
  uStack_4 = 6;
  if (pvVar5 == (void *)0x0) {
    puStack_20 = (uint *)0x0;
  }
  else {
    pHVar2 = *(HDC *)((int)param_1 + 0x160);
    puVar10 = puStack_2c;
    puStack_20 = (uint *)pvVar5;
    iVar6 = FUN_005fc0f0((int *)puStack_2c);
    iVar7 = FUN_005fc0e0((int *)puStack_2c);
    puStack_20 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar10,pHVar2);
  }
  puVar10 = puStack_20;
  uStack_4 = 0xffffffff;
  pvVar5 = (void *)FUN_00618b70(0x28);
  uStack_4 = 7;
  uStack_24 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    puStack_30 = (uint *)0x0;
  }
  else {
    pHVar2 = *(HDC *)((int)param_1 + 0x160);
    puVar16 = puStack_2c;
    iVar6 = FUN_005fc0f0((int *)puStack_2c);
    iVar7 = FUN_005fc0e0((int *)puStack_2c);
    puStack_30 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar16,pHVar2);
  }
  uStack_4 = 0xffffffff;
  FUN_005fcc30(puStack_1c,puVar10,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(uStack_18,puStack_30,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puStack_2c,puVar10,0,0);
  FUN_005fd0f0(puStack_2c,puStack_30,0,0);
  FUN_005fcc30(puStack_28,puVar10,0x19c,0,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(puStack_28,puStack_30,0x19c,0,0,0,(undefined4 *)0x0,0);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  uStack_4 = 8;
  if (pvStack_10 == (void *)0x0) {
    puVar8 = (undefined4 *)0x0;
  }
  else {
    uStack_24 = (void *)CONCAT22(DAT_0065d424,0x1900);
    puVar8 = (undefined4 *)FUN_00601700(pvStack_10,&uStack_24,0x24,0xe,0,0,0,0);
  }
  uStack_4 = 0xffffffff;
  FUN_00601c60(puVar8,5);
  FUN_00601c90(puVar8,0x2ffffff,(HWND)0x0);
  *puVar8 = 0x15e;
  puVar8[0xb] = 1;
  FUN_00403e90(puVar8,1);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  uStack_4 = 9;
  if (pvStack_10 == (void *)0x0) {
    this = (undefined *)0x0;
  }
  else {
    uStack_24 = (void *)CONCAT22(DAT_0065d424,0x1901);
    this = (undefined *)FUN_00601700(pvStack_10,&uStack_24,0x24,0x30,0,0,0,0);
  }
  uStack_4 = 0xffffffff;
  FUN_00601c60(this,4);
  FUN_00601c90(this,0x2ffffff,(HWND)0x0);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puVar10);
  uStack_24 = SelectObject(*(HDC *)((int)param_1 + 0x160),pvVar9);
  FUN_00601ce0(puVar8,*(HDC *)((int)param_1 + 0x160));
  FUN_00601ce0(this,*(HDC *)((int)param_1 + 0x160));
  SelectObject(*(HDC *)((int)param_1 + 0x160),uStack_24);
  pvVar9 = (HGDIOBJ)FUN_005fc130((int)puStack_30);
  uStack_24 = SelectObject(*(HDC *)((int)param_1 + 0x160),pvVar9);
  FUN_00601ce0(puVar8,*(HDC *)((int)param_1 + 0x160));
  FUN_00601ce0(this,*(HDC *)((int)param_1 + 0x160));
  SelectObject(*(HDC *)((int)param_1 + 0x160),uStack_24);
  if (puVar8 != (undefined4 *)0x0) {
    FUN_00601990((int)puVar8);
    FUN_00618b60((undefined *)puVar8);
  }
  if (this != (undefined *)0x0) {
    FUN_00601990((int)this);
    FUN_00618b60(this);
  }
  FUN_006075b0(param_1,(int *)puStack_20,1);
  FUN_00607610(param_1,(int *)puStack_30,2);
  puVar8 = puStack_1c;
  if (puStack_1c != (undefined4 *)0x0) {
    FUN_005fbfa0(puStack_1c);
    FUN_00618b60((undefined *)puVar8);
  }
  puVar8 = uStack_18;
  if (uStack_18 != (undefined4 *)0x0) {
    FUN_005fbfa0(uStack_18);
    FUN_00618b60((undefined *)puVar8);
  }
  if (puStack_2c != (uint *)0x0) {
    FUN_005fbfa0(puStack_2c);
    FUN_00618b60((undefined *)puStack_2c);
  }
  if (puStack_28 != (undefined4 *)0x0) {
    FUN_005fbfa0(puStack_28);
    FUN_00618b60((undefined *)puStack_28);
  }
  pvVar5 = (void *)FUN_00618b70(0x9c);
  uStack_4 = 10;
  pvStack_10 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1c);
    puVar10 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_0060d590(pvVar5,uVar1,0x24,0x48,0x99,0x29,100,param_1,puVar10);
  }
  *(void **)((int)param_1 + 0x144) = pvVar5;
  uStack_4 = 0xffffffff;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    uStack_4 = 0xb;
    if (pvStack_10 == (void *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(pvStack_10,local_14,0,0,0x31,0x29,*(int *)((int)param_1 + 0x144)
                                    ,1,0x2906,0x2907,
                                    *(uint **)(*(int *)((int)param_1 + 0x144) + 0x98),0,0);
    }
    uStack_4 = 0xffffffff;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      FUN_00602840(piVar11,0x260);
      uStack_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x1950);
      FUN_00600a40(param_1,piVar11,&uStack_18);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar11,1,0);
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    uStack_4 = 0xc;
    if (pvStack_10 == (void *)0x0) {
      piVar11 = (int *)0x0;
    }
    else {
      piVar11 = (int *)FUN_00602150(pvStack_10,local_14,0x34,0,0x31,0x29,
                                    *(int *)((int)param_1 + 0x144),2,0x2909,0x290a,
                                    *(uint **)(*(int *)((int)param_1 + 0x144) + 0x98),0,0);
    }
    uStack_4 = 0xffffffff;
    if (piVar11 != (int *)0x0) {
      FUN_005ffce0(piVar11,0);
      FUN_00602840(piVar11,0x260);
      uStack_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x1951);
      FUN_00600a40(param_1,piVar11,&uStack_18);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar11,0,0);
    }
  }
  if (*(int *)(*(int *)((int)param_1 + 0x150) + 0x9c) == 1) {
    FUN_00606c90(param_1,0x1ab,0x19,0x2912,0x2913,200,0x1954,0,0,0,0,0,0);
    uVar14 = 0x2917;
    uVar13 = 0x2916;
    uVar12 = 0x5d;
    uVar1 = 0x1ab;
  }
  else {
    FUN_00606c90(param_1,0x1aa,0x15,0x2914,0x2915,200,0x1954,0,0,0,0,0,0);
    uVar14 = 0x2919;
    uVar13 = 0x2918;
    uVar12 = 0x59;
    uVar1 = 0x1aa;
  }
  FUN_00606c90(param_1,uVar1,uVar12,uVar13,uVar14,0xc9,0x1953,0,0,0,0,0,0);
  pvVar5 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),200);
  if (pvVar5 != (void *)0x0) {
    FUN_00602840(pvVar5,0x260);
  }
  pvVar5 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),0xc9);
  if (pvVar5 != (void *)0x0) {
    FUN_00602840(pvVar5,0x260);
  }
  pvVar5 = (void *)FUN_00618b70(0xf4);
  uStack_4 = 0xd;
  pvStack_10 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1c);
    uVar13 = 0;
    puVar10 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00604cf0(pvVar5,uVar1,0x8f,0x2d,0xfa,0x12,(int)param_1,puVar10,uVar13);
  }
  *(void **)((int)param_1 + 0x148) = pvVar5;
  *(undefined4 *)((int)pvVar5 + 0x24) = 0xcb;
  uStack_4 = 0xffffffff;
  *(undefined4 *)(*(int *)((int)param_1 + 0x148) + 0xe4) = 0xffffff;
  FUN_005ffce0(*(void **)((int)param_1 + 0x148),0);
  FUN_00604f90(*(void **)((int)param_1 + 0x148),(char *)0x0);
  SetCaretPos(0,0);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  uStack_4 = 0xe;
  if (pvStack_10 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_00601880(pvStack_10,(char *)0x0,0,0,0,0,0,0);
  }
  uStack_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x154) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_00601b30(pvVar5,0x28,0x77);
    **(undefined4 **)((int)param_1 + 0x154) = 0x11b;
    *(undefined4 *)(*(int *)((int)param_1 + 0x154) + 4) = 0x10;
    *(undefined4 *)(*(int *)((int)param_1 + 0x154) + 0x2c) = 3;
    FUN_00403e90(*(void **)((int)param_1 + 0x154),0);
    FUN_00601c60(*(void **)((int)param_1 + 0x154),5);
    FUN_00601c90(*(void **)((int)param_1 + 0x154),0x2ffffff,(HWND)0x0);
    pHVar2 = GetDC(*(HWND *)((int)param_1 + 0x18));
    FUN_00601b80(*(void **)((int)param_1 + 0x154),pHVar2);
    ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar2);
  }
  pvVar5 = (void *)FUN_00618b70(0x118);
  uStack_4 = 0xf;
  pvStack_10 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar15 = 0x19;
    uVar12 = 0x14a;
    uVar1 = 0xca;
    puVar10 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00607ea0(pvVar5,local_14,0x25,0x90,0x15d,0x9f,param_1,puVar10,uVar1,uVar12,uVar15);
  }
  uStack_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x14c) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    *(undefined4 *)((int)pvVar5 + 0xf4) = 0x30000;
    FUN_005ffce0(*(void **)((int)param_1 + 0x14c),0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x14c) + 0xe4) = 0x20;
    iVar6 = *(int *)((int)param_1 + 0x14c);
    FUN_0060a490((void *)(iVar6 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar6,7,0x299d,
                 *(undefined4 *)(iVar6 + 0x3c),0,*(undefined4 *)(iVar6 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x14c) + 0xe8) = 5;
    FUN_00608280(*(void **)((int)param_1 + 0x14c),*(undefined4 *)((int)param_1 + 0x178));
  }
  if (*(void **)((int)param_1 + 0x144) != (void *)0x0) {
    if (*(int *)(*(int *)((int)param_1 + 0x150) + 0x9c) == 1) {
      uVar13 = 1;
    }
    else {
      uVar13 = 2;
    }
    FUN_0060d7e0(*(void **)((int)param_1 + 0x144),uVar13,1);
  }
  SetFocus(*(HWND *)(*(int *)((int)param_1 + 0x148) + 0x18));
  ExceptionList = pvStack_c;
  return;
}

