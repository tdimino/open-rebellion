
void __fastcall FUN_00460270(void *param_1)

{
  undefined4 uVar1;
  HDC pHVar2;
  HDC hdc;
  void *pvVar3;
  int iVar4;
  int iVar5;
  HGDIOBJ h;
  undefined4 *this;
  undefined *this_00;
  uint *puVar6;
  int *piVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  uint uVar10;
  uint uVar11;
  undefined4 uVar12;
  uint *local_2c;
  undefined4 *local_28;
  uint *puStack_24;
  undefined4 *local_20;
  undefined2 uStack_18;
  undefined2 uStack_16;
  void *local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006310ab;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  iVar4 = *(int *)((int)param_1 + 0x150);
  FUN_00606980(param_1,*(int *)(iVar4 + 0xcc),*(int *)(iVar4 + 0xd0),*(int *)(iVar4 + 0xd4),
               *(int *)(iVar4 + 0xd8));
  local_14 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (local_14 == (void *)0x0) {
    local_20 = (undefined4 *)0x0;
  }
  else {
    local_20 = (undefined4 *)FUN_005fbd20(local_14,uVar1,0x291a,10);
  }
  local_4 = 0xffffffff;
  iVar4 = *(int *)(*(int *)((int)param_1 + 0x150) + 0x9c);
  if (iVar4 == 1) {
    local_14 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (local_14 == (void *)0x0) {
      local_2c = (uint *)0x0;
    }
    else {
      local_2c = (uint *)FUN_005fbd20(local_14,uVar1,0x285f,10);
    }
    local_4 = 0xffffffff;
    local_14 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (local_14 == (void *)0x0) {
      local_28 = (undefined4 *)0x0;
    }
    else {
      local_28 = (undefined4 *)FUN_005fbd20(local_14,uVar1,0x2958,10);
    }
  }
  else if (iVar4 == 2) {
    local_14 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (local_14 == (void *)0x0) {
      local_2c = (uint *)0x0;
    }
    else {
      local_2c = (uint *)FUN_005fbd20(local_14,uVar1,0x2860,10);
    }
    local_4 = 0xffffffff;
    local_14 = (void *)FUN_00618b70(0x28);
    local_4 = 4;
    if (local_14 == (void *)0x0) {
      local_28 = (undefined4 *)0x0;
    }
    else {
      local_28 = (undefined4 *)FUN_005fbd20(local_14,uVar1,0x295c,10);
    }
  }
  local_4 = 0xffffffff;
  pHVar2 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar2);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar2);
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 5;
  local_14 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    puStack_24 = (uint *)0x0;
  }
  else {
    puVar6 = local_2c;
    pHVar2 = hdc;
    iVar4 = FUN_005fc0f0((int *)local_2c);
    iVar5 = FUN_005fc0e0((int *)local_2c);
    puStack_24 = (uint *)FUN_005fbea0(pvVar3,iVar5,iVar4,puVar6,pHVar2);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(local_20,puStack_24,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(local_2c,puStack_24,0,0);
  FUN_005fcc30(local_28,puStack_24,0x19c,0,0,0,(undefined4 *)0x0,0);
  h = (HGDIOBJ)FUN_005fc130((int)puStack_24);
  local_14 = SelectObject(hdc,h);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 6;
  if (pvStack_10 == (void *)0x0) {
    this = (undefined4 *)0x0;
  }
  else {
    uStack_16 = DAT_0065d424;
    uStack_18 = 0x1865;
    this = (undefined4 *)FUN_00601700(pvStack_10,&uStack_18,0x24,0xe,0,0,0,0);
  }
  local_4 = 0xffffffff;
  *this = 0x15e;
  this[0xb] = 1;
  FUN_00403e90(this,1);
  FUN_00601c60(this,5);
  FUN_00601c90(this,0x2ffffff,(HWND)0x0);
  FUN_00601ce0(this,hdc);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 7;
  if (pvStack_10 == (void *)0x0) {
    this_00 = (undefined *)0x0;
  }
  else {
    uStack_16 = DAT_0065d424;
    uStack_18 = 0x1866;
    this_00 = (undefined *)FUN_00601700(pvStack_10,&uStack_18,0x24,0x30,0,0,0,0);
  }
  local_4 = 0xffffffff;
  FUN_00601c60(this_00,4);
  FUN_00601c90(this_00,0x2ffffff,(HWND)0x0);
  FUN_00601ce0(this_00,hdc);
  SelectObject(hdc,local_14);
  DeleteDC(hdc);
  if (this != (undefined4 *)0x0) {
    FUN_00601990((int)this);
    FUN_00618b60((undefined *)this);
  }
  if (this_00 != (undefined *)0x0) {
    FUN_00601990((int)this_00);
    FUN_00618b60(this_00);
  }
  FUN_006075b0(param_1,(int *)puStack_24,1);
  if (local_20 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_20);
    FUN_00618b60((undefined *)local_20);
  }
  if (local_2c != (uint *)0x0) {
    FUN_005fbfa0(local_2c);
    FUN_00618b60((undefined *)local_2c);
  }
  if (local_28 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_28);
    FUN_00618b60((undefined *)local_28);
  }
  pvVar3 = (void *)FUN_00618b70(0x9c);
  local_4 = 8;
  pvStack_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar8 = *(undefined4 *)((int)param_1 + 0x1c);
    puVar6 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_0060d590(pvVar3,uVar8,0x24,0x4e,0x101,0x29,100,param_1,puVar6);
  }
  *(void **)((int)param_1 + 0x144) = pvVar3;
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_005ffce0(pvVar3,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 9;
    if (pvStack_10 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = (int *)FUN_00602150(pvStack_10,uVar1,0,0,0x31,0x2a,*(int *)((int)param_1 + 0x144),1,
                                   0x2904,0x2905,*(uint **)(*(int *)((int)param_1 + 0x144) + 0x98),0
                                   ,0);
    }
    local_4 = 0xffffffff;
    if (piVar7 != (int *)0x0) {
      FUN_005ffce0(piVar7,0);
      uStack_18 = 0x1867;
      uStack_16 = DAT_0065d424;
      FUN_00600a40(param_1,piVar7,&uStack_18);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar7,1,0);
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 10;
    if (pvStack_10 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = (int *)FUN_00602150(pvStack_10,uVar1,0x34,0,0x31,0x2a,*(int *)((int)param_1 + 0x144),
                                   2,0x2906,0x2907,*(uint **)(*(int *)((int)param_1 + 0x144) + 0x98)
                                   ,0,0);
    }
    local_4 = 0xffffffff;
    if (piVar7 != (int *)0x0) {
      FUN_005ffce0(piVar7,0);
      uStack_18 = 0x1868;
      uStack_16 = DAT_0065d424;
      FUN_00600a40(param_1,piVar7,&uStack_18);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar7,0,0);
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0xb;
    if (pvStack_10 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = (int *)FUN_00602150(pvStack_10,uVar1,0x68,0,0x31,0x2a,*(int *)((int)param_1 + 0x144),
                                   3,0x2909,0x290a,*(uint **)(*(int *)((int)param_1 + 0x144) + 0x98)
                                   ,0,0);
    }
    local_4 = 0xffffffff;
    if (piVar7 != (int *)0x0) {
      FUN_005ffce0(piVar7,0);
      uStack_16 = DAT_0065d424;
      uStack_18 = 0x1869;
      FUN_00600a40(param_1,piVar7,&uStack_18);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar7,0,0);
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0xc;
    if (pvStack_10 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = (int *)FUN_00602150(pvStack_10,uVar1,0x9c,0,0x31,0x2a,*(int *)((int)param_1 + 0x144),
                                   4,0x290c,0x290d,*(uint **)(*(int *)((int)param_1 + 0x144) + 0x98)
                                   ,0,0);
    }
    local_4 = 0xffffffff;
    if (piVar7 != (int *)0x0) {
      FUN_005ffce0(piVar7,0);
      uStack_18 = 0x1870;
      uStack_16 = DAT_0065d424;
      FUN_00600a40(param_1,piVar7,&uStack_18);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar7,0,0);
    }
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0xd;
    if (pvStack_10 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = (int *)FUN_00602150(pvStack_10,uVar1,0xd0,0,0x31,0x2a,*(int *)((int)param_1 + 0x144),
                                   5,0x290f,0x2910,*(uint **)(*(int *)((int)param_1 + 0x144) + 0x98)
                                   ,0,0);
    }
    local_4 = 0xffffffff;
    if (piVar7 != (int *)0x0) {
      FUN_005ffce0(piVar7,0);
      uStack_18 = 0x1871;
      uStack_16 = DAT_0065d424;
      FUN_00600a40(param_1,piVar7,&uStack_18);
      FUN_0060d700(*(void **)((int)param_1 + 0x144),piVar7,0,0);
    }
  }
  if (*(int *)(*(int *)((int)param_1 + 0x150) + 0x9c) == 1) {
    FUN_00606c90(param_1,0x1ab,0x19,0x2912,0x2913,200,0x1954,0,0,0,0,0,0);
    uVar11 = 0x2917;
    uVar10 = 0x2916;
    uVar9 = 0x5d;
    uVar8 = 0x1ab;
  }
  else {
    FUN_00606c90(param_1,0x1aa,0x15,0x2914,0x2915,200,0x1954,0,0,0,0,0,0);
    uVar11 = 0x2919;
    uVar10 = 0x2918;
    uVar9 = 0x59;
    uVar8 = 0x1aa;
  }
  FUN_00606c90(param_1,uVar8,uVar9,uVar10,uVar11,0xc9,0x1953,0,0,0,0,0,0);
  pvVar3 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),200);
  if (pvVar3 != (void *)0x0) {
    FUN_00602840(pvVar3,0x260);
  }
  pvVar3 = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),0xc9);
  if (pvVar3 != (void *)0x0) {
    FUN_00602840(pvVar3,0x260);
  }
  pvVar3 = (void *)FUN_00618b70(0xf4);
  local_4 = 0xe;
  pvStack_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar8 = *(undefined4 *)((int)param_1 + 0x1c);
    uVar10 = 0;
    puVar6 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_00604cf0(pvVar3,uVar8,0x8f,0x2d,0xfa,0x12,(int)param_1,puVar6,uVar10);
  }
  *(void **)((int)param_1 + 0x148) = pvVar3;
  *(undefined4 *)((int)pvVar3 + 0x24) = 0xcb;
  local_4 = 0xffffffff;
  *(undefined4 *)(*(int *)((int)param_1 + 0x148) + 0xe4) = 0xffffff;
  FUN_005ffce0(*(void **)((int)param_1 + 0x148),0);
  FUN_00604f90(*(void **)((int)param_1 + 0x148),(char *)0x0);
  SetCaretPos(0,0);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 0xf;
  if (pvStack_10 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00601880(pvStack_10,(char *)0x0,0,0,0,0,0,0);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x154) = pvVar3;
  if (pvVar3 != (void *)0x0) {
    FUN_00601b30(pvVar3,0x28,0x77);
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
  pvVar3 = (void *)FUN_00618b70(0x118);
  local_4 = 0x10;
  pvStack_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar12 = 0x14;
    uVar9 = 0x15e;
    uVar8 = 0xca;
    puVar6 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_00607ea0(pvVar3,uVar1,0x24,0x8a,0x15e,0xa5,param_1,puVar6,uVar8,uVar9,uVar12);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x14c) = pvVar3;
  if (pvVar3 != (void *)0x0) {
    *(undefined4 *)((int)pvVar3 + 0xf4) = 0x30000;
    FUN_005ffce0(*(void **)((int)param_1 + 0x14c),0);
    FUN_00609340(*(void **)((int)param_1 + 0x14c),2);
    *(undefined4 *)(*(int *)((int)param_1 + 0x14c) + 0xe4) = 0x24;
    iVar4 = *(int *)((int)param_1 + 0x14c);
    FUN_0060a490((void *)(iVar4 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar4,7,0x299d,
                 *(undefined4 *)(iVar4 + 0x3c),0,*(undefined4 *)(iVar4 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x14c) + 0xe8) = 5;
    FUN_00608280(*(void **)((int)param_1 + 0x14c),*(undefined4 *)((int)param_1 + 0x15c));
  }
  FUN_004611d0(param_1,1);
  SetFocus(*(HWND *)(*(int *)((int)param_1 + 0x148) + 0x18));
  ExceptionList = pvStack_c;
  return;
}

