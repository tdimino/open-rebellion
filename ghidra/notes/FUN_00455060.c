
void __fastcall FUN_00455060(int *param_1)

{
  undefined1 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  char *pcVar6;
  HDC hDC;
  int iVar7;
  uint *puVar8;
  uint *puVar9;
  int iVar10;
  int *piVar11;
  int *piVar12;
  int iVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  int iVar16;
  uint uVar17;
  int iVar18;
  HDC pHVar19;
  uint uVar20;
  int iVar21;
  undefined4 local_6c;
  undefined4 local_68;
  undefined4 *local_64;
  undefined4 local_60;
  int *local_5c;
  undefined **local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  uint uStack_48;
  undefined4 local_38 [3];
  undefined4 auStack_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006301c2;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_68 = FUN_004a25c0(param_1);
  param_1[0x52] = (uint)local_68[9] >> 6 & 3;
  FUN_005f2f50(local_38);
  piVar12 = (int *)0x0;
  local_4 = 0;
  FUN_004ece30(&local_5c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_60);
  local_4._0_1_ = 2;
  uVar2 = FUN_006037f0(7);
  local_6c = uVar2;
  uVar3 = FUN_006037f0(2);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 3;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_0060db00(local_64,param_1[0x53]);
  }
  param_1[0x99] = (int)pvVar4;
  local_4._0_1_ = 2;
  FUN_006071a0(param_1);
  FUN_00607ce0(param_1,2);
  DAT_006be5ac = uVar3;
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 4;
  if (local_64 == (undefined4 *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    puVar5 = FUN_004acd10(local_64);
  }
  local_4._0_1_ = 2;
  FUN_004f26d0(puVar5 + 6,param_1 + 0x51);
  FUN_004ace40(puVar5,param_1[9]);
  FUN_004acba0(param_1 + 0x45,puVar5,0);
  iVar13 = 5;
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 5;
  if (local_64 != (undefined4 *)0x0) {
    piVar12 = (int *)FUN_005fbd20(local_64,uVar2,0x27e1,2);
  }
  local_4._0_1_ = 2;
  if (piVar12 != (int *)0x0) {
    iVar13 = FUN_005fc0e0(piVar12);
    iVar13 = iVar13 + 5;
    FUN_005fbfa0(piVar12);
    FUN_00618b60((undefined *)piVar12);
  }
  pvVar4 = FUN_004f6270(local_68,&local_58);
  local_4._0_1_ = 6;
  pcVar6 = (char *)FUN_00583c40((int)pvVar4);
  FUN_00601aa0((void *)param_1[0x49],pcVar6);
  local_4._0_1_ = 2;
  FUN_005f2ff0(&local_58);
  FUN_00601c60((void *)param_1[0x49],5);
  *(undefined4 *)(param_1[0x49] + 4) = 0x10;
  *(undefined4 *)param_1[0x49] = 0xb3;
  FUN_00601b30((void *)param_1[0x49],iVar13,2);
  FUN_00403e90((void *)param_1[0x49],0x24);
  FUN_00601c90((void *)param_1[0x49],0x2000000,(HWND)0x0);
  SetRect((LPRECT)(param_1 + 0x4a),2,2,0xe0,0x12);
  PostMessageA(*(HWND *)(param_1[0x53] + 0x18),0x467,param_1[6],0);
  hDC = GetDC((HWND)param_1[6]);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  uVar2 = local_6c;
  local_4._0_1_ = 7;
  if (local_64 == (undefined4 *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    piVar12 = (int *)FUN_005fbd20(local_64,local_6c,0x2839,10);
  }
  local_4._0_1_ = 2;
  FUN_006075b0(param_1,piVar12,1);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 8;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_005fbd20(local_64,uVar2,0x2832,10);
  }
  local_4._0_1_ = 2;
  param_1[0x62] = (int)pvVar4;
  puVar5 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 9;
  local_64 = puVar5;
  if (puVar5 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    puVar8 = (uint *)param_1[0x62];
    pHVar19 = hDC;
    iVar13 = FUN_005fc0f0((int *)puVar8);
    iVar7 = FUN_005fc0e0((int *)param_1[0x62]);
    pvVar4 = FUN_005fbea0(puVar5,iVar7,iVar13,puVar8,pHVar19);
  }
  local_4._0_1_ = 2;
  param_1[0x65] = (int)pvVar4;
  FUN_00456230(param_1);
  local_64 = (undefined4 *)FUN_00618b70(0x34);
  local_4._0_1_ = 10;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_00601880(local_64,(char *)0x0,0,0,0,0,0,0);
  }
  param_1[0x5f] = (int)pvVar4;
  *(undefined4 *)((int)pvVar4 + 0x2c) = 1;
  local_4._0_1_ = 2;
  FUN_00403e90((void *)param_1[0x5f],1);
  *(int *)param_1[0x5f] = param_1[0xe];
  FUN_00601c60((void *)param_1[0x5f],4);
  FUN_00601c90((void *)param_1[0x5f],0x2ffffff,(HWND)0x0);
  FUN_00601b30((void *)param_1[0x5f],0,0x3a);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xb;
  if (local_64 == (undefined4 *)0x0) {
    puVar8 = (uint *)0x0;
  }
  else {
    puVar8 = (uint *)FUN_005fbd20(local_64,uVar2,0x283a,10);
  }
  local_4._0_1_ = 2;
  puVar5 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xc;
  local_64 = puVar5;
  if (puVar5 == (undefined4 *)0x0) {
    puVar9 = (uint *)0x0;
  }
  else {
    puVar9 = puVar8;
    iVar13 = FUN_005fc0f0((int *)puVar8);
    iVar7 = FUN_005fc0e0((int *)puVar8);
    puVar9 = (uint *)FUN_005fbda0(puVar5,iVar7,iVar13,puVar9);
  }
  uVar20 = 0;
  puVar5 = (undefined4 *)0x0;
  uVar17 = 0x47;
  iVar16 = 6;
  iVar7 = 0;
  iVar13 = 0;
  local_4 = CONCAT31(local_4._1_3_,2);
  param_1[0x66] = (int)puVar9;
  pvVar4 = (void *)FUN_006073d0(param_1);
  FUN_005fcc30(pvVar4,puVar9,iVar13,iVar7,iVar16,uVar17,puVar5,uVar20);
  FUN_005fd0f0(puVar8,(uint *)param_1[0x66],0,0);
  if (puVar8 != (uint *)0x0) {
    FUN_005fbfa0(puVar8);
    FUN_00618b60((undefined *)puVar8);
  }
  local_64 = (undefined4 *)FUN_00618b70(0x34);
  local_4._0_1_ = 0xd;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_00601880(local_64,(char *)0x0,0,0,0,0,0,0);
  }
  local_4._0_1_ = 2;
  param_1[0x67] = (int)pvVar4;
  FUN_00601c90(pvVar4,0x2ffffff,(HWND)0x0);
  FUN_00403e90((void *)param_1[0x67],1);
  *(undefined4 *)(param_1[0x67] + 0x2c) = 3;
  *(undefined4 *)param_1[0x67] = 0x2e;
  *(undefined4 *)(param_1[0x67] + 4) = 0x10;
  ReleaseDC((HWND)param_1[6],hDC);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  uVar2 = local_6c;
  local_4._0_1_ = 0xe;
  if (local_64 == (undefined4 *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    piVar12 = (int *)FUN_005fbd20(local_64,local_6c,0x277c,2);
  }
  local_4._0_1_ = 2;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 2;
  if (piVar12 != (int *)0x0) {
    puVar5 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0xf;
    local_64 = puVar5;
    if (puVar5 == (undefined4 *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      iVar13 = param_1[0xc];
      iVar21 = 1;
      iVar18 = 0;
      puVar8 = (uint *)FUN_006073d0(param_1);
      uVar20 = 0x277d;
      uVar17 = 0x277c;
      uVar3 = 0x65;
      piVar11 = param_1;
      iVar7 = FUN_005fc0f0(piVar12);
      iVar16 = FUN_005fc0e0(piVar12);
      uVar2 = 3;
      iVar10 = FUN_005fc0e0(piVar12);
      pvVar4 = FUN_00602150(puVar5,local_6c,(iVar13 - iVar10) + -3,uVar2,iVar16,iVar7,piVar11,uVar3,
                            uVar17,uVar20,puVar8,iVar18,iVar21);
    }
    local_4._0_1_ = 2;
    param_1[0x54] = (int)pvVar4;
    FUN_005ffce0(pvVar4,0);
    FUN_005fbfa0(piVar12);
    FUN_00618b60((undefined *)piVar12);
    uVar2 = local_6c;
    uVar1 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar1;
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 0x10;
  if (local_64 == (undefined4 *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    piVar12 = (int *)FUN_005fbd20(local_64,uVar2,0x280d,2);
  }
  local_4._0_1_ = 2;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 2;
  if (piVar12 != (int *)0x0) {
    puVar5 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x11;
    local_64 = puVar5;
    if (puVar5 == (undefined4 *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      iVar21 = 1;
      iVar18 = 0;
      iVar13 = *(int *)(param_1[0x54] + 0x28);
      puVar8 = (uint *)FUN_006073d0(param_1);
      uVar20 = 0x280e;
      uVar17 = 0x280d;
      uVar3 = 100;
      piVar11 = param_1;
      iVar7 = FUN_005fc0f0(piVar12);
      iVar16 = FUN_005fc0e0(piVar12);
      uVar2 = 3;
      iVar10 = FUN_005fc0e0(piVar12);
      pvVar4 = FUN_00602150(puVar5,local_6c,iVar13 - iVar10,uVar2,iVar16,iVar7,piVar11,uVar3,uVar17,
                            uVar20,puVar8,iVar18,iVar21);
    }
    local_4._0_1_ = 2;
    param_1[0x55] = (int)pvVar4;
    FUN_005ffce0(pvVar4,0);
    FUN_005fbfa0(piVar12);
    FUN_00618b60((undefined *)piVar12);
    uVar2 = local_6c;
    uVar1 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar1;
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 0x12;
  if (local_64 == (undefined4 *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    piVar12 = (int *)FUN_005fbd20(local_64,uVar2,0x27e1,2);
  }
  local_4._0_1_ = 2;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 2;
  if (piVar12 != (int *)0x0) {
    puVar5 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x13;
    local_64 = puVar5;
    if (puVar5 == (undefined4 *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      iVar10 = 1;
      iVar16 = 0;
      puVar8 = (uint *)FUN_006073d0(param_1);
      uVar20 = 0x27e0;
      uVar17 = 0x27e1;
      uVar3 = 0xc9;
      piVar11 = param_1;
      iVar13 = FUN_005fc0f0(piVar12);
      iVar7 = FUN_005fc0e0(piVar12);
      pvVar4 = FUN_00602150(puVar5,uVar2,3,3,iVar7,iVar13,piVar11,uVar3,uVar17,uVar20,puVar8,iVar16,
                            iVar10);
    }
    local_4._0_1_ = 2;
    param_1[0x56] = (int)pvVar4;
    if (pvVar4 != (void *)0x0) {
      FUN_005ffce0(pvVar4,0);
    }
    FUN_005fbfa0(piVar12);
    FUN_00618b60((undefined *)piVar12);
    uVar1 = (undefined1)local_4;
  }
  local_4._0_1_ = uVar1;
  local_64 = (undefined4 *)FUN_00618b70(0x10);
  local_4._0_1_ = 0x14;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_0060a790(local_64,4);
  }
  local_4._0_1_ = 2;
  param_1[0x5e] = (int)pvVar4;
  puVar5 = (undefined4 *)FUN_00618b70(0x118);
  local_4._0_1_ = 0x15;
  local_64 = puVar5;
  if (puVar5 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    uVar15 = 0x28;
    uVar14 = 0x45;
    uVar3 = 2;
    puVar8 = (uint *)FUN_006073d0(param_1);
    pvVar4 = FUN_00607ea0(puVar5,uVar2,8,0x4d,0xde,0xe1,param_1,puVar8,uVar3,uVar14,uVar15);
  }
  local_4._0_1_ = 2;
  param_1[0x5d] = (int)pvVar4;
  if (pvVar4 != (void *)0x0) {
    *(undefined4 *)((int)pvVar4 + 0xf4) = 0xa0000;
    (**(code **)(*(int *)param_1[0x5d] + 0x18))(10);
    *(uint *)(param_1[0x5d] + 0x50) = *(uint *)(param_1[0x5d] + 0x50) & 0xefffffff;
    FUN_005ffce0((void *)param_1[0x5d],0);
    FUN_00609340((void *)param_1[0x5d],4);
    FUN_00608280((void *)param_1[0x5d],param_1[0x5e]);
    FUN_005f4f10(param_1 + 0x1b,param_1[0x5d]);
  }
  local_64 = (undefined4 *)FUN_00618b70(200);
  local_4._0_1_ = 0x16;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_004acec0(local_64,0x38,0x7f,0xa0,4,(int)param_1,0x200f0f0,0x2000000,(uint *)0x0,0,0
                         );
  }
  param_1[0x75] = (int)pvVar4;
  *(undefined4 *)((int)pvVar4 + 0x24) = 0x6d;
  local_4._0_1_ = 2;
  FUN_005ffce0((void *)param_1[0x75],0);
  pvVar4 = (void *)FUN_00509670(local_68,0);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 0x17;
  if (local_64 == (undefined4 *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    puVar5 = FUN_004acd10(local_64);
  }
  local_4._0_1_ = 2;
  puVar8 = FUN_004025b0(pvVar4,(uint *)&local_64);
  local_4._0_1_ = 0x18;
  FUN_004f26d0(puVar5 + 6,puVar8);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  FUN_004ace40(puVar5,param_1[9]);
  piVar12 = param_1 + 0x45;
  FUN_004acba0(piVar12,puVar5,0);
  FUN_00458080(param_1,1,pvVar4);
  if (*(int *)((int)pvVar4 + 0x58) != 0) {
    FUN_00457c90(param_1,(undefined4 *)0x1,pvVar4);
  }
  local_64 = (undefined4 *)FUN_00618b70(200);
  local_4._0_1_ = 0x19;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_004acec0(local_64,0x38,0xd0,0xa0,4,(int)param_1,0x200f0f0,0x2000000,(uint *)0x0,0,0
                         );
  }
  param_1[0x83] = (int)pvVar4;
  *(undefined4 *)((int)pvVar4 + 0x24) = 0x6e;
  local_4._0_1_ = 2;
  FUN_005ffce0((void *)param_1[0x83],0);
  pvVar4 = (void *)FUN_00509670(local_68,2);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 0x1a;
  if (local_64 == (undefined4 *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    puVar5 = FUN_004acd10(local_64);
  }
  local_4._0_1_ = 2;
  puVar8 = FUN_004025b0(pvVar4,(uint *)&local_64);
  local_4._0_1_ = 0x1b;
  FUN_004f26d0(puVar5 + 6,puVar8);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  FUN_004ace40(puVar5,param_1[9]);
  FUN_004acba0(piVar12,puVar5,0);
  FUN_00458080(param_1,2,pvVar4);
  if (*(int *)((int)pvVar4 + 0x58) != 0) {
    FUN_00457c90(param_1,(undefined4 *)&DAT_00000002,pvVar4);
  }
  local_64 = (undefined4 *)FUN_00618b70(200);
  local_4._0_1_ = 0x1c;
  if (local_64 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    pvVar4 = FUN_004acec0(local_64,0x38,0x121,0xa0,4,(int)param_1,0x200f0f0,0x2000000,(uint *)0x0,0,
                          0);
  }
  param_1[0x91] = (int)pvVar4;
  *(undefined4 *)((int)pvVar4 + 0x24) = 0x6f;
  local_4._0_1_ = 2;
  FUN_005ffce0((void *)param_1[0x91],0);
  pvVar4 = (void *)FUN_00509670(local_68,1);
  local_64 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 0x1d;
  if (local_64 == (undefined4 *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    puVar5 = FUN_004acd10(local_64);
  }
  local_4._0_1_ = 2;
  puVar8 = FUN_004025b0(pvVar4,(uint *)&local_64);
  local_4._0_1_ = 0x1e;
  FUN_004f26d0(puVar5 + 6,puVar8);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  FUN_004ace40(puVar5,param_1[9]);
  FUN_004acba0(piVar12,puVar5,0);
  FUN_00458080(param_1,3,pvVar4);
  if (*(int *)((int)pvVar4 + 0x58) != 0) {
    FUN_00457c90(param_1,(undefined4 *)0x3,pvVar4);
  }
  puVar5 = (undefined4 *)FUN_00618b70(0x9c);
  local_4._0_1_ = 0x1f;
  local_64 = puVar5;
  if (puVar5 == (undefined4 *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    iVar13 = param_1[7];
    puVar8 = (uint *)FUN_006073d0(param_1);
    pvVar4 = FUN_0060d590(puVar5,iVar13,0,0x14,0xe2,0x21,0x70,param_1,puVar8);
  }
  local_4._0_1_ = 2;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 2;
  param_1[0x5b] = (int)pvVar4;
  if (pvVar4 == (void *)0x0) goto LAB_0045617d;
  FUN_005ffce0(pvVar4,0);
  iVar13 = param_1[0x52];
  if (iVar13 == 1) {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4._0_1_ = 0x20;
    if (local_64 == (undefined4 *)0x0) goto LAB_00455cc8;
    iVar13 = param_1[0x5b];
    puVar8 = *(uint **)(iVar13 + 0x98);
    uVar20 = 0x2847;
    uVar17 = 0x2848;
LAB_00455ca8:
    uVar2 = local_6c;
    local_5c = (int *)FUN_00602150(local_64,local_6c,0,0,0x24,0x21,iVar13,0x67,uVar17,uVar20,puVar8,
                                   0,1);
    local_4._0_1_ = 2;
  }
  else if (iVar13 == 2) {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    uVar2 = local_6c;
    local_4._0_1_ = 0x21;
    if (local_64 == (undefined4 *)0x0) goto LAB_00455cc8;
    local_5c = (int *)FUN_00602150(local_64,local_6c,0,0,0x24,0x21,param_1[0x5b],0x67,0x284b,0x284a,
                                   *(uint **)(param_1[0x5b] + 0x98),0,1);
    local_4._0_1_ = 2;
  }
  else {
    uVar2 = local_6c;
    if (iVar13 == 3) {
      local_64 = (undefined4 *)FUN_00618b70(0x114);
      local_4._0_1_ = 0x22;
      if (local_64 != (undefined4 *)0x0) {
        iVar13 = param_1[0x5b];
        puVar8 = *(uint **)(iVar13 + 0x98);
        uVar20 = 0x284d;
        uVar17 = 0x284e;
        goto LAB_00455ca8;
      }
LAB_00455cc8:
      local_4._0_1_ = 2;
      uVar2 = local_6c;
      local_5c = (int *)0x0;
    }
  }
  FUN_005ffce0(local_5c,0);
  local_6c = CONCAT22(DAT_0065d424,0x1835);
  FUN_00600a40(param_1,local_5c,&local_6c);
  FUN_0060d700((void *)param_1[0x5b],local_5c,1,0);
  piVar12 = local_68;
  FUN_0052d690(auStack_2c,local_68,3,param_1[0x52]);
  local_4._0_1_ = 0x23;
  iVar13 = FUN_00513180((int)auStack_2c);
  if (iVar13 == 0) {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x25);
    if (local_64 != (undefined4 *)0x0) {
      iVar13 = param_1[0x5b];
      puVar8 = *(uint **)(iVar13 + 0x98);
      uVar17 = 0x2852;
      goto LAB_00455da7;
    }
LAB_00455dbf:
    piVar11 = (int *)0x0;
  }
  else {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x24);
    if (local_64 == (undefined4 *)0x0) goto LAB_00455dbf;
    iVar13 = param_1[0x5b];
    puVar8 = *(uint **)(iVar13 + 0x98);
    uVar17 = 0x2851;
LAB_00455da7:
    piVar11 = (int *)FUN_00602150(local_64,uVar2,0xbe,0,0x24,0x21,iVar13,0x6c,uVar17,0x2850,puVar8,0
                                  ,1);
  }
  local_4._0_1_ = 0x23;
  FUN_005ffce0(piVar11,0);
  local_68 = (int *)CONCAT22(DAT_0065d424,0x1826);
  FUN_00600a40(param_1,piVar11,&local_68);
  FUN_0060d700((void *)param_1[0x5b],piVar11,0,0);
  FUN_0052d270(&local_58,piVar12,3,param_1[0x52]);
  local_4._0_1_ = 0x26;
  iVar13 = FUN_00513180((int)&local_58);
  if (iVar13 == 0) {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x28);
    if (local_64 != (undefined4 *)0x0) {
      iVar13 = param_1[0x5b];
      puVar8 = *(uint **)(iVar13 + 0x98);
      uVar17 = 0x2855;
      goto LAB_00455e8c;
    }
LAB_00455ea4:
    piVar11 = (int *)0x0;
  }
  else {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x27);
    if (local_64 == (undefined4 *)0x0) goto LAB_00455ea4;
    iVar13 = param_1[0x5b];
    puVar8 = *(uint **)(iVar13 + 0x98);
    uVar17 = 0x2854;
LAB_00455e8c:
    piVar11 = (int *)FUN_00602150(local_64,uVar2,0x98,0,0x24,0x21,iVar13,0x6b,uVar17,0x2853,puVar8,0
                                  ,1);
  }
  local_4._0_1_ = 0x26;
  FUN_005ffce0(piVar11,0);
  local_68 = (int *)CONCAT22(DAT_0065d424,0x1827);
  FUN_00600a40(param_1,piVar11,&local_68);
  FUN_0060d700((void *)param_1[0x5b],piVar11,0,0);
  iVar13 = FUN_0052c8c0(piVar12,(uint)piVar12[9] >> 6 & 3,3);
  if (iVar13 == 0) {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x2a);
    if (local_64 != (undefined4 *)0x0) {
      iVar13 = param_1[0x5b];
      puVar8 = *(uint **)(iVar13 + 0x98);
      uVar17 = 0x2858;
      goto LAB_00455f65;
    }
LAB_00455f7a:
    piVar11 = (int *)0x0;
  }
  else {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x29);
    if (local_64 == (undefined4 *)0x0) goto LAB_00455f7a;
    iVar13 = param_1[0x5b];
    puVar8 = *(uint **)(iVar13 + 0x98);
    uVar17 = 0x2857;
LAB_00455f65:
    piVar11 = (int *)FUN_00602150(local_64,uVar2,0x27,0,0x24,0x21,iVar13,0x68,uVar17,0x2856,puVar8,0
                                  ,1);
  }
  local_4._0_1_ = 0x26;
  FUN_005ffce0(piVar11,0);
  local_68 = (int *)CONCAT22(DAT_0065d424,0x1828);
  FUN_00600a40(param_1,piVar11,&local_68);
  FUN_0060d700((void *)param_1[0x5b],piVar11,0,0);
  iVar13 = FUN_0052c5a0(piVar12,param_1[0x52],3);
  if (iVar13 == 0) {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x2c);
    if (local_64 != (undefined4 *)0x0) {
      iVar13 = param_1[0x5b];
      puVar8 = *(uint **)(iVar13 + 0x98);
      uVar17 = 0x285b;
      goto LAB_00456038;
    }
LAB_0045604d:
    piVar11 = (int *)0x0;
  }
  else {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x2b);
    if (local_64 == (undefined4 *)0x0) goto LAB_0045604d;
    iVar13 = param_1[0x5b];
    puVar8 = *(uint **)(iVar13 + 0x98);
    uVar17 = 0x285a;
LAB_00456038:
    piVar11 = (int *)FUN_00602150(local_64,uVar2,0x4d,0,0x24,0x21,iVar13,0x69,uVar17,0x2859,puVar8,0
                                  ,1);
  }
  local_4._0_1_ = 0x26;
  FUN_005ffce0(piVar11,0);
  local_68 = (int *)CONCAT22(DAT_0065d424,0x1830);
  FUN_00600a40(param_1,piVar11,&local_68);
  FUN_0060d700((void *)param_1[0x5b],piVar11,0,0);
  iVar13 = FUN_0052c270(piVar12,param_1[0x52],3);
  if (iVar13 == 0) {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x2e);
    if (local_64 != (undefined4 *)0x0) {
      iVar13 = param_1[0x5b];
      puVar8 = *(uint **)(iVar13 + 0x98);
      uVar17 = 0x285e;
      goto LAB_0045610b;
    }
LAB_00456120:
    piVar12 = (int *)0x0;
  }
  else {
    local_64 = (undefined4 *)FUN_00618b70(0x114);
    local_4 = CONCAT31(local_4._1_3_,0x2d);
    if (local_64 == (undefined4 *)0x0) goto LAB_00456120;
    iVar13 = param_1[0x5b];
    puVar8 = *(uint **)(iVar13 + 0x98);
    uVar17 = 0x285d;
LAB_0045610b:
    piVar12 = (int *)FUN_00602150(local_64,uVar2,0x73,0,0x24,0x21,iVar13,0x6a,uVar17,0x285c,puVar8,0
                                  ,1);
  }
  local_4._0_1_ = 0x26;
  FUN_005ffce0(piVar12,0);
  local_68 = (int *)CONCAT22(DAT_0065d424,0x1832);
  FUN_00600a40(param_1,piVar12,&local_68);
  FUN_0060d700((void *)param_1[0x5b],piVar12,0,0);
  local_4._0_1_ = 0x23;
  FUN_0052d2f0(&local_58);
  local_4._0_1_ = 2;
  FUN_0052d710(auStack_2c);
  uVar1 = (undefined1)local_4;
LAB_0045617d:
  local_4._0_1_ = uVar1;
  FUN_0060d7e0((void *)param_1[0x5b],0x67,1);
  if ((*(byte *)(param_1[0x53] + 0xc0) & 4) == 0) {
    FUN_00610b20(&local_58,0,0,0,0,0,0);
    uStack_48 = uStack_48 & 0xfffffff9 | 1;
    local_4._0_1_ = 0x2f;
    uStack_54 = 0x25e;
    uStack_50 = 7;
    FUN_00610c30((int)&local_58);
    local_58 = &PTR_LAB_00658518;
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_38);
  ExceptionList = pvStack_c;
  return;
}

