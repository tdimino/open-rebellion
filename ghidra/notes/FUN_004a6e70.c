
void __thiscall FUN_004a6e70(void *this,int *param_1)

{
  int *piVar1;
  byte bVar2;
  int iVar3;
  undefined4 uVar4;
  undefined *puVar5;
  char *pcVar6;
  uint *this_00;
  uint *puVar7;
  undefined3 extraout_var;
  void *pvVar8;
  undefined4 *this_01;
  uint *puVar9;
  uint uVar10;
  bool bVar11;
  undefined1 uVar12;
  uint local_20;
  uint *puStack_1c;
  void *pvStack_18;
  undefined4 *puStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637eff;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004025b0(param_1,&local_20);
  bVar11 = false;
  local_4 = 0;
  iVar3 = (**(code **)(**(int **)((int)this + 0x18c) + 8))();
  uVar4 = FUN_006037f0(7);
  for (; iVar3 != 0; iVar3 = *(int *)(iVar3 + 8)) {
    if (bVar11) goto LAB_004a719d;
    bVar11 = *(uint *)(iVar3 + 0x6c) == local_20;
  }
  if (!bVar11) {
    uVar10 = -(uint)(*(int *)((int)this + 0x170) != 1) & 0x32;
    if ((*(byte *)((int)this + 0x14c) & 4) == 0) {
      uVar10 = 0x32 - uVar10;
    }
    pvStack_18 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 1;
    if (pvStack_18 == (void *)0x0) {
      puStack_1c = (uint *)0x0;
    }
    else {
      puStack_1c = FUN_005fbd20(pvStack_18,uVar4,uVar10 + 0x28b4,2);
    }
    puVar7 = puStack_1c;
    local_4._0_1_ = 0;
    pvStack_18 = (void *)FUN_00618b70(0x70);
    puVar9 = (uint *)0x0;
    local_4._0_1_ = 2;
    if (pvStack_18 == (void *)0x0) {
      pvStack_18 = (void *)0x0;
    }
    else {
      pvStack_18 = FUN_004c7e10(pvStack_18,&local_20);
    }
    local_4._0_1_ = 0;
    puVar5 = FUN_004f62d0((int)param_1);
    pcVar6 = (char *)FUN_00583c40((int)puVar5);
    FUN_005f35e0((void *)((int)pvStack_18 + 0x14),pcVar6);
    puStack_14 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 3;
    if (puStack_14 == (void *)0x0) {
      this_00 = (uint *)0x0;
    }
    else {
      this_00 = FUN_005fbda0(puStack_14,0x7d,0x32,puVar7);
    }
    local_4._0_1_ = 0;
    puStack_14 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 4;
    if (puStack_14 != (void *)0x0) {
      puVar9 = FUN_005fbda0(puStack_14,0x7d,0x32,puVar7);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    puVar7 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x1b0),&local_20,(int *)0x0,0,1);
    uVar12 = (undefined1)puVar7[2];
    iVar3 = FUN_005fc0f0((int *)this_00);
    uVar10 = FUN_005fc0e0((int *)this_00);
    FUN_005fcfe0(this_00,0,0,uVar10,iVar3,uVar12);
    FUN_005fcc30(puVar7,this_00,0x1c,4,0,0,(undefined4 *)0x0,0);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    puStack_14 = (undefined4 *)0x14;
    uStack_10 = 0x1c;
    if ((local_20 >> 0x18 < 0x14) || (0x1b < local_20 >> 0x18)) {
      bVar11 = false;
    }
    else {
      bVar11 = true;
    }
    FUN_00619730();
    piVar1 = param_1;
    if (bVar11) {
      bVar2 = FUN_004a66a0(param_1);
      FUN_004a67a0(this_00,(uint)piVar1[9] >> 6 & 3,CONCAT31(extraout_var,bVar2),0);
    }
    FUN_005fcc30(this_00,puVar9,0,0,0,0,(undefined4 *)0x0,0);
    puVar7 = puStack_1c;
    FUN_005fd0f0(puStack_1c,puVar9,1,1);
    pvVar8 = pvStack_18;
    FUN_0060be60(pvStack_18,(int)this_00,(int)puVar9);
    if (puVar7 != (uint *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
    }
    FUN_005f59f0(*(void **)((int)this + 0x18c),(int)pvVar8);
    puVar7 = FUN_004025b0(param_1,(uint *)&pvStack_18);
    local_4._0_1_ = 5;
    pvVar8 = FUN_004f5940((void *)((int)this + 0x114),puVar7);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (pvVar8 == (void *)0x0) {
      puStack_14 = (undefined4 *)FUN_00618b70(0x28);
      local_4._0_1_ = 6;
      if (puStack_14 == (undefined4 *)0x0) {
        this_01 = (undefined4 *)0x0;
      }
      else {
        this_01 = FUN_004acd10(puStack_14);
      }
      local_4._0_1_ = 0;
      puVar7 = FUN_004025b0(param_1,(uint *)&param_1);
      local_4._0_1_ = 7;
      FUN_004f26d0(this_01 + 6,puVar7);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_004ace40(this_01,*(uint *)((int)this + 0x24));
      FUN_004acba0((void *)((int)this + 0x114),this_01,0);
    }
  }
LAB_004a719d:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

