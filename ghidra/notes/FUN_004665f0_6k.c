
void __fastcall FUN_004665f0(void *param_1)

{
  bool bVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  uint *puVar4;
  undefined4 *puVar5;
  void *pvVar6;
  int iVar7;
  int iVar8;
  uint *puVar9;
  uint *puVar10;
  uint *puVar11;
  int *piVar12;
  void *pvVar13;
  uint uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  int iVar17;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 *local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631b5d;
  pvStack_c = ExceptionList;
  local_14 = *(undefined4 *)((int)param_1 + 0x1c);
  ExceptionList = &pvStack_c;
  uVar3 = FUN_006037f0(7);
  iVar7 = *(int *)((int)param_1 + 0x114);
  local_24 = uVar3;
  FUN_00606980(param_1,*(int *)(iVar7 + 0xcc),*(int *)(iVar7 + 0xd0),*(int *)(iVar7 + 0xd4),
               *(int *)(iVar7 + 0xd8));
  local_10 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (local_10 == (void *)0x0) {
    local_20 = (undefined4 *)0x0;
  }
  else {
    local_20 = (undefined4 *)FUN_005fbd20(local_10,uVar3,0x2a46,10);
  }
  local_4 = 0xffffffff;
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (local_10 == (void *)0x0) {
      puVar4 = (uint *)0x0;
    }
    else {
      puVar4 = (uint *)FUN_005fbd20(local_10,uVar3,0x285f,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (local_10 != (void *)0x0) {
      puVar5 = (undefined4 *)FUN_005fbd20(local_10,uVar3,0x2a44,10);
      goto LAB_0046675f;
    }
  }
  else {
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (local_10 == (void *)0x0) {
      puVar4 = (uint *)0x0;
    }
    else {
      puVar4 = (uint *)FUN_005fbd20(local_10,uVar3,0x2860,10);
    }
    local_4 = 0xffffffff;
    local_10 = (void *)FUN_00618b70(0x28);
    local_4 = 4;
    if (local_10 != (void *)0x0) {
      puVar5 = (undefined4 *)FUN_005fbd20(local_10,uVar3,0x2a45,10);
      goto LAB_0046675f;
    }
  }
  puVar5 = (undefined4 *)0x0;
LAB_0046675f:
  local_4 = 0xffffffff;
  local_1c = puVar5;
  pvVar6 = (void *)FUN_00618b70(0x28);
  local_4 = 5;
  local_10 = pvVar6;
  if (pvVar6 == (void *)0x0) {
    puVar9 = (uint *)0x0;
  }
  else {
    puVar9 = puVar4;
    iVar7 = FUN_005fc0f0((int *)puVar4);
    iVar8 = FUN_005fc0e0((int *)puVar4);
    puVar9 = (uint *)FUN_005fbda0(pvVar6,iVar8,iVar7,puVar9);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(local_20,puVar9,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar4,puVar9,0,0);
  FUN_005fd0f0(puVar5,puVar9,0x19c,0);
  FUN_006075b0(param_1,(int *)puVar9,0x32);
  puVar2 = local_20;
  if (local_20 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_20);
    FUN_00618b60((undefined *)puVar2);
  }
  local_10 = (void *)FUN_00618b70(0x28);
  local_4 = 6;
  if (local_10 == (void *)0x0) {
    local_18 = (undefined4 *)0x0;
  }
  else {
    local_18 = (undefined4 *)FUN_005fbd20(local_10,local_24,0x2a47,10);
  }
  local_4 = 0xffffffff;
  pvVar6 = (void *)FUN_00618b70(0x28);
  local_4 = 7;
  local_10 = pvVar6;
  if (pvVar6 == (void *)0x0) {
    puVar9 = (uint *)0x0;
  }
  else {
    puVar9 = puVar4;
    iVar7 = FUN_005fc0f0((int *)puVar4);
    iVar8 = FUN_005fc0e0((int *)puVar4);
    puVar9 = (uint *)FUN_005fbda0(pvVar6,iVar8,iVar7,puVar9);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(local_18,puVar9,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar4,puVar9,0,0);
  FUN_005fd0f0(puVar5,puVar9,0x19c,0);
  local_10 = (void *)FUN_00618b70(0x28);
  local_4 = 8;
  if (local_10 == (void *)0x0) {
    puVar10 = (uint *)0x0;
  }
  else {
    puVar10 = (uint *)FUN_005fbd20(local_10,local_24,0x2a4b,2);
  }
  local_4 = 0xffffffff;
  local_20 = (undefined4 *)FUN_00618b70(0x28);
  local_4 = 9;
  if (local_20 == (undefined4 *)0x0) {
    puVar11 = (uint *)0x0;
  }
  else {
    puVar11 = puVar10;
    iVar7 = FUN_005fc0f0((int *)puVar10);
    iVar8 = FUN_005fc0e0((int *)puVar10);
    puVar11 = (uint *)FUN_005fbda0(local_20,iVar8,iVar7,puVar11);
  }
  local_4 = 0xffffffff;
  *(uint **)((int)param_1 + 0x164) = puVar11;
  FUN_005fcc30(puVar9,puVar11,0,0,0x16,0x6c,(undefined4 *)0x0,0);
  if (puVar10 != (uint *)0x0) {
    FUN_005fbfa0(puVar10);
    FUN_00618b60((undefined *)puVar10);
  }
  FUN_00607610(param_1,(int *)puVar9,0x33);
  puVar5 = local_18;
  if (local_18 != (undefined4 *)0x0) {
    FUN_005fbfa0(local_18);
    FUN_00618b60((undefined *)puVar5);
  }
  pvVar6 = (void *)FUN_00618b70(0x28);
  local_4 = 10;
  local_10 = pvVar6;
  if (pvVar6 == (void *)0x0) {
    puVar9 = (uint *)0x0;
  }
  else {
    puVar9 = puVar4;
    iVar7 = FUN_005fc0f0((int *)puVar4);
    iVar8 = FUN_005fc0e0((int *)puVar4);
    puVar9 = (uint *)FUN_005fbda0(pvVar6,iVar8,iVar7,puVar9);
  }
  local_4 = 0xffffffff;
  local_10 = (void *)FUN_00618b70(0x28);
  local_4 = 0xb;
  if (local_10 == (void *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    puVar5 = (undefined4 *)FUN_005fbd20(local_10,local_24,0x2ab4,2);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(puVar5,puVar9,0xc,0xe8,0,0,(undefined4 *)0x0,0);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  local_10 = (void *)FUN_00618b70(0x28);
  local_4 = 0xc;
  if (local_10 == (void *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    puVar5 = (undefined4 *)FUN_005fbd20(local_10,local_24,0x2ab5,2);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(puVar5,puVar9,0xc,0xe,0,0,(undefined4 *)0x0,0);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  FUN_005fd0f0(puVar4,puVar9,0,0);
  puVar5 = local_1c;
  FUN_005fd0f0(local_1c,puVar9,0x19c,0);
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  if (puVar4 != (uint *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  FUN_00607610(param_1,(int *)puVar9,0x34);
  *(undefined4 *)((int)param_1 + 0x174) = 0x18b;
  *(undefined4 *)((int)param_1 + 0x1a0) = 1;
  FUN_00601c90((undefined4 *)((int)param_1 + 0x174),0xffffff,(HWND)0x0);
  pvVar6 = (void *)((int)param_1 + 0x1a8);
  FUN_00601b30(pvVar6,0x23,0x5a);
  FUN_00601c60(pvVar6,4);
  FUN_00601c90(pvVar6,0xffffff,(HWND)0x0);
  FUN_00403e90(pvVar6,0);
  local_10 = (void *)FUN_00618b70(0x10);
  local_4 = 0xd;
  if (local_10 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_004ad620(local_10,4);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x16c) = pvVar6;
  pvVar6 = (void *)FUN_00618b70(0x118);
  local_4 = 0xe;
  local_10 = pvVar6;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    uVar16 = 0x15;
    uVar15 = 0x15a;
    uVar3 = 0x6e;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar6 = FUN_00607ea0(pvVar6,local_14,0x19,0x6c,0x170,0xc2,param_1,puVar4,uVar3,uVar15,uVar16);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x168) = pvVar6;
  if (pvVar6 != (void *)0x0) {
    *(uint *)((int)pvVar6 + 0x50) = *(uint *)((int)pvVar6 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x168),0);
    *(uint *)(*(int *)((int)param_1 + 0x168) + 0xf4) =
         *(uint *)(*(int *)((int)param_1 + 0x168) + 0xf4) | 0x20000;
    iVar7 = *(int *)((int)param_1 + 0x168);
    FUN_0060a490((void *)(iVar7 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar7,7,0x299d,
                 *(undefined4 *)(iVar7 + 0x3c),0,*(undefined4 *)(iVar7 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 0x168) + 0xe4) = 0;
    FUN_00608280(*(void **)((int)param_1 + 0x168),*(undefined4 *)((int)param_1 + 0x16c));
  }
  pvVar6 = (void *)FUN_00618b70(0x9c);
  local_4 = 0xf;
  local_10 = pvVar6;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar6 = FUN_0060d590(pvVar6,local_14,0x16,0x2e,0x17a,0x29,0x78,param_1,puVar4);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x160) = pvVar6;
  if (pvVar6 != (void *)0x0) {
    FUN_005ffce0(pvVar6,0);
    local_10 = (void *)FUN_00618b70(0x114);
    uVar3 = local_24;
    local_4 = 0x10;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,local_24,0,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x79,0x2a4e,0x2a4f,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8010);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,1,0);
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      uVar14 = 0x2a50;
      local_28 = 0x2a51;
    }
    else {
      uVar14 = 0x2a52;
      local_28 = 0x2a53;
    }
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x11;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0x26,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x7a,uVar14,local_28,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8011);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      uVar14 = 0x2a58;
      local_28 = 0x2a59;
    }
    else {
      uVar14 = 0x2a56;
      local_28 = 0x2a57;
    }
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x12;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0x4c,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x7b,uVar14,local_28,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8017);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      uVar14 = 0x2a5a;
      local_28 = 0x2a5b;
    }
    else {
      uVar14 = 0x2a5c;
      local_28 = 0x2a5d;
    }
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x13;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0x72,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x7c,uVar14,local_28,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8014);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x14;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0x97,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x7d,0x2a54,0x2a55,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8012);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x15;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0xbd,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x7e,0x2a64,0x2a65,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8013);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x16;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0xe3,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x7f,0x2a68,0x2a69,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8016);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x17;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0x107,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x80,0x2a66,0x2a67,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8018);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x18;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0x12d,0,0x24,0x29,*(int *)((int)param_1 + 0x160),
                                    0x81,0x2a62,0x2a63,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8015);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      uVar14 = 0x2a5e;
      local_28 = 0x2a5f;
    }
    else {
      uVar14 = 0x2a60;
      local_28 = 0x2a61;
    }
    local_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x19;
    if (local_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(local_10,uVar3,0x152,0,0x25,0x29,*(int *)((int)param_1 + 0x160),
                                    0x82,uVar14,local_28,
                                    *(uint **)(*(int *)((int)param_1 + 0x160) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8032);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_006030c0(piVar12,0x80);
    FUN_006030f0(piVar12,0x10);
    FUN_0060d700(*(void **)((int)param_1 + 0x160),piVar12,0,1);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    uVar15 = 0x1a7;
    uVar3 = 0x19;
    iVar7 = 0x20;
    local_20 = (undefined4 *)0x1f;
    local_18 = (undefined4 *)0x2882;
    local_28 = 0x2883;
  }
  else {
    uVar15 = 0x1aa;
    uVar3 = 0x15;
    iVar7 = 0x2c;
    local_20 = (undefined4 *)0x29;
    local_18 = (undefined4 *)0x2888;
    local_28 = 0x2889;
  }
  local_1c = (undefined4 *)FUN_00618b70(0x114);
  local_4 = 0x1a;
  if (local_1c == (undefined4 *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    iVar17 = 0;
    iVar8 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar6 = FUN_00602150(local_1c,local_24,uVar15,uVar3,iVar7,(int)local_20,param_1,0x28,
                          (uint)local_18,local_28,puVar4,iVar8,iVar17);
  }
  local_4 = 0xffffffff;
  if (pvVar6 != (void *)0x0) {
    FUN_005ffce0(pvVar6,0);
    FUN_00602840(pvVar6,0x260);
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x1954);
    FUN_00600a40(param_1,pvVar6,&local_18);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar6);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    uVar15 = 0x1a7;
    uVar3 = 0x5d;
    local_1c = (undefined4 *)0x20;
    local_20 = (undefined4 *)0x1f;
    uVar14 = 0x2884;
    local_28 = 0x2885;
  }
  else {
    uVar15 = 0x1aa;
    uVar3 = 0x59;
    local_1c = (undefined4 *)0x2c;
    local_20 = (undefined4 *)0x29;
    uVar14 = 0x288a;
    local_28 = 0x288b;
  }
  local_18 = (undefined4 *)FUN_00618b70(0x114);
  local_4 = 0x1b;
  if (local_18 == (undefined4 *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar6 = FUN_00602150(local_18,local_24,uVar15,uVar3,(int)local_1c,(int)local_20,param_1,0x65,
                          uVar14,local_28,puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  FUN_005ffce0(pvVar6,0);
  FUN_00602840(pvVar6,0x260);
  local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8020);
  FUN_00600a40(param_1,pvVar6,&local_18);
  FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar6);
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    uVar15 = 0x1a7;
    uVar3 = 0x93;
    uVar14 = 0x2a78;
    local_28 = 0x2a79;
  }
  else {
    uVar15 = 0x1aa;
    uVar3 = 0x94;
    uVar14 = 0x2a7a;
    local_28 = 0x2a7b;
  }
  local_18 = (undefined4 *)FUN_00618b70(0x114);
  local_4 = 0x1c;
  if (local_18 == (undefined4 *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar6 = FUN_00602150(local_18,local_24,uVar15,uVar3,(int)local_1c,(int)local_20,param_1,0x66,
                          uVar14,local_28,puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  FUN_005ffce0(pvVar6,0);
  FUN_00602840(pvVar6,0x260);
  local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8005);
  FUN_00600a40(param_1,pvVar6,&local_18);
  FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar6);
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    uVar15 = 0x1a7;
    uVar3 = 0xc9;
    uVar14 = 0x2a78;
    local_28 = 0x2a79;
    local_18 = (undefined4 *)0x2acf;
  }
  else {
    uVar15 = 0x1aa;
    uVar3 = 0xcf;
    uVar14 = 0x2a7a;
    local_28 = 0x2a7b;
    local_18 = (undefined4 *)0x2ad0;
  }
  local_10 = (void *)FUN_00618b70(0x114);
  local_4 = 0x1d;
  if (local_10 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    pvVar6 = FUN_00602150(local_10,local_24,uVar15,uVar3,(int)local_1c,(int)local_20,param_1,0x67,
                          uVar14,local_28,puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  FUN_005ffce0(pvVar6,0);
  FUN_00602840(pvVar6,0x260);
  local_28 = CONCAT22(DAT_0065d424,0x8030);
  FUN_00600a40(param_1,pvVar6,&local_28);
  FUN_00603150(pvVar6,2,(uint)local_18);
  FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar6);
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    uVar15 = 0x1a7;
    uVar3 = 0xff;
    uVar14 = 0x2a7c;
    local_28 = 0x2a7d;
    local_18 = (undefined4 *)0x2a7e;
  }
  else {
    uVar15 = 0x1aa;
    uVar3 = 0x10a;
    uVar14 = 0x2a7f;
    local_28 = 0x2a80;
    local_18 = (undefined4 *)0x2a81;
  }
  local_10 = (void *)FUN_00618b70(0x114);
  local_4 = 0x1e;
  if (local_10 == (void *)0x0) {
    piVar12 = (int *)0x0;
    uVar16 = local_24;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    uVar16 = local_24;
    piVar12 = (int *)FUN_00602150(local_10,local_24,uVar15,uVar3,(int)local_1c,(int)local_20,param_1
                                  ,0x68,uVar14,local_28,puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  FUN_005ffce0(piVar12,0);
  FUN_00602840(piVar12,0x260);
  local_1c = (undefined4 *)CONCAT22(DAT_0065d424,0x8022);
  FUN_00600a40(param_1,piVar12,&local_1c);
  pvVar6 = (void *)((int)param_1 + 0x6c);
  FUN_005f4f10(pvVar6,(int)piVar12);
  iVar7 = FUN_00401080();
  if (iVar7 == 1) {
    FUN_00603150(piVar12,2,(uint)local_18);
    (**(code **)(*piVar12 + 0x10))();
  }
  pvVar13 = (void *)FUN_00618b70(0x114);
  local_4 = 0x1f;
  local_10 = pvVar13;
  if (pvVar13 == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    piVar12 = (int *)FUN_00602150(pvVar13,uVar16,0x11a,0x57,0x38,0x14,param_1,0x90,0x2a94,0x2a95,
                                  puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  if (piVar12 != (int *)0x0) {
    piVar12[0x14] = piVar12[0x14] & 0xefffffff;
    FUN_006030c0(piVar12,0x80);
    FUN_005ffce0(piVar12,0);
    (**(code **)(*piVar12 + 8))();
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8008);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_005f4f10(pvVar6,(int)piVar12);
  }
  pvVar13 = (void *)FUN_00618b70(0x114);
  local_4 = 0x20;
  local_10 = pvVar13;
  if (pvVar13 == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006073d0(param_1);
    piVar12 = (int *)FUN_00602150(pvVar13,uVar16,0x154,0x57,0x38,0x14,param_1,0x91,0x2a96,0x2a97,
                                  puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  if (piVar12 != (int *)0x0) {
    piVar12[0x14] = piVar12[0x14] & 0xefffffff;
    FUN_006030c0(piVar12,0x80);
    FUN_005ffce0(piVar12,0);
    (**(code **)(*piVar12 + 8))();
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8009);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_005f4f10(pvVar6,(int)piVar12);
  }
  pvVar13 = (void *)FUN_00618b70(0x114);
  local_4 = 0x21;
  local_10 = pvVar13;
  if (pvVar13 == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006076b0(param_1,0x34);
    piVar12 = (int *)FUN_00602150(pvVar13,uVar16,0x186,0xf,0x13,0xf,param_1,0x96,0x2aa7,0x2aa8,
                                  puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  if (piVar12 != (int *)0x0) {
    FUN_00603150(piVar12,2,0x2aa9);
    piVar12[0x14] = piVar12[0x14] & 0xefffffff;
    FUN_005ffce0(piVar12,0);
    (**(code **)(*piVar12 + 8))();
    FUN_005f4f10(pvVar6,(int)piVar12);
  }
  pvVar13 = (void *)FUN_00618b70(0x114);
  local_4 = 0x22;
  local_10 = pvVar13;
  if (pvVar13 == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006076b0(param_1,0x34);
    piVar12 = (int *)FUN_00602150(pvVar13,uVar16,0x16f,0xf,0x13,0xf,param_1,0x9a,0x2ac4,0x2ac5,
                                  puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  if (piVar12 != (int *)0x0) {
    FUN_00603150(piVar12,2,0x2ac6);
    piVar12[0x14] = piVar12[0x14] & 0xefffffff;
    FUN_005ffce0(piVar12,0);
    (**(code **)(*piVar12 + 8))();
    FUN_005f4f10(pvVar6,(int)piVar12);
  }
  pvVar13 = (void *)FUN_00618b70(0x114);
  local_4 = 0x23;
  local_10 = pvVar13;
  if (pvVar13 == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006076b0(param_1,0x34);
    piVar12 = (int *)FUN_00602150(pvVar13,uVar16,0x163,0xf4,0x33,0x23,param_1,0x98,0x2ab6,0x2ab7,
                                  puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  if (piVar12 != (int *)0x0) {
    piVar12[0x14] = piVar12[0x14] & 0xefffffff;
    FUN_005ffce0(piVar12,0);
    (**(code **)(*piVar12 + 8))();
    FUN_005f4f10(pvVar6,(int)piVar12);
  }
  pvVar13 = (void *)FUN_00618b70(0x114);
  local_4 = 0x24;
  local_10 = pvVar13;
  if (pvVar13 == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006076b0(param_1,0x34);
    piVar12 = (int *)FUN_00602150(pvVar13,uVar16,0x163,0x119,0x33,0x23,param_1,0x99,0x2aba,0x2abb,
                                  puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  if (piVar12 != (int *)0x0) {
    piVar12[0x14] = piVar12[0x14] & 0xefffffff;
    FUN_005ffce0(piVar12,0);
    (**(code **)(*piVar12 + 8))();
    FUN_005f4f10(pvVar6,(int)piVar12);
  }
  pvVar13 = (void *)FUN_00618b70(0x114);
  local_4 = 0x25;
  local_10 = pvVar13;
  if (pvVar13 == (void *)0x0) {
    piVar12 = (int *)0x0;
  }
  else {
    iVar8 = 0;
    iVar7 = 0;
    puVar4 = (uint *)FUN_006076b0(param_1,0x34);
    piVar12 = (int *)FUN_00602150(pvVar13,uVar16,0x160,0xff,0x33,0x34,param_1,0x97,0x2aba,0x2abb,
                                  puVar4,iVar7,iVar8);
  }
  local_4 = 0xffffffff;
  if (piVar12 != (int *)0x0) {
    piVar12[0x14] = piVar12[0x14] & 0xefffffff;
    FUN_005ffce0(piVar12,0);
    (**(code **)(*piVar12 + 8))();
    local_18 = (undefined4 *)CONCAT22(DAT_0065d424,0x8030);
    FUN_00600a40(param_1,piVar12,&local_18);
    FUN_005f4f10(pvVar6,(int)piVar12);
  }
  pvVar6 = (void *)FUN_00618b70(0x178);
  local_4 = 0x26;
  local_10 = pvVar6;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    iVar7 = 0;
    puVar4 = (uint *)FUN_006076b0(param_1,0x34);
    pvVar6 = FUN_0041ecf0(pvVar6,local_14,0x11,0xea,0x18b,0x50,param_1,puVar4,iVar7);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x1dc) = pvVar6;
  if (pvVar6 != (void *)0x0) {
    *(uint *)((int)pvVar6 + 0x50) = *(uint *)((int)pvVar6 + 0x50) & 0xefffffff;
    *(undefined4 *)(*(int *)((int)param_1 + 0x1dc) + 0x24) = 0xa0;
    iVar7 = *(int *)((int)param_1 + 0x1dc);
    *(undefined2 *)(iVar7 + 0x134) = 7;
    *(undefined2 *)(iVar7 + 0x136) = 0x299d;
    FUN_00420550(*(void **)((int)param_1 + 0x1dc),4);
    FUN_005ffce0(*(void **)((int)param_1 + 0x1dc),0);
  }
  FUN_0041d1f0(1);
  iVar7 = *(int *)((int)param_1 + 0x118);
  uVar14 = *(uint *)((int)param_1 + 0x11c);
  *(undefined4 *)((int)param_1 + 0x118) = 0;
  *(undefined4 *)((int)param_1 + 0x11c) = 0;
  FUN_00468fb0(param_1,iVar7);
  FUN_0060d7e0(*(void **)((int)param_1 + 0x160),uVar14,0);
  FUN_004697b0(param_1,uVar14);
  FUN_0041d210(1);
  if (DAT_006b2914 != 0) {
    piVar12 = (int *)(**(code **)(**(int **)((int)param_1 + 0x16c) + 8))();
    bVar1 = false;
    piVar12[0xf] = piVar12[0xf] & 0xfffffffe;
    while (piVar12 != (int *)0x0) {
      if (bVar1) goto LAB_00467e39;
      if (*(int *)(piVar12[0x1a] + 0x18) == DAT_006b2904) {
        bVar1 = true;
      }
      else {
        piVar12 = (int *)(**(code **)(*piVar12 + 0xc))();
      }
    }
    if (bVar1) {
LAB_00467e39:
      piVar12[0xf] = piVar12[0xf] | 1;
      SetFocus(*(HWND *)(*(int *)((int)param_1 + 0x168) + 0x18));
      FUN_0060a280(*(void **)((int)param_1 + 0x168));
    }
  }
  FUN_00469db0(DAT_006b2910);
  ExceptionList = pvStack_c;
  return;
}

