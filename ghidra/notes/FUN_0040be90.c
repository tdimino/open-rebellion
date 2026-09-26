
uint __thiscall FUN_0040be90(void *this,int *param_1)

{
  float fVar1;
  undefined3 extraout_var;
  uint uVar2;
  uint uVar3;
  int iVar4;
  undefined *puVar5;
  uint *puVar6;
  uint *puVar7;
  undefined4 uVar8;
  uint uVar9;
  int iVar10;
  bool bVar11;
  undefined1 auStack_114 [8];
  undefined4 uStack_10c;
  undefined4 uStack_108;
  int iStack_104;
  float fStack_100;
  float fStack_fc;
  float fStack_f8;
  undefined4 uStack_f4;
  int iStack_f0;
  undefined4 uStack_ec;
  float fStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  uint auStack_84 [5];
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined1 *puStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  float fStack_54;
  undefined4 uStack_50;
  float fStack_4c;
  undefined4 uStack_48;
  float fStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  int *local_24;
  undefined4 local_20 [3];
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  undefined3 extraout_var_00;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a0f0;
  pvStack_c = ExceptionList;
  iVar10 = 0;
  bVar11 = *(int *)((int)this + 4) == 0;
  local_20[0] = 0;
  local_24 = (int *)0x0;
  uVar3 = 0;
  if (!bVar11) {
    uVar3 = (uint)(*(int *)((int)this + 8) != 0);
    bVar11 = uVar3 == 0;
  }
  ExceptionList = &pvStack_c;
  if (!bVar11) {
    uStack_3c = 0x40bee6;
    ExceptionList = &pvStack_c;
    bVar11 = FUN_004f6b50(param_1,(int *)&local_24);
    uVar3 = CONCAT31(extraout_var,bVar11);
  }
  if (uVar3 != 0) {
    local_14 = 0x14;
    local_10 = 0x1c;
    local_4 = 0;
    uVar2 = (**(code **)(*local_24 + 4))();
    if ((uVar2 < local_14) || (local_10 <= uVar2)) {
      iVar10 = 0;
    }
    else {
      iVar10 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar10 != 0) {
      uStack_3c = 0x40bf43;
      uVar3 = FUN_004f6df0(local_24,local_20);
    }
  }
  uVar2 = 0;
  if (uVar3 != 0) {
    (**(code **)(*param_1 + 0x240))();
    uStack_3c = 0x40bf6d;
    uStack_3c = (**(code **)(*param_1 + 0x23c))();
    uStack_40 = 0;
    fStack_44 = 5.94616e-39;
    iVar4 = (**(code **)(*param_1 + 0x200))();
    fStack_44 = (float)iVar4;
    uStack_48 = 1;
    fStack_4c = 5.946194e-39;
    local_4 = (**(code **)(*param_1 + 0x200))();
    fStack_4c = (float)local_4;
    uStack_50 = 2;
    fStack_54 = 5.946228e-39;
    puStack_8 = (undefined1 *)(**(code **)(*param_1 + 0x200))();
    fStack_54 = (float)(int)puStack_8;
    uStack_58 = 0;
    uStack_5c = 3;
    uStack_60 = 0x40bfc4;
    uStack_60 = (**(code **)(*param_1 + 0x1fc))();
    uStack_64 = 0;
    puStack_68 = (undefined1 *)0x2;
    uStack_6c = 0x40bfd3;
    uStack_6c = (**(code **)(*param_1 + 0x1fc))();
    uStack_70 = 0;
    auStack_84[4] = 1;
    auStack_84[3] = 0x40bfe2;
    auStack_84[3] = (**(code **)(*param_1 + 0x1fc))();
    auStack_84[2] = 0;
    auStack_84[1] = 0;
    auStack_84[0] = 0x40bff1;
    auStack_84[0] = (**(code **)(*param_1 + 0x1fc))();
    (**(code **)(*param_1 + 0x1fc))();
    (**(code **)(*param_1 + 0x1fc))();
    (**(code **)(*param_1 + 0x1fc))();
    (**(code **)(*param_1 + 0x1fc))();
    uStack_bc = 3;
    uStack_c0 = 0x40c03c;
    uStack_c0 = (**(code **)(*param_1 + 0x1fc))();
    uStack_c4 = 2;
    uStack_c8 = 2;
    uStack_cc = 0x40c04b;
    uStack_cc = (**(code **)(*param_1 + 0x1fc))();
    uStack_d0 = 2;
    uStack_d4 = 1;
    uStack_d8 = 0x40c05a;
    uStack_d8 = (**(code **)(*param_1 + 0x1fc))();
    uStack_dc = 2;
    uStack_e0 = 0;
    uStack_e4 = 0x40c069;
    uStack_e4 = (**(code **)(*param_1 + 0x1fc))();
    fStack_e8 = 5.946511e-39;
    iVar4 = (**(code **)(*param_1 + 0x1ec))();
    puStack_68 = (undefined1 *)(uint)(iVar4 != 0);
    fVar1 = (float)(int)puStack_68;
    fStack_e8 = 5.94655e-39;
    iVar4 = (**(code **)(*param_1 + 0x1f0))();
    puStack_68 = (undefined1 *)(uint)(iVar4 != 0);
    fStack_e8 = (float)(int)puStack_68 + fVar1;
    uStack_ec = 0;
    iStack_f0 = 0x40c0b5;
    iStack_f0 = FUN_0040d730(param_1);
    uStack_f4 = 0x40c0c0;
    uStack_f4 = (**(code **)(*param_1 + 0x244))();
    fStack_f8 = 5.946633e-39;
    puStack_68 = (undefined1 *)(**(code **)(*param_1 + 0x1e0))();
    fStack_f8 = (float)(int)puStack_68;
    fStack_fc = 5.946672e-39;
    puStack_68 = (undefined1 *)(**(code **)(*param_1 + 0x1e8))();
    fStack_fc = (float)(int)puStack_68;
    fStack_100 = 5.946711e-39;
    puStack_68 = (undefined1 *)(**(code **)(*param_1 + 0x1e4))();
    fStack_100 = (float)(int)puStack_68;
    uStack_108 = 3;
    uStack_10c = 0x40c122;
    iStack_104 = iVar10;
    puVar5 = FUN_004f62d0((int)param_1);
    puStack_68 = auStack_114;
    FUN_005f2f90(auStack_114,(int)puVar5);
    uStack_70 = 1;
    puVar6 = FUN_004025b0(param_1,auStack_84 + 1);
    uStack_70._0_1_ = 2;
    puVar7 = FUN_004025b0(param_1,auStack_84);
    uStack_70 = CONCAT31(uStack_70._1_3_,5);
    uVar3 = (**(code **)(*param_1 + 0x30))();
    uVar3 = uVar3 & 0xffff;
    uVar2 = FUN_0040d690((int)param_1);
    bVar11 = FUN_0040d660(uVar2);
    uVar8 = CONCAT31(extraout_var_00,bVar11);
    uVar2 = FUN_0040c4a0(puVar6);
    uVar9 = FUN_0040c490(puVar7);
    uStack_bc = 0x40c1a4;
    iVar10 = FUN_005964c0(uVar9,uVar2,uVar8,uVar3);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar2 = (uint)(iVar10 != 0);
    if (uVar2 != 0) {
      uStack_3c = 0x40c1de;
      FUN_004f7f60(param_1,iVar10);
    }
  }
  ExceptionList = pvStack_c;
  return uVar2;
}

