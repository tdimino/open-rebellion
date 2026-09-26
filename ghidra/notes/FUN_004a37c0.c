
void __thiscall FUN_004a37c0(void *this,void *param_1,int *param_2)

{
  undefined4 uVar1;
  uint *puVar2;
  void *pvVar3;
  int iVar4;
  uint uVar5;
  HGDIOBJ h;
  undefined4 *puVar6;
  undefined *puVar7;
  char *pcVar8;
  int iVar9;
  uint *puVar10;
  undefined4 *puVar11;
  undefined1 uVar12;
  HDC pHVar13;
  void *local_34;
  undefined4 *local_30;
  uint local_28;
  uint local_24;
  HPALETTE pHStack_20;
  HGDIOBJ pvStack_1c;
  HGDIOBJ local_18;
  undefined4 *local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637b2d;
  pvStack_c = ExceptionList;
  puVar11 = (undefined4 *)0x0;
  local_28 = 0;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  local_14 = (undefined4 *)FUN_00618b70(0x28);
  local_4 = 0;
  if (local_14 != (undefined4 *)0x0) {
    puVar11 = FUN_004acd10(local_14);
  }
  local_4 = 0xffffffff;
  puVar2 = FUN_004025b0(param_1,&local_28);
  local_4 = 1;
  FUN_004f26d0(puVar11 + 6,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_004ace40(puVar11,*(uint *)((int)this + 0x24));
  FUN_004acba0((void *)((int)this + 0x114),puVar11,0);
  pvVar3 = (void *)FUN_00618b70(0x70);
  local_4 = 2;
  local_14 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    local_34 = (void *)0x0;
  }
  else {
    puVar2 = FUN_004025b0(param_1,&local_24);
    local_28 = 1;
    local_4 = CONCAT31(local_4._1_3_,3);
    local_34 = FUN_004c7e10(pvVar3,puVar2);
  }
  local_4 = 2;
  if (pvVar3 != (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  local_14 = (undefined4 *)FUN_00618b70(0x28);
  local_4 = 4;
  if (local_14 == (void *)0x0) {
    puVar2 = (uint *)0x0;
  }
  else {
    puVar2 = FUN_005fbea0(local_14,0x5b,0x32,*(uint **)((int)this + 0xa4),
                          *(HDC *)((int)this + 0x1ac));
  }
  local_4 = 0xffffffff;
  iVar4 = FUN_005fc130((int)puVar2);
  if (iVar4 == 0) goto LAB_004a3cc9;
  local_28 = 0;
  if ((*(uint *)((int)param_1 + 0x24) >> 6 & 3) == 1) {
    local_14 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = 5;
    if (local_14 == (void *)0x0) {
      local_30 = (undefined4 *)0x0;
    }
    else {
      local_30 = FUN_005fbd20(local_14,uVar1,0x28a0,2);
    }
    local_4 = 0xffffffff;
    local_14 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = 6;
    if (local_14 == (void *)0x0) goto LAB_004a3a15;
    puVar11 = FUN_005fbd20(local_14,uVar1,0x28a1,2);
  }
  else {
    local_28 = 0x32;
    local_14 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = 7;
    if (local_14 == (void *)0x0) {
      local_30 = (undefined4 *)0x0;
    }
    else {
      local_30 = FUN_005fbd20(local_14,uVar1,0x28d2,2);
    }
    local_4 = 0xffffffff;
    local_14 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = 8;
    if (local_14 == (void *)0x0) {
LAB_004a3a15:
      puVar11 = (undefined4 *)0x0;
    }
    else {
      puVar11 = FUN_005fbd20(local_14,uVar1,0x28d3,2);
    }
  }
  local_4 = 0xffffffff;
  uVar12 = *(undefined1 *)(local_30 + 2);
  iVar4 = FUN_005fc0f0((int *)puVar2);
  uVar5 = FUN_005fc0e0((int *)puVar2);
  FUN_005fcfe0(puVar2,0,0,uVar5,iVar4,uVar12);
  h = (HGDIOBJ)FUN_005fc130((int)puVar2);
  local_18 = SelectObject(*(HDC *)((int)this + 0x1ac),h);
  pHStack_20 = SelectPalette(*(HDC *)((int)this + 0x1ac),DAT_006be5a0,0);
  pvStack_1c = SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a0));
  local_14 = SelectObject(*(HDC *)((int)this + 0x1ac),*(HGDIOBJ *)((int)this + 0x1a4));
  if ((*(byte *)((int)param_1 + 0x50) & 0x10) != 0) {
    pvStack_10 = (void *)FUN_00618b70(0x28);
    local_4 = 9;
    if (pvStack_10 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = FUN_005fbd20(pvStack_10,uVar1,local_28 + 0x28b7,2);
    }
    local_4 = 0xffffffff;
    FUN_005fd0f0(puVar6,puVar2,5,5);
    if (puVar6 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar6);
      FUN_00618b60((undefined *)puVar6);
    }
  }
  if ((*(uint *)((int)param_1 + 0x50) & 0x200) != 0) {
    pvStack_10 = (void *)FUN_00618b70(0x28);
    local_4 = 10;
    if (pvStack_10 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = FUN_005fbd20(pvStack_10,uVar1,local_28 + 0x28b8,2);
    }
    local_4 = 0xffffffff;
    FUN_005fd0f0(puVar6,puVar2,5,5);
    if (puVar6 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar6);
      FUN_00618b60((undefined *)puVar6);
    }
  }
  if (local_30 != (undefined4 *)0x0) {
    FUN_005fd0f0(local_30,puVar2,5,5);
    FUN_005fbfa0(local_30);
    FUN_00618b60((undefined *)local_30);
  }
  if ((*(uint *)((int)param_1 + 0x24) >> 6 & 3) == *(uint *)((int)this + 0x170)) {
    uVar5 = *(uint *)((int)local_34 + 0x68) | 8;
  }
  else {
    uVar5 = *(uint *)((int)local_34 + 0x68) & 0xfffffff7;
  }
  *(uint *)((int)local_34 + 0x68) = uVar5;
  Polyline(*(HDC *)((int)this + 0x1ac),(POINT *)&apt_006a87b8,2);
  puVar7 = FUN_004f62d0((int)param_1);
  pcVar8 = (char *)FUN_00583c40((int)puVar7);
  FUN_005f35e0((void *)((int)local_34 + 0x14),pcVar8);
  SelectPalette(*(HDC *)((int)this + 0x1ac),pHStack_20,0);
  SelectObject(*(HDC *)((int)this + 0x1ac),pvStack_1c);
  SelectObject(*(HDC *)((int)this + 0x1ac),local_18);
  SelectObject(*(HDC *)((int)this + 0x1ac),local_14);
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 0xb;
  pvStack_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    puVar10 = (uint *)0x0;
  }
  else {
    pHVar13 = *(HDC *)((int)this + 0x1ac);
    puVar10 = puVar2;
    iVar4 = FUN_005fc0f0((int *)puVar2);
    iVar9 = FUN_005fc0e0((int *)puVar2);
    puVar10 = FUN_005fbea0(pvVar3,iVar9,iVar4,puVar10,pHVar13);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(puVar2,puVar10,0,0,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar11,puVar10,0,0);
  if (puVar11 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar11);
    FUN_00618b60((undefined *)puVar11);
  }
  FUN_0060be60(local_34,(int)puVar2,(int)puVar10);
LAB_004a3cc9:
  uVar5 = *(uint *)((int)local_34 + 0x68);
  *(uint *)((int)local_34 + 0x68) = uVar5 | 4;
  *(uint *)((int)local_34 + 0x68) = uVar5 & 0xfffffffd | 4;
  iVar4 = *param_2;
  if (iVar4 == 0) {
    FUN_005f59f0(*(void **)((int)this + 0x188),(int)local_34);
  }
  else {
    iVar9 = *(int *)(iVar4 + 4);
    *(void **)(iVar4 + 4) = local_34;
    *(int *)((int)local_34 + 4) = iVar9;
    *(int *)((int)local_34 + 8) = *param_2;
    if (iVar9 == 0) {
      FUN_005f5b60(*(int *)((int)this + 0x188));
    }
    else {
      *(void **)(iVar9 + 8) = local_34;
    }
  }
  *param_2 = (int)local_34;
  FUN_004a3d40(this,(int)param_1,param_2);
  ExceptionList = pvStack_c;
  return;
}

