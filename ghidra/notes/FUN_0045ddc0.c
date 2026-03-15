
void __fastcall FUN_0045ddc0(void *param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  uint *puVar3;
  undefined4 *this;
  HDC pHVar4;
  HDC hdc;
  void *pvVar5;
  int iVar6;
  int iVar7;
  uint *puVar8;
  uint *puVar9;
  HGDIOBJ pvVar10;
  undefined *this_00;
  void *pvVar11;
  int *piVar12;
  uint uVar13;
  undefined4 *puStack_24;
  undefined4 *puStack_20;
  undefined4 local_18;
  undefined4 local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630e6b;
  pvStack_c = ExceptionList;
  uVar1 = *(undefined4 *)((int)param_1 + 0x1c);
  ExceptionList = &pvStack_c;
  uVar2 = FUN_006037f0(7);
  iVar6 = *(int *)((int)param_1 + 0x160);
  FUN_00606980(param_1,*(int *)(iVar6 + 0xcc),*(int *)(iVar6 + 0xd0),*(int *)(iVar6 + 0xd4),
               *(int *)(iVar6 + 0xd8));
  if (*(int *)(*(int *)((int)param_1 + 0x160) + 0x9c) == 1) {
    local_14 = (uint *)FUN_00618b70(0x28);
    local_4 = 0;
    if (local_14 == (uint *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = (uint *)FUN_005fbd20(local_14,uVar2,0x285f,10);
    }
    local_4 = 0xffffffff;
    local_14 = puVar3;
    local_18 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (local_18 == (void *)0x0) {
LAB_0045defa:
      this = (undefined4 *)0x0;
    }
    else {
      this = (undefined4 *)FUN_005fbd20(local_18,uVar2,0x2959,10);
    }
  }
  else {
    local_14 = (uint *)FUN_00618b70(0x28);
    local_4 = 2;
    if (local_14 == (uint *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = (uint *)FUN_005fbd20(local_14,uVar2,0x2860,10);
    }
    local_4 = 0xffffffff;
    local_14 = puVar3;
    local_18 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (local_18 == (void *)0x0) goto LAB_0045defa;
    this = (undefined4 *)FUN_005fbd20(local_18,uVar2,0x295d,10);
  }
  local_4 = 0xffffffff;
  pHVar4 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar4);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar4);
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4 = 4;
  local_18 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar8 = (uint *)0x0;
  }
  else {
    puVar8 = puVar3;
    iVar6 = FUN_005fc0f0((int *)puVar3);
    iVar7 = FUN_005fc0e0((int *)puVar3);
    puVar8 = (uint *)FUN_005fbda0(pvVar5,iVar7,iVar6,puVar8);
  }
  local_4 = 0xffffffff;
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4 = 5;
  local_18 = pvVar5;
  if (pvVar5 == (void *)0x0) {
    puVar9 = (uint *)0x0;
  }
  else {
    puVar9 = puVar3;
    pHVar4 = hdc;
    iVar6 = FUN_005fc0f0((int *)puVar3);
    iVar7 = FUN_005fc0e0((int *)puVar3);
    puVar9 = (uint *)FUN_005fbea0(pvVar5,iVar7,iVar6,puVar9,pHVar4);
  }
  local_4 = 0xffffffff;
  local_18 = (void *)FUN_00618b70(0x28);
  local_4 = 6;
  if (local_18 == (void *)0x0) {
    puStack_24 = (undefined4 *)0x0;
  }
  else {
    puStack_24 = (undefined4 *)FUN_005fbd20(local_18,uVar2,0x2861,10);
  }
  local_4 = 0xffffffff;
  local_18 = (void *)FUN_00618b70(0x28);
  local_4 = 7;
  if (local_18 == (void *)0x0) {
    puStack_20 = (undefined4 *)0x0;
  }
  else {
    puStack_20 = (undefined4 *)FUN_005fbd20(local_18,uVar2,0x2862,10);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(puStack_24,puVar8,0xc,0xe,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar3,puVar8,0,0);
  FUN_005fcc30(this,puVar8,0x19c,0,0,0,(undefined4 *)0x0,0);
  FUN_00607610(param_1,(int *)puVar8,1);
  FUN_005fcc30(puStack_20,puVar9,0xc,0xd,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(puVar3,puVar9,0,0);
  FUN_005fcc30(this,puVar9,0x19c,0,0,0,(undefined4 *)0x0,0);
  pvVar10 = (HGDIOBJ)FUN_005fc130((int)puVar9);
  pvVar10 = SelectObject(hdc,pvVar10);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 8;
  if (pvStack_10 == (void *)0x0) {
    this_00 = (undefined *)0x0;
  }
  else {
    local_18 = (void *)CONCAT22(DAT_0065d424,0x1843);
    this_00 = (undefined *)FUN_00601700(pvStack_10,&local_18,0x24,0x30,0,0,0,0);
  }
  local_4 = 0xffffffff;
  FUN_00601c60(this_00,4);
  FUN_00601c90(this_00,0x2ffffff,(HWND)0x0);
  FUN_00601ce0(this_00,hdc);
  FUN_00607610(param_1,(int *)puVar9,2);
  SelectObject(hdc,pvVar10);
  DeleteDC(hdc);
  if (this_00 != (undefined *)0x0) {
    FUN_00601990((int)this_00);
    FUN_00618b60(this_00);
  }
  puVar3 = local_14;
  if (local_14 != (uint *)0x0) {
    FUN_005fbfa0(local_14);
    FUN_00618b60((undefined *)puVar3);
  }
  if (this != (undefined4 *)0x0) {
    FUN_005fbfa0(this);
    FUN_00618b60((undefined *)this);
  }
  if (puStack_24 != (undefined4 *)0x0) {
    FUN_005fbfa0(puStack_24);
    FUN_00618b60((undefined *)puStack_24);
  }
  if (puStack_20 != (undefined4 *)0x0) {
    FUN_005fbfa0(puStack_20);
    FUN_00618b60((undefined *)puStack_20);
  }
  if (*(int *)((int)param_1 + 0x114) == 1) {
    iVar6 = 2;
  }
  else {
    iVar6 = 1;
  }
  FUN_006075e0(param_1,iVar6);
  if (*(int *)(*(int *)((int)param_1 + 0x160) + 0x9c) == 1) {
    pvVar5 = (void *)FUN_00618b70(0x114);
    local_4 = 9;
    pvStack_10 = pvVar5;
    if (pvVar5 != (void *)0x0) {
      iVar7 = 0;
      iVar6 = 0;
      puVar3 = (uint *)FUN_006073d0(param_1);
      pvVar5 = FUN_00602150(pvVar5,uVar2,0x1a7,0x19,0x20,0x1f,param_1,0xfb,0x2882,0x2883,puVar3,
                            iVar6,iVar7);
      goto LAB_0045e2d1;
    }
  }
  else {
    pvVar5 = (void *)FUN_00618b70(0x114);
    local_4 = 10;
    pvStack_10 = pvVar5;
    if (pvVar5 != (void *)0x0) {
      iVar7 = 0;
      iVar6 = 0;
      puVar3 = (uint *)FUN_006073d0(param_1);
      pvVar5 = FUN_00602150(pvVar5,uVar2,0x1aa,0x15,0x2c,0x29,param_1,0xfb,0x2888,0x2889,puVar3,
                            iVar6,iVar7);
      goto LAB_0045e2d1;
    }
  }
  pvVar5 = (void *)0x0;
LAB_0045e2d1:
  local_4 = 0xffffffff;
  FUN_005ffce0(pvVar5,0);
  FUN_00602840(pvVar5,0x260);
  local_14 = (uint *)CONCAT22(DAT_0065d424,0x1954);
  FUN_00600a40(param_1,pvVar5,&local_14);
  FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar5);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 0xb;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1842);
    pvVar11 = FUN_00601700(pvStack_10,&local_14,0,0,0,0,0,0);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x120) = pvVar11;
  if (pvVar11 != (void *)0x0) {
    FUN_00601b30(pvVar11,0x24,0xe);
    **(undefined4 **)((int)param_1 + 0x120) = 0x15e;
    *(undefined4 *)(*(int *)((int)param_1 + 0x120) + 0x2c) = 1;
    FUN_00403e90(*(void **)((int)param_1 + 0x120),1);
    FUN_00601c60(*(void **)((int)param_1 + 0x120),5);
    FUN_00601c90(*(void **)((int)param_1 + 0x120),0x2ffffff,(HWND)0x0);
  }
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 0xc;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1850);
    pvVar11 = FUN_00601700(pvStack_10,&local_14,0,0,0,0,0,0);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x130) = pvVar11;
  if (pvVar11 != (void *)0x0) {
    FUN_00601b30(pvVar11,0x28,0x77);
    **(undefined4 **)((int)param_1 + 0x130) = 0x11b;
    *(undefined4 *)(*(int *)((int)param_1 + 0x130) + 0x2c) = 1;
    FUN_00403e90(*(void **)((int)param_1 + 0x130),0);
    FUN_00601c60(*(void **)((int)param_1 + 0x130),5);
    FUN_00601c90(*(void **)((int)param_1 + 0x130),0x2ffffff,(HWND)0x0);
  }
  if (*(int *)(*(int *)((int)param_1 + 0x160) + 0x9c) == 1) {
    pvVar11 = (void *)FUN_00618b70(0x9c);
    local_4 = 0xd;
    pvStack_10 = pvVar11;
    if (pvVar11 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      puVar3 = (uint *)FUN_006073d0(param_1);
      pvVar11 = FUN_0060d590(pvVar11,uVar1,0x1a7,0x5d,0x20,0x74,0x66,param_1,puVar3);
    }
    *(void **)((int)param_1 + 0x11c) = pvVar11;
  }
  else {
    pvVar11 = (void *)FUN_00618b70(0x9c);
    local_4 = 0xe;
    pvStack_10 = pvVar11;
    if (pvVar11 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      puVar3 = (uint *)FUN_006073d0(param_1);
      pvVar11 = FUN_0060d590(pvVar11,uVar1,0x1aa,0x59,0x2c,0x88,0x66,param_1,puVar3);
    }
    *(void **)((int)param_1 + 0x11c) = pvVar11;
  }
  local_4 = 0xffffffff;
  if (*(void **)((int)param_1 + 0x11c) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)param_1 + 0x11c),0);
    if (*(int *)(*(int *)((int)param_1 + 0x160) + 0x9c) == 1) {
      pvStack_10 = (void *)FUN_00618b70(0x114);
      local_4 = 0xf;
      if (pvStack_10 == (void *)0x0) {
        piVar12 = (int *)0x0;
      }
      else {
        piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0,0x36,0x20,0x1f,
                                      *(int *)((int)param_1 + 0x11c),0x68,0x2884,0x2885,
                                      *(uint **)(*(int *)((int)param_1 + 0x11c) + 0x98),0,0);
      }
      uVar13 = 0x2885;
    }
    else {
      pvStack_10 = (void *)FUN_00618b70(0x114);
      local_4 = 0x10;
      if (pvStack_10 == (void *)0x0) {
        piVar12 = (int *)0x0;
      }
      else {
        piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0,0x36,0x2c,0x29,
                                      *(int *)((int)param_1 + 0x11c),0x68,0x288a,0x288b,
                                      *(uint **)(*(int *)((int)param_1 + 0x11c) + 0x98),0,0);
      }
      uVar13 = 0x288b;
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    FUN_00602840(piVar12,0x260);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1859);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_00603150(piVar12,2,uVar13);
    FUN_0060d700(*(void **)((int)param_1 + 0x11c),piVar12,0,0);
    if (*(int *)(*(int *)((int)param_1 + 0x160) + 0x9c) == 1) {
      pvStack_10 = (void *)FUN_00618b70(0x114);
      local_4 = 0x11;
      if (pvStack_10 == (void *)0x0) {
        piVar12 = (int *)0x0;
      }
      else {
        piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0,0,0x20,0x1f,*(int *)((int)param_1 + 0x11c),
                                      0x67,0x2886,0x2887,
                                      *(uint **)(*(int *)((int)param_1 + 0x11c) + 0x98),0,0);
      }
      uVar13 = 0x2887;
    }
    else {
      pvStack_10 = (void *)FUN_00618b70(0x114);
      local_4 = 0x12;
      if (pvStack_10 == (void *)0x0) {
        piVar12 = (int *)0x0;
      }
      else {
        piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0,0,0x2c,0x29,*(int *)((int)param_1 + 0x11c),
                                      0x67,0x288c,0x288d,
                                      *(uint **)(*(int *)((int)param_1 + 0x11c) + 0x98),0,0);
      }
      uVar13 = 0x288d;
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    FUN_00602840(piVar12,0x260);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1860);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_00603150(piVar12,2,uVar13);
    FUN_0060d700(*(void **)((int)param_1 + 0x11c),piVar12,0,0);
  }
  pvStack_10 = (void *)FUN_00618b70(0x9c);
  local_4 = 0x13;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    pvVar11 = FUN_0060d590(pvStack_10,uVar1,0x24,0x4e,0x169,0x29,0x6e,param_1,puVar9);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x124) = pvVar11;
  if (pvVar11 != (void *)0x0) {
    *(uint *)((int)pvVar11 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 0x124),0);
    iVar6 = *(int *)(*(int *)((int)param_1 + 0x160) + 0x9c);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x14;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0,0,0x31,0x29,*(int *)((int)param_1 + 0x124),
                                    0x6f,0x2864,0x2863,
                                    *(uint **)(*(int *)((int)param_1 + 0x124) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1850);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x124),piVar12,1,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x15;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0xd0,0,0x31,0x29,*(int *)((int)param_1 + 0x124)
                                    ,0x73,(-(uint)(iVar6 != 1) & 2) + 0x2d60,
                                    (-(uint)(iVar6 != 1) & 2) + 0x2d5f,
                                    *(uint **)(*(int *)((int)param_1 + 0x124) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1851);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x124),piVar12,0,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x16;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0x9c,0,0x31,0x29,*(int *)((int)param_1 + 0x124)
                                    ,0x72,(-(uint)(iVar6 != 1) & 0xc) + 0x2868,
                                    (-(uint)(iVar6 != 1) & 0xc) + 0x2867,
                                    *(uint **)(*(int *)((int)param_1 + 0x124) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1852);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x124),piVar12,0,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x17;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0x138,0,0x31,0x29,
                                    *(int *)((int)param_1 + 0x124),0x75,
                                    (-(uint)(iVar6 != 1) & 0xc) + 0x286a,
                                    (-(uint)(iVar6 != 1) & 0xc) + 0x2869,
                                    *(uint **)(*(int *)((int)param_1 + 0x124) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1853);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x124),piVar12,0,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x18;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0x68,0,0x31,0x29,*(int *)((int)param_1 + 0x124)
                                    ,0x71,(-(uint)(iVar6 != 1) & 0xc) + 0x286c,
                                    (-(uint)(iVar6 != 1) & 0xc) + 0x286b,
                                    *(uint **)(*(int *)((int)param_1 + 0x124) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1854);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x124),piVar12,0,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x19;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0x34,0,0x31,0x29,*(int *)((int)param_1 + 0x124)
                                    ,0x70,0x286e,0x286d,
                                    *(uint **)(*(int *)((int)param_1 + 0x124) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1855);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x124),piVar12,0,0);
    pvStack_10 = (void *)FUN_00618b70(0x114);
    local_4 = 0x1a;
    if (pvStack_10 == (void *)0x0) {
      piVar12 = (int *)0x0;
    }
    else {
      piVar12 = (int *)FUN_00602150(pvStack_10,uVar2,0x104,0,0x31,0x29,
                                    *(int *)((int)param_1 + 0x124),0x74,
                                    (-(uint)(iVar6 != 1) & 10) + 0x2870,
                                    (-(uint)(iVar6 != 1) & 10) + 0x286f,
                                    *(uint **)(*(int *)((int)param_1 + 0x124) + 0x98),0,0);
    }
    local_4 = 0xffffffff;
    FUN_005ffce0(piVar12,0);
    local_14 = (uint *)CONCAT22(DAT_0065d424,0x1856);
    FUN_00600a40(param_1,piVar12,&local_14);
    FUN_0060d700(*(void **)((int)param_1 + 0x124),piVar12,0,0);
  }
  pvStack_10 = (void *)FUN_00618b70(0xf4);
  local_4 = 0x1b;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    pvVar11 = FUN_00604cf0(pvStack_10,uVar1,0x8f,0x2d,0xf5,0x12,(int)param_1,puVar9,0);
  }
  *(void **)((int)param_1 + 0x128) = pvVar11;
  *(undefined4 *)((int)pvVar11 + 0x24) = 100;
  local_4 = 0xffffffff;
  *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0xe4) = 0xffffff;
  *(uint *)(*(int *)((int)param_1 + 0x128) + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
  FUN_005ffce0(*(void **)((int)param_1 + 0x128),0);
  FUN_00605070(*(void **)((int)param_1 + 0x128));
  pvStack_10 = (void *)FUN_00618b70(0x118);
  local_4 = 0x1c;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    pvVar11 = FUN_00607ea0(pvStack_10,uVar2,0x24,0x89,0x15e,0xa0,param_1,puVar9,0x65,0x15e,0x14);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 300) = pvVar11;
  if (pvVar11 != (void *)0x0) {
    *(undefined4 *)((int)pvVar11 + 0xf4) = 0x30000;
    *(uint *)(*(int *)((int)param_1 + 300) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 300) + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)param_1 + 300),0);
    FUN_00609340(*(void **)((int)param_1 + 300),2);
    *(undefined4 *)(*(int *)((int)param_1 + 300) + 0xe4) = 0x24;
    iVar6 = *(int *)((int)param_1 + 300);
    FUN_0060a490((void *)(iVar6 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar6,7,0x299d,
                 *(undefined4 *)(iVar6 + 0x3c),0,*(undefined4 *)(iVar6 + 0x3c),0,(uint *)0x0);
    *(undefined4 *)(*(int *)((int)param_1 + 300) + 0xe8) = 5;
    pvStack_10 = (void *)FUN_00618b70(0x10);
    local_4 = 0x1d;
    if (pvStack_10 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_0060a790(pvStack_10,2);
    }
    local_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x134) = pvVar11;
    FUN_0045f100(param_1,0x6f,1);
  }
  pvStack_10 = (void *)FUN_00618b70(0x178);
  local_4 = 0x1e;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    pvVar11 = FUN_0041ecf0(pvStack_10,*(undefined4 *)((int)param_1 + 0x1c),0x11,0xe7,0x18b,0x50,
                           param_1,puVar8,0);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x13c) = pvVar11;
  if (pvVar11 != (void *)0x0) {
    *(uint *)((int)pvVar11 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
    iVar6 = *(int *)((int)param_1 + 0x13c);
    *(undefined2 *)(iVar6 + 0x134) = 7;
    *(undefined2 *)(iVar6 + 0x136) = 0x299d;
    FUN_005ffce0(*(void **)((int)param_1 + 0x13c),0);
  }
  pvStack_10 = (void *)FUN_00618b70(0x114);
  local_4 = 0x1f;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    pvVar11 = FUN_00602150(pvStack_10,uVar2,0x17c,0xe,0x15,0x11,param_1,0x84,0x288e,0x288f,puVar8,0,
                           0);
  }
  *(void **)((int)param_1 + 0x140) = pvVar11;
  *(uint *)((int)pvVar11 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
  local_4 = 0xffffffff;
  FUN_005ffce0(*(void **)((int)param_1 + 0x140),0);
  FUN_00603150(*(void **)((int)param_1 + 0x140),2,0x2890);
  local_14 = (uint *)CONCAT22(DAT_0065d424,0x1857);
  FUN_00600a40(param_1,*(void **)((int)param_1 + 0x140),&local_14);
  pvStack_10 = (void *)FUN_00618b70(0x114);
  local_4 = 0x20;
  if (pvStack_10 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    pvVar11 = FUN_00602150(pvStack_10,uVar2,0x1c,0xe,0x15,0x11,param_1,0x83,0x2891,0x2892,puVar8,0,0
                          );
  }
  *(void **)((int)param_1 + 0x144) = pvVar11;
  *(uint *)((int)pvVar11 + 0x50) = *(uint *)((int)pvVar5 + 0x50) & 0xefffffff;
  local_4 = 0xffffffff;
  FUN_005ffce0(*(void **)((int)param_1 + 0x144),0);
  FUN_00603150(*(void **)((int)param_1 + 0x144),2,0x2893);
  local_14 = (uint *)CONCAT22(DAT_0065d424,0x1858);
  FUN_00600a40(param_1,*(void **)((int)param_1 + 0x144),&local_14);
  if ((*(int *)((int)param_1 + 0x114) == 2) && (*(int *)((int)param_1 + 0x148) != 0)) {
    uVar13 = 0x67;
  }
  else {
    uVar13 = 0x68;
  }
  FUN_0060d7e0(*(void **)((int)param_1 + 0x11c),uVar13,1);
  FUN_0041d3f0();
  ExceptionList = pvStack_c;
  return;
}

