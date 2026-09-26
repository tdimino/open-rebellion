
void __fastcall FUN_0044bb70(int param_1)

{
  undefined1 uVar1;
  int iVar2;
  undefined4 uVar3;
  int *piVar4;
  uint uVar5;
  char *pcVar6;
  uint *puVar7;
  int iVar8;
  int iVar9;
  uint *puVar10;
  void *pvVar11;
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
  puStack_8 = &LAB_0062f5eb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_004f3dd0(2,2);
  FUN_005f2f50(local_2c);
  pvVar11 = (void *)0x0;
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
  if (local_34 == (void *)0x0) {
    piVar4 = (int *)0x0;
  }
  else {
    piVar4 = FUN_005fbd20(local_34,uVar3,0x27fa,10);
  }
  uVar1 = (undefined1)piVar4[2];
  local_4._0_1_ = 1;
  uVar5 = FUN_005fc0e0(piVar4);
  iVar2 = FUN_005fc0f0(piVar4);
  local_34 = (uint *)FUN_00618b70(0x68);
  local_4._0_1_ = 3;
  if (local_34 != (void *)0x0) {
    pvVar11 = FUN_0060ba00(local_34,4);
  }
  local_4._0_1_ = 1;
  *(undefined4 *)((int)pvVar11 + 0x3c) = 4;
  FUN_0060be60(pvVar11,(int)piVar4,0);
  if ((local_44 & 0x300) == 0) {
    local_34._0_2_ = 0x8780;
  }
  else {
    local_34._0_2_ = 0x8783;
  }
  local_34 = (uint *)CONCAT22(DAT_0065d424,(undefined2)local_34);
  FUN_005f3010(local_2c,&local_34);
  pcVar6 = (char *)FUN_00583c40((int)local_2c);
  FUN_005f35e0((void *)((int)pvVar11 + 0x14),pcVar6);
  FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar11);
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
      local_34 = FUN_005fbda0(local_34,uVar5,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(local_34,0,0,uVar5,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    puVar10 = local_34;
    FUN_005fcc30(puVar7,local_34,(int)(uVar5 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,
                 0);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_34 = (uint *)FUN_00618b70(0x68);
    local_4._0_1_ = 6;
    if (local_34 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_0060ba00(local_34,5);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar11 + 0x3c) = 4;
    FUN_0060be60(pvVar11,(int)puVar10,0);
    if ((local_44 & 0x2000) == 0) {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x8788);
    }
    else {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x878b);
    }
    FUN_005f3010(local_2c,&local_34);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar11 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar11);
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
      local_34 = FUN_005fbda0(local_34,uVar5,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(local_34,0,0,uVar5,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    puVar10 = local_34;
    FUN_005fcc30(puVar7,local_34,(int)(uVar5 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,
                 0);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_34 = (uint *)FUN_00618b70(0x68);
    local_4._0_1_ = 9;
    if (local_34 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_0060ba00(local_34,6);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar11 + 0x3c) = 4;
    FUN_0060be60(pvVar11,(int)puVar10,0);
    if ((local_44 & 0x1000) == 0) {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x878c);
    }
    else {
      local_34 = (uint *)CONCAT22(DAT_0065d424,0x878f);
    }
    FUN_005f3010(local_2c,&local_34);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar11 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar11);
  }
  local_34 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 10;
  if (local_34 == (void *)0x0) {
    puVar7 = (uint *)0x0;
  }
  else {
    puVar7 = FUN_005fbd20(local_34,uVar3,0x388,2);
  }
  local_4._0_1_ = 1;
  local_34 = (uint *)FUN_00618b70(0x28);
  local_4._0_1_ = 0xb;
  if (local_34 == (void *)0x0) {
    local_34 = (uint *)0x0;
  }
  else {
    local_34 = FUN_005fbda0(local_34,uVar5,iVar2,puVar7);
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005fcfe0(local_34,0,0,uVar5,iVar2,(char)puVar7[2]);
  iVar8 = FUN_005fc0e0((int *)puVar7);
  iVar9 = FUN_005fc0f0((int *)puVar7);
  puVar10 = local_34;
  FUN_005fcc30(puVar7,local_34,(int)(uVar5 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,0)
  ;
  if (puVar7 != (uint *)0x0) {
    FUN_005fbfa0(puVar7);
    FUN_00618b60((undefined *)puVar7);
  }
  local_34 = (uint *)FUN_00618b70(0x68);
  local_4._0_1_ = 0xc;
  if (local_34 == (void *)0x0) {
    pvVar11 = (void *)0x0;
  }
  else {
    pvVar11 = FUN_0060ba00(local_34,1);
  }
  local_4._0_1_ = 1;
  *(undefined4 *)((int)pvVar11 + 0x3c) = 4;
  FUN_0060be60(pvVar11,(int)puVar10,0);
  if ((local_44 & 1) == 0) {
    local_34 = (uint *)CONCAT22(DAT_0065d424,0x8785);
  }
  else {
    local_34 = (uint *)CONCAT22(DAT_0065d424,0x8786);
  }
  FUN_005f3010(local_2c,&local_34);
  pcVar6 = (char *)FUN_00583c40((int)local_2c);
  FUN_005f35e0((void *)((int)pvVar11 + 0x14),pcVar6);
  FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar11);
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
      local_34 = FUN_005fbda0(local_30,uVar5,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(local_34,0,0,uVar5,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    puVar10 = local_34;
    FUN_005fcc30(puVar7,local_34,(int)(uVar5 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,
                 0);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_30 = (void *)FUN_00618b70(0x68);
    local_4._0_1_ = 0xf;
    if (local_30 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_0060ba00(local_30,2);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar11 + 0x3c) = 4;
    FUN_0060be60(pvVar11,(int)puVar10,0);
    if ((local_44 & 0x10) == 0) {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8795);
    }
    else {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8796);
    }
    FUN_005f3010(local_2c,&local_30);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar11 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar11);
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
      puVar10 = FUN_005fbda0(local_30,uVar5,iVar2,puVar7);
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005fcfe0(puVar10,0,0,uVar5,iVar2,uVar1);
    iVar8 = FUN_005fc0e0((int *)puVar7);
    iVar9 = FUN_005fc0f0((int *)puVar7);
    FUN_005fcc30(puVar7,puVar10,(int)(uVar5 - iVar8) / 2,(iVar2 - iVar9) / 2,0,0,(undefined4 *)0x0,0
                );
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    local_30 = (void *)FUN_00618b70(0x68);
    local_4._0_1_ = 0x12;
    if (local_30 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_0060ba00(local_30,3);
    }
    local_4._0_1_ = 1;
    *(undefined4 *)((int)pvVar11 + 0x3c) = 4;
    FUN_0060be60(pvVar11,(int)puVar10,0);
    if ((local_44 & 0x20) == 0) {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8791);
    }
    else {
      local_30 = (void *)CONCAT22(DAT_0065d424,0x8792);
    }
    FUN_005f3010(local_2c,&local_30);
    pcVar6 = (char *)FUN_00583c40((int)local_2c);
    FUN_005f35e0((void *)((int)pvVar11 + 0x14),pcVar6);
    FUN_005f59f0(*(void **)(param_1 + 0x11c),(int)pvVar11);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0051c780(local_20);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_2c);
  ExceptionList = local_c;
  return;
}

