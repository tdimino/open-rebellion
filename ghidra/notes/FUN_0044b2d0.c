
void __fastcall FUN_0044b2d0(int param_1)

{
  undefined1 uVar1;
  int iVar2;
  undefined4 uVar3;
  uint uVar4;
  void *pvVar5;
  char *pcVar6;
  uint *puVar7;
  int iVar8;
  int iVar9;
  uint *puVar10;
  int *piVar11;
  uint local_44;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [3];
  undefined4 local_20 [2];
  int local_18;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f50b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_004f3dd0(1,1);
  FUN_005f2f50(local_2c);
  piVar11 = (int *)0x0;
  local_4 = 0;
  FUN_0051c740(local_20);
  local_4._0_1_ = 1;
  FUN_004fce50(local_20);
  uVar3 = FUN_006037f0(7);
  local_30 = (void *)(uint)(local_18 == 0);
  if (iVar2 != 0) {
    local_44 = *(uint *)(iVar2 + 0xbc);
  }
  local_34 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 2;
  if (local_34 != (void *)0x0) {
    piVar11 = FUN_005fbd20(local_34,uVar3,0x27fa,10);
  }
  local_4._0_1_ = 1;
  uVar4 = FUN_005fc0e0(piVar11);
  iVar2 = FUN_005fc0f0(piVar11);
  local_34 = (uint *)FUN_00618b70(0x68);
  local_4._0_1_ = 3;
  if (local_34 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_0060ba00(local_34,1);
  }
  *(undefined4 *)((int)pvVar5 + 0x3c) = 4;
  uVar1 = (undefined1)piVar11[2];
  local_4._0_1_ = 1;
  FUN_0060be60(pvVar5,(int)piVar11,0);
  if ((local_44 & 0x300) == 0) {
    local_34._0_2_ = 0x8781;
  }
  else {
    local_34._0_2_ = 0x8782;
  }
  local_34 = (uint *)CONCAT22(DAT_0065d424,(undefined2)local_34);
  FUN_005f3010(local_2c,&local_34);
  pcVar6 = (char *)FUN_00583c40((int)local_2c);
  FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar6);
  FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar5);
  if (local_30 != (void *)0x0) {
    local_34 = (uint *)FUN_00618b70(0x28);
    local_4._0_1_ = 4;
    if (local_34 == (void *)0x0) {
      puVar7 = (uint *)0x0;
    }
    else {
      puVar7 = FUN_005fbd20(local_34,uVar3,0x2b05,2);
    }
    local_4._0_1_ = 1;
    local_34 = (uint *)FUN_00618b70(0x28);
    local_4._0_1_ = 5;
    if (local_34 == (void *)0x0) {
      local_34 = (uint *)0x0;
    }
    else {
      local_34 = FUN_005fbda0(local_34,uVar4,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(local_34,0,0,uVar4,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    puVar10 = local_34;
    FUN_005fcc30(puVar7,local_34,(int)(uVar4 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,
                 0);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_34 = (uint *)FUN_00618b70(0x68);
    local_4._0_1_ = 6;
    if (local_34 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_0060ba00(local_34,2);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar5 + 0x3c) = 4;
    FUN_0060be60(pvVar5,(int)puVar10,0);
    if ((local_44 & 0x2000) == 0) {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x8789);
    }
    else {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x878a);
    }
    FUN_005f3010(local_2c,&local_34);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar5);
    local_34 = (uint *)FUN_00618b70(0x28);
    local_4._0_1_ = 7;
    if (local_34 == (void *)0x0) {
      puVar7 = (uint *)0x0;
    }
    else {
      puVar7 = FUN_005fbd20(local_34,uVar3,0x2b2f,2);
    }
    local_4._0_1_ = 1;
    local_34 = (uint *)FUN_00618b70(0x28);
    local_4._0_1_ = 8;
    if (local_34 == (void *)0x0) {
      local_34 = (uint *)0x0;
    }
    else {
      local_34 = FUN_005fbda0(local_34,uVar4,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(local_34,0,0,uVar4,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    puVar10 = local_34;
    FUN_005fcc30(puVar7,local_34,(int)(uVar4 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,
                 0);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_34 = (uint *)FUN_00618b70(0x68);
    local_4._0_1_ = 9;
    if (local_34 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_0060ba00(local_34,3);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar5 + 0x3c) = 4;
    FUN_0060be60(pvVar5,(int)puVar10,0);
    if ((local_44 & 0x1000) == 0) {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x878d);
    }
    else {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x878e);
    }
    FUN_005f3010(local_2c,&local_34);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar5);
  }
  local_34 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 10;
  if (local_34 == (void *)0x0) {
    puVar7 = (uint *)0x0;
  }
  else {
    puVar7 = FUN_005fbd20(local_34,uVar3,0x388,10);
  }
  local_4._0_1_ = 1;
  local_34 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xb;
  if (local_34 == (void *)0x0) {
    local_34 = (uint *)0x0;
  }
  else {
    local_34 = FUN_005fbda0(local_34,uVar4,iVar2,puVar7);
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005fcfe0(local_34,0,0,uVar4,iVar2,(char)puVar7[2]);
  iVar8 = FUN_005fc0e0((int *)puVar7);
  iVar9 = FUN_005fc0f0((int *)puVar7);
  puVar10 = local_34;
  FUN_005fcc30(puVar7,local_34,(int)(uVar4 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,0)
  ;
  if (puVar7 != (uint *)0x0) {
    FUN_005fbfa0(puVar7);
    FUN_00618b60((undefined *)puVar7);
  }
  local_34 = (uint *)FUN_00618b70(0x68);
  local_4._0_1_ = 0xc;
  if (local_34 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_0060ba00(local_34,4);
  }
  local_4._0_1_ = 1;
  *(undefined4 *)((int)pvVar5 + 0x3c) = 4;
  FUN_0060be60(pvVar5,(int)puVar10,0);
  if ((local_44 & 1) == 0) {
    local_34 = (uint *)CONCAT22(DAT_0065d424,0x8784);
  }
  else {
    local_34 = (uint *)CONCAT22(DAT_0065d424,0x8787);
  }
  FUN_005f3010(local_2c,&local_34);
  pcVar6 = (char *)FUN_00583c40((int)local_2c);
  FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar6);
  FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar5);
  if (local_30 != (void *)0x0) {
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 0xd;
    if (local_30 == (void *)0x0) {
      puVar7 = (uint *)0x0;
    }
    else {
      puVar7 = FUN_005fbd20(local_30,uVar3,0x2b14,2);
    }
    local_4._0_1_ = 1;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 0xe;
    if (local_30 == (void *)0x0) {
      local_34 = (uint *)0x0;
    }
    else {
      local_34 = FUN_005fbda0(local_30,uVar4,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(local_34,0,0,uVar4,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    puVar10 = local_34;
    FUN_005fcc30(puVar7,local_34,(int)(uVar4 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,
                 0);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_30 = (void *)FUN_00618b70(0x68);
    local_4._0_1_ = 0xf;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_0060ba00(local_30,5);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar5 + 0x3c) = 4;
    FUN_0060be60(pvVar5,(int)puVar10,0);
    if ((local_44 & 0x10) == 0) {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8794);
    }
    else {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8797);
    }
    FUN_005f3010(local_2c,&local_30);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar5);
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x10;
    if (local_30 == (void *)0x0) {
      puVar7 = (uint *)0x0;
    }
    else {
      puVar7 = FUN_005fbd20(local_30,uVar3,0x2b18,2);
    }
    local_4._0_1_ = 1;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 0x11;
    if (local_30 == (void *)0x0) {
      puVar10 = (uint *)0x0;
    }
    else {
      puVar10 = FUN_005fbda0(local_30,uVar4,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(puVar10,0,0,uVar4,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    FUN_005fcc30(puVar7,puVar10,(int)(uVar4 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,0
                );
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_30 = (void *)FUN_00618b70(0x68);
    local_4._0_1_ = 0x12;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_0060ba00(local_30,6);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar5 + 0x3c) = 4;
    FUN_0060be60(pvVar5,(int)puVar10,0);
    if ((local_44 & 0x20) == 0) {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8790);
    }
    else {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8793);
    }
    FUN_005f3010(local_2c,&local_30);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar5);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0051c780(local_20);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_2c);
  ExceptionList = local_c;
  return;
}

