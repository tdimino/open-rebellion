
void __fastcall FUN_004a8790(int *param_1)

{
  undefined4 uVar1;
  void *pvVar2;
  int iVar3;
  undefined *puVar4;
  char *pcVar5;
  uint *puVar6;
  int *piVar7;
  int iVar8;
  undefined4 uVar9;
  uint uVar10;
  uint uVar11;
  undefined4 uVar12;
  UINT UVar13;
  uint uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  int iVar18;
  int iVar19;
  uint *local_30;
  undefined4 local_2c;
  void *local_28;
  undefined1 auStack_24 [4];
  undefined4 uStack_20;
  undefined4 uStack_1c;
  uint uStack_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006380f2;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_2c = FUN_004a25c0(param_1);
  piVar7 = (int *)0x0;
  if (local_2c != (int *)0x0) {
    param_1[0x52] = (uint)local_2c[9] >> 6 & 3;
    uVar1 = FUN_006037f0(7);
    FUN_006037f0(2);
    FUN_006071a0(param_1);
    FUN_00607ce0(param_1,2);
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (pvVar2 == (void *)0x0) {
      local_30 = (uint *)0x0;
    }
    else {
      local_30 = FUN_005fbd20(pvVar2,uVar1,0x2951,10);
    }
    local_4 = 0xffffffff;
    FUN_006075b0(param_1,(int *)local_30,0x32);
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (local_28 != (void *)0x0) {
      piVar7 = FUN_005fbd20(local_28,uVar1,0x277c,2);
    }
    local_4 = 0xffffffff;
    iVar8 = 0;
    if (piVar7 != (int *)0x0) {
      iVar8 = param_1[0xc];
      iVar3 = FUN_005fc0e0(piVar7);
      iVar8 = (iVar8 - iVar3) + -3;
      FUN_00606c90(param_1,iVar8,3,0x277c,0x277d,200,0,0,0,0,0,0,1);
      FUN_005fbfa0(piVar7);
      FUN_00618b60((undefined *)piVar7);
    }
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (local_28 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = FUN_005fbd20(local_28,uVar1,0x280d,2);
    }
    local_4 = 0xffffffff;
    if (piVar7 != (int *)0x0) {
      iVar19 = 1;
      iVar18 = 0;
      uVar17 = 0;
      uVar16 = 0;
      uVar15 = 0;
      uVar14 = 0;
      UVar13 = 0;
      uVar12 = 0xc9;
      uVar11 = 0x280e;
      uVar10 = 0x280d;
      uVar9 = 3;
      iVar3 = FUN_005fc0e0(piVar7);
      FUN_00606c90(param_1,iVar8 - iVar3,uVar9,uVar10,uVar11,uVar12,UVar13,uVar14,uVar15,uVar16,
                   uVar17,iVar18,iVar19);
      FUN_005fbfa0(piVar7);
      FUN_00618b60((undefined *)piVar7);
    }
    local_28 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (local_28 == (void *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = FUN_005fbd20(local_28,uVar1,0x27e1,2);
    }
    local_4 = 0xffffffff;
    FUN_00606c90(param_1,3,3,0x27e1,0x27e0,0xca,0,0,0,0,0,0,1);
    iVar8 = 2;
    if (piVar7 != (int *)0x0) {
      iVar8 = FUN_005fc0e0(piVar7);
      iVar8 = iVar8 + 5;
      FUN_005fbfa0(piVar7);
      FUN_00618b60((undefined *)piVar7);
    }
    FUN_00601b30((void *)param_1[0x49],iVar8,2);
    *(undefined4 *)(param_1[0x49] + 4) = 0x10;
    puVar4 = FUN_004f62d0((int)local_2c);
    pcVar5 = (char *)FUN_00583c40((int)puVar4);
    FUN_00601aa0((void *)param_1[0x49],pcVar5);
    FUN_00601c60((void *)param_1[0x49],5);
    FUN_00403e90((void *)param_1[0x49],0x24);
    SetRect((LPRECT)(param_1 + 0x4a),2,2,0xe9,0x12);
    local_28 = (void *)FUN_00618b70(0x34);
    local_4 = 4;
    if (local_28 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_00601880(local_28,(char *)0x0,0,0,0,0,0,0);
    }
    local_4 = 0xffffffff;
    param_1[0x5b] = (int)pvVar2;
    FUN_00601b30(pvVar2,2,0x33);
    *(undefined4 *)param_1[0x5b] = 0xe7;
    *(undefined4 *)(param_1[0x5b] + 4) = 0x10;
    FUN_00601c60((void *)param_1[0x5b],4);
    *(undefined4 *)(param_1[0x5b] + 0x2c) = 3;
    FUN_00403e90((void *)param_1[0x5b],0x25);
    FUN_00601c90((void *)param_1[0x5b],0xffffff,(HWND)0x0);
    local_28 = (void *)FUN_00618b70(0x34);
    local_4 = 5;
    if (local_28 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_00601880(local_28,(char *)0x0,0,0,0,0,0,0);
    }
    local_4 = 0xffffffff;
    param_1[0x5c] = (int)pvVar2;
    FUN_00601b30(pvVar2,2,0x3f);
    *(undefined4 *)param_1[0x5c] = 0xe4;
    *(undefined4 *)(param_1[0x5c] + 4) = 0x11;
    FUN_00601c60((void *)param_1[0x5c],4);
    *(undefined4 *)(param_1[0x5c] + 0x2c) = 3;
    FUN_00403e90((void *)param_1[0x5c],0x25);
    FUN_00601c90((void *)param_1[0x5c],0xffffff,(HWND)0x0);
    pvVar2 = (void *)FUN_00618b70(0x9c);
    local_4 = 6;
    local_28 = pvVar2;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      iVar8 = param_1[7];
      puVar6 = (uint *)FUN_006073d0(param_1);
      pvVar2 = FUN_0060d590(pvVar2,iVar8,0,0x14,0x130,0x21,100,param_1,puVar6);
    }
    local_4 = 0xffffffff;
    param_1[0x58] = (int)pvVar2;
    if (pvVar2 != (void *)0x0) {
      FUN_005ffce0(pvVar2,0);
      local_28 = (void *)FUN_00618b70(0x114);
      local_4 = 7;
      if (local_28 == (void *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = FUN_00602150(local_28,uVar1,0xac,0,0x24,0x21,param_1[0x58],5,0x2936,0x2937,
                              *(uint **)(param_1[0x58] + 0x98),0,1);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(piVar7,0);
      local_2c = (int *)CONCAT22(DAT_0065d424,0x1740);
      FUN_00600a40(param_1,piVar7,&local_2c);
      FUN_0060d700((void *)param_1[0x58],piVar7,1,0);
      local_28 = (void *)FUN_00618b70(0x114);
      local_4 = 8;
      if (local_28 == (void *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = FUN_00602150(local_28,uVar1,0x88,0,0x24,0x21,param_1[0x58],4,0x2939,0x293a,
                              *(uint **)(param_1[0x58] + 0x98),0,1);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(piVar7,0);
      FUN_00603150(piVar7,0x40,0x293b);
      FUN_00603150(piVar7,2,0x2939);
      local_2c = (int *)CONCAT22(DAT_0065d424,0x1741);
      FUN_00600a40(param_1,piVar7,&local_2c);
      FUN_0060d700((void *)param_1[0x58],piVar7,0,1);
      local_28 = (void *)FUN_00618b70(0x114);
      local_4 = 9;
      if (local_28 == (void *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = FUN_00602150(local_28,uVar1,100,0,0x24,0x21,param_1[0x58],3,0x2942,0x2942,
                              *(uint **)(param_1[0x58] + 0x98),0,1);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(piVar7,0);
      local_2c = (int *)CONCAT22(DAT_0065d424,0x1742);
      FUN_00600a40(param_1,piVar7,&local_2c);
      FUN_0060d700((void *)param_1[0x58],piVar7,0,1);
      local_28 = (void *)FUN_00618b70(0x114);
      local_4 = 10;
      if (local_28 == (void *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = FUN_00602150(local_28,uVar1,0x40,0,0x24,0x21,param_1[0x58],2,0x2949,0x2949,
                              *(uint **)(param_1[0x58] + 0x98),0,1);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(piVar7,0);
      local_2c = (int *)CONCAT22(DAT_0065d424,0x1743);
      FUN_00600a40(param_1,piVar7,&local_2c);
      FUN_0060d700((void *)param_1[0x58],piVar7,0,1);
      local_28 = (void *)FUN_00618b70(0x114);
      local_4 = 0xb;
      if (local_28 == (void *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = FUN_00602150(local_28,uVar1,0x1c,0,0x24,0x21,param_1[0x58],1,0x2950,0x2950,
                              *(uint **)(param_1[0x58] + 0x98),0,1);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(piVar7,0);
      local_2c = (int *)CONCAT22(DAT_0065d424,0x1744);
      FUN_00600a40(param_1,piVar7,&local_2c);
      FUN_0060d700((void *)param_1[0x58],piVar7,0,1);
    }
    local_28 = (void *)FUN_00618b70(0x118);
    local_4 = 0xc;
    if (local_28 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_00607ea0(local_28,uVar1,7,0x51,0xde,0xd2,param_1,local_30,0xcb,0x46,0x46);
    }
    local_4 = 0xffffffff;
    param_1[0x59] = (int)pvVar2;
    if (pvVar2 != (void *)0x0) {
      local_28 = (void *)FUN_00618b70(0x10);
      local_4 = 0xd;
      if (local_28 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_0060a790(local_28,4);
      }
      param_1[0x5a] = (int)pvVar2;
      local_4 = 0xffffffff;
      *(undefined4 *)(param_1[0x59] + 0xf4) = 0xa0000;
      (**(code **)(*(int *)param_1[0x59] + 0x18))(10);
      *(undefined4 *)(param_1[0x59] + 0xe4) = 0x10;
      *(undefined4 *)(param_1[0x59] + 0xec) = 0x1b;
      (**(code **)(*(int *)param_1[0x59] + 0x18))(10);
      FUN_00609950((void *)param_1[0x59],0x2ffffff);
      FUN_00609290((void *)param_1[0x59],*(HWND *)(param_1[0x53] + 0x18));
      FUN_005ffce0((void *)param_1[0x59],0);
      FUN_00609340((void *)param_1[0x59],4);
      FUN_00608280((void *)param_1[0x59],param_1[0x5a]);
      iVar8 = param_1[0x59];
      FUN_0060a490((void *)(iVar8 + 0x94),param_1[7],iVar8,7,0x29cc,*(undefined4 *)(iVar8 + 0x3c),0,
                   *(undefined4 *)(iVar8 + 0x3c),0,(uint *)0x0);
      FUN_005f4f10(param_1 + 0x1b,param_1[0x59]);
    }
    FUN_004a9ce0(param_1);
    FUN_0060d7e0((void *)param_1[0x58],1,0);
    FUN_004a90d0(param_1,1);
    PostMessageA(*(HWND *)(param_1[0x53] + 0x18),0x467,param_1[6],0);
    if ((*(byte *)(param_1[0x53] + 0xc0) & 4) == 0) {
      FUN_00610b20(auStack_24,0,0,0,0,0,0);
      uStack_14 = uStack_14 & 0xfffffff9 | 1;
      local_4 = 0xe;
      uStack_1c = 7;
      uStack_20 = 0x25e;
      FUN_00610c30((int)auStack_24);
    }
  }
  ExceptionList = local_c;
  return;
}

