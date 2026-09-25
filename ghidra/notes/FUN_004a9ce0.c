
void __fastcall FUN_004a9ce0(int *param_1)

{
  undefined4 *puVar1;
  bool bVar2;
  undefined4 uVar3;
  int *piVar4;
  void *pvVar5;
  int iVar6;
  int iVar7;
  uint *puVar8;
  uint uVar9;
  uint uVar10;
  undefined4 local_38;
  int *local_34;
  void *local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063828b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_38);
  local_4 = 0;
  uVar3 = FUN_006037f0(7);
  piVar4 = FUN_004a25c0(param_1);
  local_34 = piVar4;
  if (piVar4 == (int *)0x0) goto LAB_004aa27e;
  puVar1 = (undefined4 *)param_1[0x5f];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x60];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x5d];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x5e];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  iVar6 = param_1[0x52];
  if (iVar6 == 1) {
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 1;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_30,uVar3,0x283b,10);
    }
    local_4._0_1_ = 0;
    param_1[0x5f] = (int)pvVar5;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 2;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_30,uVar3,0x27d8,10);
    }
    local_4._0_1_ = 0;
    param_1[0x60] = (int)pvVar5;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,3);
    if (local_30 == (void *)0x0) {
LAB_004a9f77:
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_30,uVar3,0x2952,10);
    }
  }
  else if (iVar6 == 2) {
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 4;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_30,uVar3,0x27d9,10);
    }
    local_4._0_1_ = 0;
    param_1[0x5f] = (int)pvVar5;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 5;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_30,uVar3,0x283e,10);
    }
    local_4._0_1_ = 0;
    param_1[0x60] = (int)pvVar5;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,6);
    if (local_30 == (void *)0x0) goto LAB_004a9f77;
    pvVar5 = FUN_005fbd20(local_30,uVar3,0x2953,10);
  }
  else {
    if (iVar6 != 3) goto LAB_004aa27e;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 7;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_30,uVar3,0x283f,10);
    }
    local_4._0_1_ = 0;
    param_1[0x5f] = (int)pvVar5;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 8;
    if (local_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(local_30,uVar3,0x2840,10);
    }
    local_4._0_1_ = 0;
    param_1[0x60] = (int)pvVar5;
    local_30 = (void *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,9);
    if (local_30 == (void *)0x0) goto LAB_004a9f77;
    pvVar5 = FUN_005fbd20(local_30,uVar3,0x2953,10);
  }
  param_1[0x5d] = (int)pvVar5;
  local_4 = local_4 & 0xffffff00;
  if (param_1[0x58] != 0) {
    iVar6 = FUN_005273d0(piVar4,param_1[0x52],3);
    iVar7 = FUN_00527150(piVar4,param_1[0x52],3);
    pvVar5 = (void *)FUN_00604500((void *)(param_1[0x58] + 0x6c),5);
    FUN_00603150(pvVar5,0,(-(uint)(iVar6 + iVar7 != 0) & 0xfffffffe) + 0x2938);
    iVar6 = FUN_0051c0d0(piVar4,param_1[0x52],3);
    iVar7 = FUN_005276d0(piVar4,param_1[0x52],3);
    pvVar5 = (void *)FUN_00604500((void *)(param_1[0x58] + 0x6c),4);
    FUN_00603150(pvVar5,0,(-(uint)(iVar6 + iVar7 != 0) & 0xfffffffe) + 0x293b);
    iVar6 = FUN_00503550(piVar4,param_1[0x52],3);
    pvVar5 = (void *)FUN_00604500((void *)(param_1[0x58] + 0x6c),3);
    if (param_1[0x52] == 1) {
      uVar9 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x293e;
      uVar10 = 0x293d;
    }
    else if (param_1[0x52] == 2) {
      uVar9 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x2941;
      uVar10 = 0x2940;
    }
    else {
      uVar9 = 0x2942;
      uVar10 = 0x2942;
    }
    FUN_00603150(pvVar5,1,uVar10);
    FUN_00603150(pvVar5,4,uVar10);
    FUN_00603150(pvVar5,0,uVar9);
    iVar6 = FUN_005044f0(local_34,param_1[0x52],3);
    pvVar5 = (void *)FUN_00604500((void *)(param_1[0x58] + 0x6c),2);
    if (param_1[0x52] == 1) {
      uVar9 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x2945;
      uVar10 = 0x2944;
    }
    else if (param_1[0x52] == 2) {
      uVar9 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x2948;
      uVar10 = 0x2947;
    }
    else {
      uVar9 = 0x2949;
      uVar10 = 0x2949;
    }
    FUN_00603150(pvVar5,1,uVar10);
    FUN_00603150(pvVar5,4,uVar10);
    FUN_00603150(pvVar5,0,uVar9);
    iVar6 = 0;
    FUN_00536e20(local_2c,local_34,3,param_1[0x52]);
    local_4 = CONCAT31(local_4._1_3_,10);
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      pvVar5 = (void *)FUN_0052bed0((int)local_2c);
      local_30 = (void *)0x0;
      puVar8 = FUN_0042d170(pvVar5,&local_34);
      local_4._0_1_ = 0xb;
      iVar7 = FUN_004ece60(puVar8);
      if ((iVar7 == 0) || ((*(uint *)((int)pvVar5 + 0x78) & 0x100) != 0)) {
        bVar2 = true;
      }
      else {
        bVar2 = false;
      }
      local_4 = CONCAT31(local_4._1_3_,10);
      FUN_00619730();
      pvVar5 = (void *)0x1;
      if (!bVar2) {
        pvVar5 = local_30;
      }
      if (pvVar5 != (void *)0x0) {
        iVar6 = iVar6 + 1;
      }
      FUN_005130d0((int)local_2c);
    }
    pvVar5 = (void *)FUN_00604500((void *)(param_1[0x58] + 0x6c),1);
    if (param_1[0x52] == 1) {
      uVar10 = 0x294b;
      uVar9 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x294c;
    }
    else if (param_1[0x52] == 2) {
      uVar10 = 0x294e;
      uVar9 = (-(uint)(iVar6 != 0) & 0xfffffffe) + 0x294f;
    }
    else {
      uVar9 = 0x2950;
      uVar10 = 0x2950;
    }
    FUN_00603150(pvVar5,1,uVar10);
    FUN_00603150(pvVar5,4,uVar10);
    FUN_00603150(pvVar5,0,uVar9);
    local_4 = local_4 & 0xffffff00;
    FUN_00536ea0(local_2c);
  }
  if ((void *)param_1[0x59] != (void *)0x0) {
    if (param_1[0x52] == 1) {
      uVar3 = 0x20000ff;
    }
    else if (param_1[0x52] == 2) {
      uVar3 = 0x200ff00;
    }
    else {
      uVar3 = 0x2ffff00;
    }
    FUN_00609940((void *)param_1[0x59],uVar3);
  }
LAB_004aa27e:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

