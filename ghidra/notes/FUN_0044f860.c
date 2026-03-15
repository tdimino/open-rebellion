
void __fastcall FUN_0044f860(void *param_1)

{
  undefined4 *puVar1;
  undefined1 uVar2;
  undefined4 uVar3;
  uint *puVar4;
  HDC pHVar5;
  HDC hdc;
  char *pcVar6;
  void *pvVar7;
  int iVar8;
  int iVar9;
  uint *puVar10;
  HGDIOBJ pvVar11;
  int *piVar12;
  short sVar13;
  short sVar14;
  uint uVar15;
  short sVar16;
  uint uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  uint local_8c;
  uint local_84;
  undefined4 local_80;
  void *local_7c;
  void *local_78;
  undefined4 auStack_74 [11];
  undefined4 uStack_48;
  undefined4 auStack_40 [11];
  undefined4 uStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fae8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005970a0('\x03');
  local_78 = *(void **)((int)param_1 + 0x1c);
  uVar3 = FUN_006037f0(7);
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    uVar17 = 0x29d6;
    local_80 = (undefined4 *)0x29d8;
    local_8c = 0x29da;
    local_84 = 0xff;
  }
  else {
    uVar17 = 0x29d7;
    local_80 = (undefined4 *)0x29d9;
    local_8c = 0x29db;
    local_84 = 0xff00;
  }
  local_7c = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (local_7c == (void *)0x0) {
    puVar4 = (uint *)0x0;
  }
  else {
    puVar4 = (uint *)FUN_005fbd20(local_7c,uVar3,uVar17,10);
  }
  local_4 = 0xffffffff;
  pHVar5 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar5);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar5);
  FUN_00601880(auStack_40,(char *)0x0,0,0,0,0,0,0);
  local_4 = 1;
  FUN_00601880(auStack_74,(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 2;
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x114) + 0x54);
  FUN_00601aa0(auStack_40,pcVar6);
  FUN_00601c60(auStack_40,0xb);
  FUN_00403e90(auStack_40,1);
  FUN_00601b30(auStack_40,0,0x10);
  FUN_00601c90(auStack_40,local_84,(HWND)0x0);
  auStack_40[0] = 400;
  uStack_14 = 1;
  local_7c = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 3;
  if (local_7c == (void *)0x0) {
    local_80 = (undefined4 *)0x0;
  }
  else {
    local_80 = (undefined4 *)FUN_005fbd20(local_7c,uVar3,(uint)local_80,10);
  }
  local_4._0_1_ = 2;
  pvVar7 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 4;
  local_7c = pvVar7;
  if (pvVar7 == (void *)0x0) {
    puVar10 = (uint *)0x0;
  }
  else {
    puVar10 = puVar4;
    pHVar5 = hdc;
    iVar8 = FUN_005fc0f0((int *)puVar4);
    iVar9 = FUN_005fc0e0((int *)puVar4);
    puVar10 = (uint *)FUN_005fbea0(pvVar7,iVar9,iVar8,puVar10,pHVar5);
  }
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005fcc30(local_80,puVar10,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar4,puVar10,0,0);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)puVar10);
  local_7c = SelectObject(hdc,pvVar11);
  FUN_00601ce0(auStack_40,hdc);
  pcVar6 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x114) + 0x60);
  FUN_00601aa0(auStack_74,pcVar6);
  uStack_48 = 1;
  FUN_00601c60(auStack_74,6);
  FUN_00403e90(auStack_74,0x110);
  auStack_74[0] = 0x148;
  FUN_00601b30(auStack_74,0x25,0xd9);
  FUN_00601c90(auStack_74,0,(HWND)0x0);
  FUN_00601ce0(auStack_74,hdc);
  FUN_00601b30(auStack_74,0x24,0xd8);
  FUN_00601b80(auStack_74,hdc);
  FUN_00601c90(auStack_74,local_84,(HWND)0x0);
  FUN_00601ce0(auStack_74,hdc);
  FUN_006075b0(param_1,(int *)puVar10,0x32);
  puVar1 = local_80;
  if (local_80 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_80);
    FUN_00618b60((undefined *)puVar1);
  }
  local_80 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 5;
  if (local_80 == (undefined4 *)0x0) {
    local_80 = (undefined4 *)0x0;
  }
  else {
    local_80 = (undefined4 *)FUN_005fbd20(local_80,uVar3,local_8c,10);
  }
  local_4._0_1_ = 2;
  pvVar7 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 6;
  if (pvVar7 == (void *)0x0) {
    puVar10 = (uint *)0x0;
  }
  else {
    puVar10 = puVar4;
    pHVar5 = hdc;
    iVar8 = FUN_005fc0f0((int *)puVar4);
    iVar9 = FUN_005fc0e0((int *)puVar4);
    puVar10 = (uint *)FUN_005fbea0(pvVar7,iVar9,iVar8,puVar10,pHVar5);
  }
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005fcc30(local_80,puVar10,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar4,puVar10,0,0);
  pvVar11 = (HGDIOBJ)FUN_005fc130((int)puVar10);
  SelectObject(hdc,pvVar11);
  FUN_00601ce0(auStack_40,hdc);
  FUN_00607610(param_1,(int *)puVar10,0x33);
  if (puVar4 != (uint *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar1 = local_80;
  if (local_80 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_80);
    FUN_00618b60((undefined *)puVar1);
  }
  SelectObject(hdc,local_7c);
  DeleteDC(hdc);
  local_7c = (void *)FUN_00618b70(0x10);
  local_4._0_1_ = 7;
  if (local_7c == (void *)0x0) {
    pvVar7 = (void *)0x0;
  }
  else {
    pvVar7 = FUN_0060a790(local_7c,4);
  }
  local_4._0_1_ = 2;
  *(void **)((int)param_1 + 0x130) = pvVar7;
  pvVar7 = (void *)FUN_00618b70(0x118);
  local_4._0_1_ = 8;
  local_7c = pvVar7;
  if (pvVar7 == (void *)0x0) {
    pvVar7 = (void *)0x0;
  }
  else {
    uVar20 = 0x28;
    uVar19 = 0xf0;
    uVar18 = 100;
    puVar4 = (uint *)FUN_006076b0(param_1,0x33);
    pvVar7 = FUN_00607ea0(pvVar7,local_78,0x53,0x28,0x100,0xea,param_1,puVar4,uVar18,uVar19,uVar20);
  }
  local_4._0_1_ = 2;
  *(void **)((int)param_1 + 300) = pvVar7;
  if (pvVar7 != (void *)0x0) {
    *(undefined4 *)((int)pvVar7 + 0xf4) = 0x30000;
    *(uint *)(*(int *)((int)param_1 + 300) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 300) + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 300),0);
    iVar8 = *(int *)((int)param_1 + 300);
    FUN_0060a490((void *)(iVar8 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar8,7,0x29fc,
                 *(undefined4 *)(iVar8 + 0x3c),0,*(undefined4 *)(iVar8 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 300) + 0xe4) = 4;
    FUN_00608280(*(void **)((int)param_1 + 300),*(undefined4 *)((int)param_1 + 0x130));
    FUN_00609950(*(void **)((int)param_1 + 300),0xffffff);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    sVar16 = 0x1a2;
    sVar13 = 0x15;
    sVar14 = 0x29;
    local_80 = (undefined4 *)0xe1;
  }
  else {
    sVar16 = 0x1aa;
    sVar13 = 0x11;
    sVar14 = 0x2c;
    local_80 = (undefined4 *)0xe9;
  }
  local_78 = (void *)FUN_00618b70(0x9c);
  local_4._0_1_ = 9;
  if (local_78 == (void *)0x0) {
    pvVar7 = (void *)0x0;
  }
  else {
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar7 = FUN_0060d590(local_78,uVar3,(int)sVar16,(int)sVar13,(int)sVar14,(int)(short)local_80,
                          200,param_1,puVar4);
  }
  local_4._0_1_ = 2;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 2;
  *(void **)((int)param_1 + 0x134) = pvVar7;
  if (pvVar7 != (void *)0x0) {
    FUN_005ffce0(pvVar7,0);
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      sVar13 = 0x29;
      uVar17 = 0x29e8;
      sVar14 = 0x29;
      local_8c = 0x29e9;
    }
    else {
      sVar13 = 0x2c;
      sVar14 = 0x2d;
      uVar17 = 0x29f2;
      local_8c = 0x29f3;
    }
    local_78 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 10;
    if (local_78 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_78,uVar3,0,0,(int)sVar13,(int)sVar14,
                                    *(int *)((int)param_1 + 0x134),1,uVar17,local_8c,
                                    *(uint **)(*(int *)((int)param_1 + 0x134) + 0x98),0,0);
    }
    local_4._0_1_ = 2;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      FUN_00602840(piVar12,0x260);
      local_80 = (undefined4 *)CONCAT22(DAT_0065d424,0x1922);
      FUN_00600a40(param_1,piVar12,&local_80);
      FUN_0060d700(*(void **)((int)param_1 + 0x134),piVar12,1,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      local_80 = (undefined4 *)0x3c;
      local_84 = 0x29ea;
      local_8c = 0x29eb;
    }
    else {
      local_80 = (undefined4 *)0x3f;
      local_84 = 0x29f4;
      local_8c = 0x29f5;
    }
    local_78 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 0xb;
    if (local_78 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_78,uVar3,0,(int)(short)local_80,(int)sVar13,(int)sVar14,
                                    *(int *)((int)param_1 + 0x134),2,local_84,local_8c,
                                    *(uint **)(*(int *)((int)param_1 + 0x134) + 0x98),0,0);
    }
    local_4._0_1_ = 2;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      FUN_00602840(piVar12,0x260);
      local_80 = (undefined4 *)CONCAT22(DAT_0065d424,0x1923);
      FUN_00600a40(param_1,piVar12,&local_80);
      FUN_0060d700(*(void **)((int)param_1 + 0x134),piVar12,0,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      local_80 = (undefined4 *)0x7a;
      local_84 = 0x29ec;
      local_8c = 0x29ed;
    }
    else {
      local_80 = (undefined4 *)0x7e;
      local_84 = 0x29f6;
      local_8c = 0x29f7;
    }
    local_78 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 0xc;
    if (local_78 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_78,uVar3,0,(int)(short)local_80,(int)sVar13,(int)sVar14,
                                    *(int *)((int)param_1 + 0x134),3,local_84,local_8c,
                                    *(uint **)(*(int *)((int)param_1 + 0x134) + 0x98),0,0);
    }
    local_4._0_1_ = 2;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      FUN_00602840(piVar12,0x260);
      local_80 = (undefined4 *)CONCAT22(DAT_0065d424,0x1924);
      FUN_00600a40(param_1,piVar12,&local_80);
      FUN_0060d700(*(void **)((int)param_1 + 0x134),piVar12,0,0);
    }
    if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
      local_80 = (undefined4 *)0xb8;
      local_84 = 0x29ee;
      local_8c = 0x29ef;
    }
    else {
      local_80 = (undefined4 *)0xbd;
      local_84 = 0x29f8;
      local_8c = 0x29f9;
    }
    local_78 = (void *)FUN_00618b70(0x114);
    local_4._0_1_ = 0xd;
    if (local_78 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_78,uVar3,0,(int)(short)local_80,(int)sVar13,(int)sVar14,
                                    *(int *)((int)param_1 + 0x134),4,local_84,local_8c,
                                    *(uint **)(*(int *)((int)param_1 + 0x134) + 0x98),0,0);
    }
    local_4._0_1_ = 2;
    uVar2 = (undefined1)local_4;
    if (piVar12 != (int *)0x0) {
      FUN_005ffce0(piVar12,0);
      FUN_00602840(piVar12,0x260);
      local_80 = (undefined4 *)CONCAT22(DAT_0065d424,0x1925);
      FUN_00600a40(param_1,piVar12,&local_80);
      FUN_0060d700(*(void **)((int)param_1 + 0x134),piVar12,0,0);
      uVar2 = (undefined1)local_4;
    }
  }
  local_4._0_1_ = uVar2;
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    uVar17 = 0x29e2;
    uVar15 = 0x29e3;
    local_80 = (undefined4 *)0x29e4;
  }
  else {
    uVar17 = 0x29dc;
    uVar15 = 0x29dd;
    local_80 = (undefined4 *)0x29de;
  }
  pvVar7 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xe;
  local_78 = pvVar7;
  if (pvVar7 == (void *)0x0) {
    pvVar7 = (void *)0x0;
  }
  else {
    iVar9 = 0;
    iVar8 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar7 = FUN_00602150(pvVar7,uVar3,0x92,0x128,0x86,0x1b,param_1,0x65,uVar17,uVar15,puVar4,iVar8,
                          iVar9);
  }
  local_4._0_1_ = 2;
  *(void **)((int)param_1 + 0x120) = pvVar7;
  if (pvVar7 != (void *)0x0) {
    FUN_005ffce0(pvVar7,0);
    FUN_00603150(*(void **)((int)param_1 + 0x120),2,(uint)local_80);
    local_80 = (undefined4 *)CONCAT22(DAT_0065d424,0x1926);
    FUN_00600a40(param_1,*(void **)((int)param_1 + 0x120),&local_80);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    uVar17 = 0x29e5;
    uVar15 = 0x29e6;
    local_80 = (undefined4 *)0x29e7;
  }
  else {
    uVar17 = 0x29df;
    uVar15 = 0x29e0;
    local_80 = (undefined4 *)0x29e1;
  }
  pvVar7 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0xf;
  local_78 = pvVar7;
  if (pvVar7 == (void *)0x0) {
    pvVar7 = (void *)0x0;
  }
  else {
    iVar9 = 0;
    iVar8 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar7 = FUN_00602150(pvVar7,uVar3,0x118,0x128,0x86,0x1b,param_1,0x66,uVar17,uVar15,puVar4,iVar8
                          ,iVar9);
  }
  local_4._0_1_ = 2;
  *(void **)((int)param_1 + 0x124) = pvVar7;
  if (pvVar7 != (void *)0x0) {
    FUN_005ffce0(pvVar7,0);
    FUN_00603150(*(void **)((int)param_1 + 0x124),2,(uint)local_80);
    local_80 = (undefined4 *)CONCAT22(DAT_0065d424,0x1927);
    FUN_00600a40(param_1,*(void **)((int)param_1 + 0x124),&local_80);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    uVar17 = 0x2ade;
    uVar15 = 0x2adf;
    local_80 = (undefined4 *)0x2ae0;
  }
  else {
    uVar17 = 0x2adb;
    uVar15 = 0x2adc;
    local_80 = (undefined4 *)0x2add;
  }
  pvVar7 = (void *)FUN_00618b70(0x114);
  local_4._0_1_ = 0x10;
  local_78 = pvVar7;
  if (pvVar7 == (void *)0x0) {
    pvVar7 = (void *)0x0;
  }
  else {
    iVar9 = 0;
    iVar8 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar7 = FUN_00602150(pvVar7,uVar3,0xc,0x128,0x86,0x1b,param_1,0x67,uVar17,uVar15,puVar4,iVar8,
                          iVar9);
  }
  local_4 = CONCAT31(local_4._1_3_,2);
  *(void **)((int)param_1 + 0x128) = pvVar7;
  if (pvVar7 != (void *)0x0) {
    FUN_005ffce0(pvVar7,0);
    FUN_00603150(*(void **)((int)param_1 + 0x128),2,(uint)local_80);
    local_80 = (undefined4 *)CONCAT22(DAT_0065d424,0x194a);
    FUN_00600a40(param_1,*(void **)((int)param_1 + 0x128),&local_80);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x50) == 0) {
    (**(code **)(**(int **)((int)param_1 + 0x128) + 0x10))();
  }
  else {
    (**(code **)(**(int **)((int)param_1 + 0x128) + 0xc))();
  }
  FUN_00450660(param_1,1);
  if (*(int *)(*(int *)((int)param_1 + 0x118) + 0x9c) == 1) {
    uVar17 = 10;
  }
  else {
    uVar17 = 0xb;
  }
  FUN_00417610(uVar17);
  iVar8 = *(int *)((int)param_1 + 0x118);
  FUN_00606980(param_1,*(int *)(iVar8 + 0xcc),*(int *)(iVar8 + 0xd0),*(int *)(iVar8 + 0xd4),
               *(int *)(iVar8 + 0xd8));
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00601990((int)auStack_74);
  local_4 = 0xffffffff;
  FUN_00601990((int)auStack_40);
  ExceptionList = pvStack_c;
  return;
}

