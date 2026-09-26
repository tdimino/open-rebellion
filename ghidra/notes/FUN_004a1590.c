
int __fastcall FUN_004a1590(int *param_1)

{
  bool bVar1;
  int *this;
  undefined4 uVar2;
  int *piVar3;
  int iVar4;
  void *pvVar5;
  uint *puVar6;
  undefined4 *puVar7;
  undefined4 *puVar8;
  int iVar9;
  int iVar10;
  uint uVar11;
  void *pvVar12;
  uint *puVar13;
  uint *puVar14;
  char *pcVar15;
  void **ppvVar16;
  undefined1 *puVar17;
  undefined1 local_85;
  uint local_80;
  void *local_7c;
  int local_78;
  int local_74;
  int local_70;
  undefined1 local_6c [4];
  void *local_68;
  undefined4 local_64 [2];
  undefined4 local_5c [2];
  undefined4 local_54 [7];
  int local_38;
  undefined4 local_34 [10];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637787;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_006037f0(7);
  piVar3 = FUN_004a25c0(param_1);
  if (piVar3 == (int *)0x0) {
    iVar4 = 0;
  }
  else {
    FUN_005fbd20(local_34,uVar2,(*(int *)(param_1[0x53] + 0x9c) != 1) + 0x2b77,2);
    local_4 = 0;
    local_74 = 0;
    local_85 = 0;
    FUN_0053df50(local_64);
    local_4._0_1_ = 1;
    FUN_004f4340(local_5c);
    local_4._0_1_ = 2;
    FUN_00536da0(local_54,piVar3,3);
    local_4._0_1_ = 3;
    FUN_00513120((int)local_54);
    while (local_38 != 0) {
      ppvVar16 = &local_7c;
      pvVar5 = (void *)FUN_0052bed0((int)local_54);
      puVar6 = FUN_0042d170(pvVar5,ppvVar16);
      local_4._0_1_ = 4;
      iVar4 = FUN_004ece60(puVar6);
      if (iVar4 == 0) {
LAB_004a16e3:
        bVar1 = false;
      }
      else {
        iVar4 = FUN_0052bed0((int)local_54);
        local_68 = (void *)(*(uint *)(iVar4 + 0x78) >> 8 & 1);
        if (local_68 != (void *)0x0) goto LAB_004a16e3;
        piVar3 = &local_78;
        pvVar5 = (void *)FUN_0052bed0((int)local_54);
        puVar6 = FUN_0042d170(pvVar5,piVar3);
        local_4._0_1_ = 5;
        pvVar5 = FUN_004f5940(local_64,puVar6);
        local_4._0_1_ = 4;
        FUN_00619730();
        if (pvVar5 != (void *)0x0) goto LAB_004a16e3;
        bVar1 = true;
      }
      local_4._0_1_ = 3;
      FUN_00619730();
      if (bVar1) {
        piVar3 = &local_70;
        pvVar5 = (void *)FUN_0052bed0((int)local_54);
        puVar7 = FUN_004a25f0(pvVar5,piVar3);
        puVar6 = &local_80;
        local_4._0_1_ = 6;
        pvVar5 = (void *)FUN_0052bed0((int)local_54);
        puVar8 = FUN_0042d170(pvVar5,puVar6);
        local_4._0_1_ = 7;
        FUN_0053e020(local_64,puVar8,puVar7);
        local_4._0_1_ = 6;
        FUN_00619730();
        local_4._0_1_ = 3;
        FUN_00619730();
        iVar4 = FUN_0052bed0((int)local_54);
        uVar11 = *(uint *)(iVar4 + 0x24);
        puVar17 = local_6c;
        pvVar5 = (void *)FUN_0052bed0((int)local_54);
        puVar7 = FUN_0042d170(pvVar5,puVar17);
        local_4._0_1_ = 8;
        FUN_004f44b0(local_5c,puVar7,uVar11 >> 6 & 3);
        local_4._0_1_ = 3;
        FUN_00619730();
      }
      FUN_005130d0((int)local_54);
    }
    piVar3 = (int *)(**(code **)(*(int *)param_1[0x6d] + 8))();
    while (this = piVar3, this != (int *)0x0) {
      piVar3 = (int *)(**(code **)(*this + 0xc))();
      puVar6 = FUN_0042d170(this,local_6c);
      local_4._0_1_ = 9;
      pvVar5 = FUN_004f5940(local_64,puVar6);
      local_4._0_1_ = 3;
      FUN_00619730();
      if (pvVar5 == (void *)0x0) {
        FUN_004a0c20(param_1,(int)this);
        FUN_005f5ac0((void *)param_1[0x6d],(int)this);
        if (this != (int *)0x0) {
          (**(code **)*this)(1);
        }
        FUN_0060a280((void *)param_1[0x6e]);
      }
    }
    pvVar5 = (void *)thunk_FUN_005f5060((int)local_64);
    local_7c = pvVar5;
    iVar4 = thunk_FUN_005f5060((int)local_5c);
    while ((local_78 = iVar4, pvVar5 != (void *)0x0 && (iVar4 != 0))) {
      local_74 = 1;
      FUN_00403040(pvVar5,&local_80);
      iVar4 = *(int *)(iVar4 + 0x1c);
      local_4._0_1_ = 10;
      puVar6 = FUN_004a2610(pvVar5,local_6c);
      local_4._0_1_ = 0xb;
      iVar9 = FUN_0051cab0(puVar6);
      local_4._0_1_ = 10;
      local_70 = iVar9;
      FUN_00619730();
      if ((iVar9 != 0) &&
         (iVar10 = FUN_0060a860((void *)param_1[0x6d],local_80 & 0xffffff), iVar10 == 0)) {
        local_68 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0xc;
        if (local_68 == (void *)0x0) {
          puVar7 = (undefined4 *)0x0;
        }
        else {
          puVar7 = FUN_004acd80(local_68,&local_80,0);
        }
        local_4._0_1_ = 10;
        FUN_004acba0(param_1 + 0x45,puVar7,*(uint *)(param_1[0x6e] + 0x24));
        local_68 = (void *)FUN_00618b70(0x74);
        local_4._0_1_ = 0xd;
        if (local_68 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004421d0(local_68,&local_80);
        }
        local_4._0_1_ = 10;
        if (iVar4 == 1) {
          uVar11 = (*(uint *)(iVar9 + 0x30) & 0xfff) + 0x4000;
        }
        else {
          uVar11 = (*(uint *)(iVar9 + 0x30) & 0xfff) + 0x5000;
        }
        local_68 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0xe;
        if (local_68 == (void *)0x0) {
          puVar6 = (uint *)0x0;
        }
        else {
          puVar6 = FUN_005fbd20(local_68,param_1[0x6c],uVar11,2);
        }
        local_4._0_1_ = 10;
        pvVar12 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0xf;
        local_68 = pvVar12;
        if (pvVar12 == (void *)0x0) {
          puVar13 = (uint *)0x0;
        }
        else {
          puVar13 = puVar6;
          iVar4 = FUN_005fc0f0((int *)puVar6);
          iVar9 = FUN_005fc0e0((int *)puVar6);
          puVar13 = FUN_005fbda0(pvVar12,iVar9,iVar4,puVar13);
        }
        local_4._0_1_ = 10;
        pvVar12 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0x10;
        local_68 = pvVar12;
        if (pvVar12 == (void *)0x0) {
          puVar14 = (uint *)0x0;
        }
        else {
          puVar14 = puVar6;
          iVar4 = FUN_005fc0f0((int *)puVar6);
          iVar9 = FUN_005fc0e0((int *)puVar6);
          puVar14 = FUN_005fbda0(pvVar12,iVar9,iVar4,puVar14);
        }
        local_4._0_1_ = 10;
        FUN_005fcc30(puVar6,puVar13,0,0,0,0,(undefined4 *)0x0,0);
        FUN_005fcc30(puVar6,puVar14,0,0,0,0,(undefined4 *)0x0,0);
        FUN_005fd0f0(local_34,puVar14,0,0);
        local_85 = (undefined1)puVar6[2];
        pcVar15 = (char *)FUN_00583c40(local_70 + 0x34);
        FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar15);
        FUN_0060be60(pvVar5,(int)puVar13,(int)puVar14);
        FUN_005f59f0((void *)param_1[0x6d],(int)pvVar5);
        pvVar5 = local_7c;
        if (puVar6 != (uint *)0x0) {
          FUN_005fbfa0(puVar6);
          FUN_00618b60((undefined *)puVar6);
          pvVar5 = local_7c;
        }
      }
      local_4._0_1_ = 3;
      FUN_00619730();
      pvVar5 = *(void **)((int)pvVar5 + 0x10);
      local_7c = pvVar5;
      iVar4 = *(int *)(local_78 + 0x10);
    }
    FUN_004a2200(param_1);
    iVar4 = local_74;
    if (local_74 != 0) {
      *(undefined1 *)(param_1[0x6e] + 0xd4) = local_85;
      FUN_0060a280((void *)param_1[0x6e]);
    }
    local_4._0_1_ = 2;
    FUN_00536ea0(local_54);
    local_4._0_1_ = 1;
    FUN_004f4380(local_5c);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_0053df90(local_64);
    local_4 = 0xffffffff;
    FUN_005fbfa0(local_34);
  }
  ExceptionList = local_c;
  return iVar4;
}

