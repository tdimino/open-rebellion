
void __fastcall FUN_0049f540(int *param_1)

{
  undefined4 uVar1;
  int *piVar2;
  void *pvVar3;
  uint *puVar4;
  int iVar5;
  int iVar6;
  undefined4 *puVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  int *piVar10;
  undefined4 uVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  undefined4 local_30;
  int *local_2c;
  void *local_28;
  undefined1 auStack_24 [4];
  undefined4 uStack_20;
  undefined4 uStack_1c;
  uint uStack_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006374bb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_006037f0(7);
  local_30 = uVar1;
  local_2c = FUN_004a25c0(param_1);
  if (local_2c != (int *)0x0) {
    FUN_006071a0(param_1);
    FUN_00607ce0(param_1,2);
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (local_28 == (void *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = FUN_005fbd20(local_28,uVar1,0x2b9d,2);
    }
    local_4 = 0xffffffff;
    FUN_006075b0(param_1,piVar2,0xf);
    param_1[0x5a] = 0;
    param_1[0x5b] = 0;
    FUN_004a2200(param_1);
    param_1[0x59] = 2;
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (local_28 == (void *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = FUN_005fbd20(local_28,uVar1,0x27e1,2);
    }
    local_4 = 0xffffffff;
    if (piVar2 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x114);
      local_4 = 2;
      local_28 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        iVar16 = 1;
        iVar14 = 0;
        puVar4 = (uint *)FUN_006073d0(param_1);
        uVar13 = 0x27e0;
        uVar12 = 0x27e1;
        uVar11 = 0xca;
        piVar10 = param_1;
        iVar5 = FUN_005fc0f0(piVar2);
        iVar6 = FUN_005fc0e0(piVar2);
        pvVar3 = FUN_00602150(pvVar3,uVar1,3,3,iVar6,iVar5,piVar10,uVar11,uVar12,uVar13,puVar4,
                              iVar14,iVar16);
      }
      local_4 = 0xffffffff;
      param_1[0x56] = (int)pvVar3;
      if (pvVar3 != (void *)0x0) {
        FUN_005ffce0(pvVar3,0);
        iVar5 = FUN_005fc0e0(piVar2);
        param_1[0x59] = iVar5 + 5;
      }
      FUN_005fbfa0(piVar2);
      FUN_00618b60((undefined *)piVar2);
    }
    piVar2 = (int *)0x0;
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (local_28 != (void *)0x0) {
      piVar2 = FUN_005fbd20(local_28,uVar1,0x277c,2);
    }
    local_4 = 0xffffffff;
    if (piVar2 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x114);
      local_4 = 4;
      local_28 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        iVar5 = param_1[0xc];
        iVar17 = 1;
        iVar15 = 0;
        puVar4 = (uint *)FUN_006073d0(param_1);
        uVar13 = 0x277d;
        uVar12 = 0x277c;
        uVar11 = 0x14;
        piVar10 = param_1;
        iVar6 = FUN_005fc0f0(piVar2);
        iVar14 = FUN_005fc0e0(piVar2);
        uVar1 = 3;
        iVar16 = FUN_005fc0e0(piVar2);
        pvVar3 = FUN_00602150(pvVar3,local_30,(iVar5 - iVar16) + -3,uVar1,iVar14,iVar6,piVar10,
                              uVar11,uVar12,uVar13,puVar4,iVar15,iVar17);
      }
      local_4 = 0xffffffff;
      param_1[0x54] = (int)pvVar3;
      FUN_005ffce0(pvVar3,0);
      FUN_005fbfa0(piVar2);
      FUN_00618b60((undefined *)piVar2);
      uVar1 = local_30;
    }
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 5;
    if (local_28 == (void *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = FUN_005fbd20(local_28,uVar1,0x280d,2);
    }
    local_4 = 0xffffffff;
    if (piVar2 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x114);
      local_4 = 6;
      local_28 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        iVar17 = 1;
        iVar15 = 0;
        iVar5 = *(int *)(param_1[0x54] + 0x28);
        puVar4 = (uint *)FUN_006073d0(param_1);
        uVar13 = 0x280e;
        uVar12 = 0x280d;
        uVar11 = 0x15;
        piVar10 = param_1;
        iVar6 = FUN_005fc0f0(piVar2);
        iVar14 = FUN_005fc0e0(piVar2);
        uVar1 = 3;
        iVar16 = FUN_005fc0e0(piVar2);
        pvVar3 = FUN_00602150(pvVar3,local_30,iVar5 - iVar16,uVar1,iVar14,iVar6,piVar10,uVar11,
                              uVar12,uVar13,puVar4,iVar15,iVar17);
      }
      local_4 = 0xffffffff;
      param_1[0x55] = (int)pvVar3;
      FUN_005ffce0(pvVar3,0);
      FUN_005fbfa0(piVar2);
      FUN_00618b60((undefined *)piVar2);
      uVar1 = local_30;
    }
    local_28 = (void *)FUN_00618b70(0x10);
    local_4 = 7;
    if (local_28 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_0060a790(local_28,4);
    }
    local_4 = 0xffffffff;
    param_1[0x6d] = (int)pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x118);
    local_4 = 8;
    local_28 = pvVar3;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      uVar9 = 0x32;
      uVar8 = 0x5a;
      uVar11 = 10;
      puVar4 = (uint *)FUN_006073d0(param_1);
      pvVar3 = FUN_00607ea0(pvVar3,uVar1,5,0x18,0x5e,0x113,param_1,puVar4,uVar11,uVar8,uVar9);
    }
    local_4 = 0xffffffff;
    param_1[0x6e] = (int)pvVar3;
    if (pvVar3 != (void *)0x0) {
      *(uint *)((int)pvVar3 + 0x50) = *(uint *)((int)pvVar3 + 0x50) & 0xefffffff;
      FUN_005ffce0((void *)param_1[0x6e],0);
      *(uint *)(param_1[0x6e] + 0xf4) = *(uint *)(param_1[0x6e] + 0xf4) | 0x20000;
      iVar5 = param_1[0x6e];
      FUN_0060a490((void *)(iVar5 + 0x94),param_1[7],iVar5,7,0x29fc,*(undefined4 *)(iVar5 + 0x3c),0,
                   *(undefined4 *)(iVar5 + 0x3c),0,(uint *)0x0);
      FUN_00609950((void *)param_1[0x6e],0xffffff);
      *(undefined4 *)(param_1[0x6e] + 0xe4) = 0x10;
      (**(code **)(*(int *)param_1[0x6e] + 0x18))(10);
      FUN_00608280((void *)param_1[0x6e],param_1[0x6d]);
    }
    local_28 = (void *)FUN_00618b70(0x10);
    local_4 = 9;
    if (local_28 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_0060a790(local_28,4);
    }
    local_4 = 0xffffffff;
    param_1[0x6f] = (int)pvVar3;
    pvVar3 = (void *)FUN_00618b70(0x118);
    local_4 = 10;
    local_28 = pvVar3;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      uVar9 = 0x2b;
      uVar8 = 0x73;
      uVar11 = 0xb;
      puVar4 = (uint *)FUN_006073d0(param_1);
      pvVar3 = FUN_00607ea0(pvVar3,uVar1,0x6b,0x91,0x75,0x93,param_1,puVar4,uVar11,uVar8,uVar9);
    }
    local_4 = 0xffffffff;
    param_1[0x70] = (int)pvVar3;
    if (pvVar3 != (void *)0x0) {
      *(uint *)((int)pvVar3 + 0x50) = *(uint *)((int)pvVar3 + 0x50) & 0xefffffff;
      FUN_005ffce0((void *)param_1[0x70],0);
      *(uint *)(param_1[0x70] + 0xf4) = *(uint *)(param_1[0x70] + 0xf4) | 0xa0000;
      iVar5 = param_1[0x70];
      FUN_0060a490((void *)(iVar5 + 0x94),param_1[7],iVar5,7,0x29fc,*(undefined4 *)(iVar5 + 0x3c),0,
                   *(undefined4 *)(iVar5 + 0x3c),0,(uint *)0x0);
      FUN_00609950((void *)param_1[0x70],0xffffff);
      *(undefined4 *)(param_1[0x70] + 0xe4) = 1;
      FUN_00608280((void *)param_1[0x70],param_1[0x6f]);
    }
    pvVar3 = (void *)FUN_00618b70(0x9c);
    local_4 = 0xb;
    local_28 = pvVar3;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      iVar5 = param_1[7];
      puVar4 = (uint *)FUN_006073d0(param_1);
      pvVar3 = FUN_0060d590(pvVar3,iVar5,0x69,0x7f,0x7a,0x10,0x16,param_1,puVar4);
    }
    param_1[0x57] = (int)pvVar3;
    local_4 = 0xffffffff;
    if (pvVar3 != (void *)0x0) {
      FUN_005ffce0(pvVar3,0);
      FUN_005f4f10(param_1 + 0x1b,param_1[0x57]);
      pvVar3 = (void *)FUN_00618b70(0x114);
      local_4 = 0xc;
      local_28 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        piVar2 = (int *)0x0;
      }
      else {
        iVar6 = 0;
        iVar5 = 0;
        puVar4 = (uint *)FUN_006073d0(param_1);
        piVar2 = FUN_00602150(pvVar3,uVar1,0,0,0x3d,0x2b,param_1[0x57],0x17,0x2d28,0x2d29,puVar4,
                              iVar5,iVar6);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(piVar2,0);
      local_30 = CONCAT22(DAT_0065d424,0x8500);
      FUN_00600a40(param_1,piVar2,&local_30);
      FUN_0060d700((void *)param_1[0x57],piVar2,1,0);
      pvVar3 = (void *)FUN_00618b70(0x114);
      local_4 = 0xd;
      local_28 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        piVar2 = (int *)0x0;
      }
      else {
        iVar6 = 0;
        iVar5 = 0;
        puVar4 = (uint *)FUN_006073d0(param_1);
        piVar2 = FUN_00602150(pvVar3,uVar1,0x3d,0,0x3d,0x2b,param_1[0x57],0x18,0x2d2a,0x2d2b,puVar4,
                              iVar5,iVar6);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(piVar2,0);
      local_30 = CONCAT22(DAT_0065d424,0x8501);
      FUN_00600a40(param_1,piVar2,&local_30);
      FUN_0060d700((void *)param_1[0x57],piVar2,0,1);
    }
    local_28 = (void *)FUN_00618b70(0x34);
    local_4 = 0xe;
    if (local_28 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_00601880(local_28,(char *)0x0,0,0,0,0,0,0);
    }
    local_4 = 0xffffffff;
    param_1[0x5d] = (int)pvVar3;
    FUN_00601b30(pvVar3,0x6d,0x5b);
    FUN_00601c90((void *)param_1[0x5d],0xffffff,(HWND)0x0);
    FUN_00601c60((void *)param_1[0x5d],5);
    *(undefined4 *)param_1[0x5d] = 0x71;
    *(undefined4 *)(param_1[0x5d] + 4) = 0x20;
    FUN_00403e90((void *)param_1[0x5d],0x11);
    *(undefined4 *)(param_1[0x5d] + 0x2c) = 3;
    local_28 = (void *)FUN_00618b70(0x34);
    local_4 = 0xf;
    if (local_28 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_00601880(local_28,(char *)0x0,0,0,0,0,0,0);
    }
    local_4 = 0xffffffff;
    param_1[0x5e] = (int)pvVar3;
    FUN_00601b30(pvVar3,0x6d,0x19);
    FUN_00601c90((void *)param_1[0x5e],0xffffff,(HWND)0x0);
    FUN_00601c60((void *)param_1[0x5e],5);
    *(undefined4 *)param_1[0x5e] = 0x71;
    *(undefined4 *)(param_1[0x5e] + 4) = 0x10;
    FUN_00403e90((void *)param_1[0x5e],1);
    *(undefined4 *)(param_1[0x5e] + 0x2c) = 3;
    piVar2 = param_1 + 0x5f;
    FUN_00601c60(piVar2,5);
    FUN_00601c90(piVar2,0,(HWND)0x0);
    *piVar2 = 0x7a;
    param_1[0x6a] = 1;
    puVar4 = FUN_004025b0(local_2c,(uint *)&local_2c);
    local_4 = 0x10;
    FUN_004f26d0(param_1 + 0x51,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 0x11;
    if (local_28 == (void *)0x0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = FUN_004acd80(local_28,param_1 + 0x51,0);
    }
    local_4 = 0xffffffff;
    FUN_004acba0(param_1 + 0x45,puVar7,0);
    iVar5 = FUN_004a1590(param_1);
    if (iVar5 != 0) {
      (**(code **)(*(int *)param_1[0x6e] + 4))(5);
      iVar5 = (**(code **)(*(int *)param_1[0x6d] + 8))();
      FUN_00609500((void *)param_1[0x6e],*(int *)(iVar5 + 0xc),1);
    }
    SetFocus(*(HWND *)(param_1[0x6e] + 0x18));
    if ((*(byte *)(param_1[0x53] + 0xc0) & 4) == 0) {
      FUN_00610b20(auStack_24,0,0,0,0,0,0);
      uStack_14 = uStack_14 & 0xfffffff9 | 1;
      local_4 = 0x12;
      uStack_20 = 0x25e;
      uStack_1c = 7;
      FUN_00610c30((int)auStack_24);
    }
  }
  ExceptionList = local_c;
  return;
}

