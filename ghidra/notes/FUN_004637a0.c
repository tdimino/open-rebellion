
void __fastcall FUN_004637a0(void *param_1)

{
  undefined4 uVar1;
  uint *puVar2;
  HBRUSH pHVar3;
  HDC pHVar4;
  HDC hdc;
  void *pvVar5;
  int iVar6;
  int iVar7;
  uint *puVar8;
  uint *puVar9;
  undefined4 *puVar10;
  undefined *this;
  HGDIOBJ pvVar11;
  int *piVar12;
  void *pvVar13;
  char *pcVar14;
  uint uVar15;
  undefined4 uVar16;
  uint uVar17;
  undefined4 uVar18;
  int iVar19;
  int iVar20;
  undefined4 uStack_4c;
  uint *puStack_48;
  uint *puStack_44;
  undefined4 local_40;
  uint local_3c;
  undefined4 *local_38;
  uint *puStack_34;
  uint *local_30;
  undefined4 *local_2c;
  undefined4 *local_28;
  undefined4 *local_24;
  undefined4 uStack_20;
  undefined4 *local_1c;
  undefined4 uStack_18;
  undefined4 local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063167a;
  pvStack_c = ExceptionList;
  local_3c = 0;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  iVar6 = *(int *)((int)param_1 + 0x154);
  local_40 = uVar1;
  FUN_00606980(param_1,*(int *)(iVar6 + 0xcc),*(int *)(iVar6 + 0xd0),*(int *)(iVar6 + 0xd4),
               *(int *)(iVar6 + 0xd8));
  local_14 = (uint *)FUN_00618b70(0x28);
  local_4 = 0;
  if (local_14 == (uint *)0x0) {
    local_24 = (undefined4 *)0x0;
  }
  else {
    local_24 = (undefined4 *)FUN_005fbd20(local_14,uVar1,0x292a,10);
  }
  local_4 = 0xffffffff;
  local_14 = (uint *)FUN_00618b70(0x28);
  local_4 = 1;
  if (local_14 == (uint *)0x0) {
    local_1c = (undefined4 *)0x0;
  }
  else {
    local_1c = (undefined4 *)FUN_005fbd20(local_14,uVar1,0x292b,10);
  }
  local_4 = 0xffffffff;
  local_14 = (uint *)FUN_00618b70(0x28);
  local_4 = 2;
  if (local_14 == (uint *)0x0) {
    local_2c = (undefined4 *)0x0;
  }
  else {
    local_2c = (undefined4 *)FUN_005fbd20(local_14,uVar1,0x2928,10);
  }
  local_4 = 0xffffffff;
  local_14 = (uint *)FUN_00618b70(0x28);
  local_4 = 3;
  if (local_14 == (uint *)0x0) {
    local_28 = (undefined4 *)0x0;
  }
  else {
    local_28 = (undefined4 *)FUN_005fbd20(local_14,uVar1,0x2929,10);
  }
  local_4 = 0xffffffff;
  iVar6 = *(int *)(*(int *)((int)param_1 + 0x154) + 0x9c);
  if (iVar6 == 1) {
    local_14 = (uint *)FUN_00618b70(0x28);
    local_4 = 4;
    if (local_14 == (uint *)0x0) {
      puVar2 = (uint *)0x0;
    }
    else {
      puVar2 = (uint *)FUN_005fbd20(local_14,uVar1,0x285f,10);
    }
    local_4 = 0xffffffff;
    local_14 = puVar2;
    local_30 = (uint *)FUN_00618b70(0x28);
    local_4 = 5;
    if (local_30 != (uint *)0x0) {
      local_38 = (undefined4 *)FUN_005fbd20(local_30,uVar1,0x295a,10);
      goto LAB_004639f5;
    }
  }
  else {
    puVar2 = local_14;
    if (iVar6 != 2) goto LAB_004639f5;
    local_14 = (uint *)FUN_00618b70(0x28);
    local_4 = 6;
    if (local_14 == (uint *)0x0) {
      puVar2 = (uint *)0x0;
    }
    else {
      puVar2 = (uint *)FUN_005fbd20(local_14,uVar1,0x2860,10);
    }
    local_4 = 0xffffffff;
    local_14 = puVar2;
    local_30 = (uint *)FUN_00618b70(0x28);
    local_4 = 7;
    if (local_30 != (uint *)0x0) {
      local_38 = (undefined4 *)FUN_005fbd20(local_30,uVar1,0x295e,10);
      goto LAB_004639f5;
    }
  }
  local_38 = (undefined4 *)0x0;
LAB_004639f5:
  local_4 = 0xffffffff;
  pHVar3 = CreateSolidBrush(0x20000ff);
  *(HBRUSH *)((int)param_1 + 0x15c) = pHVar3;
  pHVar3 = CreateSolidBrush(0x200ff00);
  *(HBRUSH *)((int)param_1 + 0x160) = pHVar3;
  pHVar3 = CreateSolidBrush(0x808080);
  *(HBRUSH *)((int)param_1 + 0x164) = pHVar3;
  pHVar3 = CreateSolidBrush(0x80);
  *(HBRUSH *)((int)param_1 + 0x168) = pHVar3;
  pHVar3 = CreateSolidBrush(0x8000);
  *(HBRUSH *)((int)param_1 + 0x16c) = pHVar3;
  pHVar4 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar4);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar4);
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4 = 8;
  local_30 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    puStack_48 = (uint *)0x0;
  }
  else {
    puVar8 = puVar2;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)puVar2);
    iVar7 = FUN_005fc0e0((int *)puVar2);
    puStack_48 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar8,pHVar4);
  }
  local_4 = 0xffffffff;
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4 = 9;
  local_30 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    puStack_44 = (uint *)0x0;
  }
  else {
    puVar8 = puVar2;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)puVar2);
    iVar7 = FUN_005fc0e0((int *)puVar2);
    puStack_44 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar8,pHVar4);
  }
  local_4 = 0xffffffff;
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4 = 10;
  local_30 = (uint *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar8 = (uint *)0x0;
  }
  else {
    puVar8 = puVar2;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)puVar2);
    iVar7 = FUN_005fc0e0((int *)puVar2);
    puVar8 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar8,pHVar4);
  }
  local_4 = 0xffffffff;
  puStack_34 = puVar8;
  local_30 = (uint *)FUN_00618b70(0x28);
  local_4 = 0xb;
  if (local_30 == (uint *)0x0) {
    puVar9 = (uint *)0x0;
  }
  else {
    puVar9 = puVar2;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)puVar2);
    iVar7 = FUN_005fc0e0((int *)puVar2);
    puVar9 = (uint *)FUN_005fbea0(local_30,iVar7,iVar6,puVar9,pHVar4);
  }
  local_4 = 0xffffffff;
  local_30 = puVar9;
  FUN_005fcc30(local_2c,puStack_48,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(local_28,puStack_44,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(local_24,puVar8,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(local_1c,puVar9,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar2,puStack_48,0,0);
  FUN_005fd0f0(puVar2,puStack_44,0,0);
  FUN_005fd0f0(puVar2,puVar8,0,0);
  FUN_005fd0f0(puVar2,puVar9,0,0);
  puVar10 = local_38;
  if (*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c) == 1) {
    FUN_005fcc30(local_38,puStack_48,0x19c,0,0,0,(undefined4 *)0x0,0);
  }
  else {
    FUN_005fcc30(local_38,puStack_48,0x19c,0,0,0,(undefined4 *)0x0,0);
  }
  FUN_005fcc30(puVar10,puStack_44,0x19c,0,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(puVar10,puVar8,0x19c,0,0,0,(undefined4 *)0x0,0);
  FUN_005fcc30(puVar10,puVar9,0x19c,0,0,0,(undefined4 *)0x0,0);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 0xc;
  if (pvStack_10 == (void *)0x0) {
    puVar10 = (undefined4 *)0x0;
  }
  else {
    uStack_4c = (HGDIOBJ)CONCAT22(DAT_0065d424,0x1890);
    puVar10 = (undefined4 *)FUN_00601700(pvStack_10,&uStack_4c,0x24,0xe,0,0,0,0);
  }
  local_4 = 0xffffffff;
  FUN_00601c60(puVar10,5);
  FUN_00601c90(puVar10,0x2ffffff,(HWND)0x0);
  *puVar10 = 0x15e;
  puVar10[0xb] = 1;
  FUN_00403e90(puVar10,1);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 0xd;
  if (pvStack_10 == (void *)0x0) {
    this = (undefined *)0x0;
  }
  else {
    uStack_4c = (HGDIOBJ)CONCAT22(DAT_0065d424,0x1891);
    this = (undefined *)FUN_00601700(pvStack_10,&uStack_4c,0x24,0x30,0,0,0,0);
  }
  local_4 = 0xffffffff;
  FUN_00601c60(this,4);
  FUN_00601c90(this,0x2ffffff,(HWND)0x0);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)puStack_34);
  uStack_4c = SelectObject(hdc,pvVar11);
  FUN_00601ce0(puVar10,hdc);
  FUN_00601ce0(this,hdc);
  SelectObject(hdc,uStack_4c);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)local_30);
  uStack_4c = SelectObject(hdc,pvVar11);
  FUN_00601ce0(puVar10,hdc);
  FUN_00601ce0(this,hdc);
  SelectObject(hdc,uStack_4c);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)puStack_48);
  uStack_4c = SelectObject(hdc,pvVar11);
  FUN_00601ce0(puVar10,hdc);
  FUN_00601ce0(this,hdc);
  SelectObject(hdc,uStack_4c);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)puStack_44);
  uStack_4c = SelectObject(hdc,pvVar11);
  FUN_00601ce0(puVar10,hdc);
  FUN_00601ce0(this,hdc);
  SelectObject(hdc,uStack_4c);
  if (puVar10 != (undefined4 *)0x0) {
    FUN_00601990((int)puVar10);
    FUN_00618b60((undefined *)puVar10);
  }
  if (this != (undefined *)0x0) {
    FUN_00601990((int)this);
    FUN_00618b60(this);
  }
  FUN_006075b0(param_1,(int *)puStack_34,2);
  FUN_00607610(param_1,(int *)local_30,3);
  FUN_00607610(param_1,(int *)puStack_48,4);
  FUN_00607610(param_1,(int *)puStack_44,5);
  puVar10 = local_2c;
  if (local_2c != (undefined4 *)0x0) {
    FUN_005fbfa0(local_2c);
    FUN_00618b60((undefined *)puVar10);
  }
  puVar10 = local_28;
  if (local_28 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_28);
    FUN_00618b60((undefined *)puVar10);
  }
  puVar10 = local_24;
  if (local_24 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_24);
    FUN_00618b60((undefined *)puVar10);
  }
  puVar10 = local_1c;
  if (local_1c != (undefined4 *)0x0) {
    FUN_005fbfa0(local_1c);
    FUN_00618b60((undefined *)puVar10);
  }
  puVar2 = local_14;
  if (local_14 != (uint *)0x0) {
    FUN_005fbfa0(local_14);
    FUN_00618b60((undefined *)puVar2);
  }
  puVar10 = local_38;
  if (local_38 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_38);
    FUN_00618b60((undefined *)puVar10);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c) == 1) {
    iVar7 = 0x1a7;
    uVar15 = 0x93;
    iVar6 = 0x20;
    local_14 = (uint *)0x55;
  }
  else {
    iVar7 = 0x1aa;
    uVar15 = 0x8f;
    iVar6 = 0x2a;
    local_14 = (uint *)0x5f;
  }
  local_24 = (undefined4 *)FUN_00618b70(0x9c);
  local_4 = 0xe;
  if (local_24 == (undefined4 *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    local_1c = *(undefined4 **)((int)param_1 + 0x1c);
    puVar2 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_0060d590(local_24,local_1c,iVar7,uVar15,iVar6,(int)local_14,0x6e,param_1,puVar2);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x148) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    if (*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c) == 1) {
      uVar1 = 0x1a7;
      local_1c = (undefined4 *)0x5d;
      iVar6 = 0x20;
      iVar7 = 0x1f;
      local_14 = (uint *)0x2916;
      uStack_4c = (HGDIOBJ)0x2917;
    }
    else {
      uVar1 = 0x1aa;
      local_1c = (undefined4 *)0x59;
      iVar6 = 0x2a;
      iVar7 = 0x29;
      local_14 = (uint *)0x2918;
      uStack_4c = (HGDIOBJ)0x2919;
    }
    local_24 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = 0xf;
    if (local_24 == (undefined4 *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      iVar20 = 0;
      iVar19 = 0;
      puVar2 = (uint *)FUN_006073d0(param_1);
      pvVar5 = FUN_00602150(local_24,local_40,uVar1,local_1c,iVar6,iVar7,param_1,0xc9,(uint)local_14
                            ,(uint)uStack_4c,puVar2,iVar19,iVar20);
    }
    local_4 = 0xffffffff;
    if (pvVar5 != (void *)0x0) {
      FUN_005ffce0(pvVar5,0);
      local_14 = (uint *)CONCAT22(DAT_0065d424,0x1953);
      FUN_00600a40(param_1,pvVar5,&local_14);
      FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar5);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c) == 1) {
      uVar15 = 0x292e;
      uStack_4c = (HGDIOBJ)0x292f;
    }
    else {
      uVar15 = 0x2930;
      uStack_4c = (HGDIOBJ)0x2931;
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x10;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,local_40,0,0,iVar6,iVar7,
                                    *(int *)((int)param_1 + 0x148),3,uVar15,(uint)uStack_4c,
                                    *(uint **)(*(int *)((int)param_1 + 0x148) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      local_14 = (uint *)CONCAT22(DAT_0065d424,0x1899);
      FUN_00600a40(param_1,piVar12,&local_14);
      FUN_0060d700(*(void **)((int)param_1 + 0x148),piVar12,0,0);
      FUN_00603150(piVar12,2,(uint)uStack_4c);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c) == 1) {
      uVar15 = 0x2932;
      uStack_4c = (HGDIOBJ)0x2933;
    }
    else {
      uVar15 = 0x2934;
      uStack_4c = (HGDIOBJ)0x2935;
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x11;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,local_40,0,0x36,iVar6,iVar7,
                                    *(int *)((int)param_1 + 0x148),2,uVar15,(uint)uStack_4c,
                                    *(uint **)(*(int *)((int)param_1 + 0x148) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      local_14 = (uint *)CONCAT22(DAT_0065d424,0x1898);
      FUN_00600a40(param_1,piVar12,&local_14);
      FUN_0060d700(*(void **)((int)param_1 + 0x148),piVar12,0,0);
      FUN_00603150(piVar12,2,(uint)uStack_4c);
    }
  }
  pvVar5 = (void *)FUN_00618b70(0x9c);
  local_4 = 0x12;
  pvStack_10 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1c);
    puVar2 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_0060d590(pvVar5,uVar1,0x24,0x48,0x99,0x29,100,param_1,puVar2);
  }
  *(void **)((int)param_1 + 0x144) = pvVar5;
  local_4 = 0xffffffff;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x13;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,local_40,0,0,0x31,0x29,*(int *)((int)param_1 + 0x144)
                                    ,1,0x2906,0x2907,
                                    *(uint **)(*(int *)((int)param_1 + 0x144) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      local_14 = (uint *)CONCAT22(DAT_0065d424,0x1893);
      FUN_00600a40(param_1,piVar12,&local_14);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar12,0,0);
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x14;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,local_40,0x34,0,0x31,0x29,
                                    *(int *)((int)param_1 + 0x144),2,0x2909,0x290a,
                                    *(uint **)(*(int *)((int)param_1 + 0x144) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      local_14 = (uint *)CONCAT22(DAT_0065d424,0x1894);
      FUN_00600a40(param_1,piVar12,&local_14);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar12,0,0);
    }
  }
  if (*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c) == 1) {
    uVar17 = 0x2913;
    uVar15 = 0x2912;
    uVar16 = 0x19;
    uVar1 = 0x1a7;
  }
  else {
    uVar17 = 0x2915;
    uVar15 = 0x2914;
    uVar16 = 0x15;
    uVar1 = 0x1aa;
  }
  FUN_00606c90(param_1,uVar1,uVar16,uVar15,uVar17,200,0x1954,0,0,0,0,0,0);
  pvVar5 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),200);
  if (pvVar5 != (void *)0x0) {
    FUN_00602840(pvVar5,0x260);
  }
  pvVar5 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),0xc9);
  if (pvVar5 != (void *)0x0) {
    FUN_00602840(pvVar5,0x260);
  }
  pvVar5 = (void *)FUN_00618b70(0xf4);
  local_4 = 0x15;
  pvStack_10 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar1 = *(undefined4 *)((int)param_1 + 0x1c);
    uVar15 = 0;
    puVar2 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00604cf0(pvVar5,uVar1,0x8f,0x2d,0xfa,0x12,(int)param_1,puVar2,uVar15);
  }
  *(void **)((int)param_1 + 0x14c) = pvVar5;
  *(undefined4 *)((int)pvVar5 + 0x24) = 0xcb;
  local_4 = 0xffffffff;
  *(undefined4 *)(*(int *)((int)param_1 + 0x14c) + 0xe4) = 0xffffff;
  FUN_005ffce0(*(void **)((int)param_1 + 0x14c),0);
  FUN_00604f90(*(void **)((int)param_1 + 0x14c),(char *)0x0);
  SetCaretPos(0,0);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 0x16;
  if (pvStack_10 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_00601880(pvStack_10,(char *)0x0,0,0,0,0,0,0);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x158) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_00601b30(pvVar5,0x28,0x73);
    **(undefined4 **)((int)param_1 + 0x158) = 0x11b;
    *(undefined4 *)(*(int *)((int)param_1 + 0x158) + 4) = 0x10;
    *(undefined4 *)(*(int *)((int)param_1 + 0x158) + 0x2c) = 3;
    FUN_00403e90(*(void **)((int)param_1 + 0x158),0);
    FUN_00601c60(*(void **)((int)param_1 + 0x158),5);
    FUN_00601c90(*(void **)((int)param_1 + 0x158),0x2ffffff,(HWND)0x0);
    pHVar4 = GetDC(*(HWND *)((int)param_1 + 0x18));
    FUN_00601b80(*(void **)((int)param_1 + 0x158),pHVar4);
    ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar4);
  }
  pvVar5 = (void *)FUN_00618b70(0x118);
  local_4 = 0x17;
  pvStack_10 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar18 = 0x14;
    uVar16 = 0x15e;
    uVar1 = 0xca;
    puVar2 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00607ea0(pvVar5,local_40,0x25,0x8f,0x15d,0xa1,param_1,puVar2,uVar1,uVar16,uVar18);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x150) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    *(undefined4 *)((int)pvVar5 + 0xf4) = 0x30000;
    FUN_005ffce0(*(void **)((int)param_1 + 0x150),0);
    FUN_00609340(*(void **)((int)param_1 + 0x150),2);
    *(undefined4 *)(*(int *)((int)param_1 + 0x150) + 0xe4) = 0x24;
    iVar6 = *(int *)((int)param_1 + 0x150);
    FUN_0060a490((void *)(iVar6 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar6,7,0x299d,
                 *(undefined4 *)(iVar6 + 0x3c),0,*(undefined4 *)(iVar6 + 0x3c),0,(uint *)0x0);
    local_1c = (undefined4 *)0x30;
    uStack_18 = 0x3c;
    *(undefined4 *)(*(int *)((int)param_1 + 0x150) + 0xe8) = 5;
    local_4 = 0x18;
    pvVar5 = (void *)FUN_004f3d50(*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c),(uint *)&local_1c,
                                  0);
    local_4 = 0xffffffff;
    FUN_00619730();
    while (pvVar5 != (void *)0x0) {
      pvVar13 = (void *)FUN_00618b70(0x74);
      local_4 = 0x19;
      pvStack_10 = pvVar13;
      if (pvVar13 == (void *)0x0) {
        pvVar13 = (void *)0x0;
      }
      else {
        puVar2 = FUN_004025b0(pvVar5,&local_14);
        local_3c = local_3c | 1;
        local_4 = CONCAT31(local_4._1_3_,0x1a);
        pvVar13 = FUN_004421d0(pvVar13,puVar2);
      }
      local_4 = 0x19;
      if ((local_3c & 1) != 0) {
        local_3c = local_3c & 0xfffffffe;
        FUN_00619730();
      }
      *(undefined4 *)((int)pvVar13 + 0x6c) = 0;
      local_4 = 0xffffffff;
      pcVar14 = (char *)FUN_00583c40(*(int *)((int)pvVar5 + 0x2c) + 0x34);
      FUN_005f35e0((void *)((int)pvVar13 + 0x14),pcVar14);
      uVar1 = FUN_004edc80((int)pvVar5);
      *(undefined4 *)((int)pvVar13 + 0x54) = uVar1;
      FUN_005f59f0(*(void **)((int)param_1 + 0x174),(int)pvVar13);
      local_24 = (undefined4 *)0x30;
      uStack_20 = 0x3c;
      local_4 = 0x1b;
      pvVar5 = (void *)FUN_004f6010(pvVar5,(uint *)&local_24,0);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  if (*(int *)(*(int *)((int)param_1 + 0x154) + 0x9c) == 1) {
    uVar15 = 1;
  }
  else {
    uVar15 = 2;
  }
  FUN_0060d7e0(*(void **)((int)param_1 + 0x144),uVar15,1);
  FUN_0060d7e0(*(void **)((int)param_1 + 0x148),3,1);
  SetFocus(*(HWND *)(*(int *)((int)param_1 + 0x14c) + 0x18));
  DeleteDC(hdc);
  ExceptionList = pvStack_c;
  return;
}

