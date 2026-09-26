
void __fastcall FUN_00422620(void *param_1)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int *piVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  char *pcVar10;
  undefined *puVar11;
  uint *puVar12;
  bool bVar13;
  uint local_84;
  void *local_80;
  uint local_7c;
  uint local_78;
  void *local_74;
  undefined4 local_70;
  void *local_6c;
  undefined1 local_68 [8];
  uint local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c;
  undefined4 local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28 [3];
  int local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062bcd8;
  local_c = ExceptionList;
  local_7c = 0;
  ExceptionList = &local_c;
  local_80 = param_1;
  uVar2 = FUN_00401060();
  iVar3 = FUN_00401060();
  pvVar4 = (void *)FUN_004f3dd0(iVar3,uVar2);
  FUN_006037f0(7);
  FUN_00441520((int)param_1);
  FUN_004ece30(&local_70);
  local_4 = 0;
  FUN_004292e0(pvVar4,local_68);
  local_4._0_1_ = 1;
  FUN_00429200(param_1,0x14c,*(int *)((int)pvVar4 + 0x78));
  puVar5 = FUN_00429300(pvVar4,&local_30);
  local_4._0_1_ = 2;
  FUN_00520580(local_68,puVar5);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_00429200(param_1,0x14d,*(int *)((int)pvVar4 + 0x7c));
  puVar5 = FUN_0041bde0(pvVar4,&local_30);
  local_4._0_1_ = 3;
  FUN_00520580(local_68,puVar5);
  local_4._0_1_ = 1;
  FUN_00619730();
  piVar6 = FUN_0041bde0(pvVar4,&local_30);
  local_4._0_1_ = 4;
  FUN_00429200(param_1,0x14e,*piVar6 - *(int *)((int)pvVar4 + 0x74));
  local_4._0_1_ = 1;
  FUN_00619730();
  if (*(int *)((int)param_1 + 0x474) == 0) {
    FUN_004f3f70(local_1c);
    local_4._0_1_ = 5;
    FUN_0051caf0(local_1c);
    local_74 = (void *)FUN_00618b70(0x10);
    local_4._0_1_ = 6;
    if (local_74 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_0060a790(local_74,2);
    }
    local_4._0_1_ = 5;
    *(void **)((int)param_1 + 0x474) = pvVar4;
    local_84 = 0;
    pvVar4 = (void *)thunk_FUN_005f5060((int)local_1c);
    local_74 = pvVar4;
    FUN_004ece30(&local_78);
    local_4 = CONCAT31(local_4._1_3_,7);
    if (pvVar4 != (void *)0x0) {
      do {
        uVar2 = *(uint *)((int)pvVar4 + 0x24) >> 0x18;
        local_60 = 0x14;
        local_5c = 0x20;
        if ((uVar2 < 0x14) || (0x1f < uVar2)) {
          bVar13 = false;
        }
        else {
          bVar13 = true;
        }
        FUN_00619730();
        local_58 = 0x28;
        local_54 = 0x30;
        if ((uVar2 < 0x28) || (0x2f < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          bVar13 = true;
        }
        local_50 = 0x22;
        local_4c = 0x28;
        if ((uVar2 < 0x22) || (0x27 < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          bVar13 = true;
        }
        local_48 = 0x10;
        local_44 = 0x14;
        if ((uVar2 < 0x10) || (0x13 < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          bVar13 = true;
        }
        local_40 = 0x30;
        local_3c = 0x40;
        if ((uVar2 < 0x30) || (0x3f < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          bVar13 = true;
        }
        local_38 = 8;
        local_34 = 0x10;
        if ((uVar2 < 8) || (0xf < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          bVar13 = true;
        }
        local_30 = 0x50;
        local_2c = 0x80;
        if ((uVar2 < 0x50) || (0x7f < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if ((bVar1) && (iVar3 = FUN_0051cab0((uint *)((int)pvVar4 + 0x24)), iVar3 != 0)) {
          bVar13 = *(int *)(iVar3 + 0x5c) == 0;
        }
        if (bVar13) {
          iVar3 = FUN_0051cab0((uint *)((int)pvVar4 + 0x24));
          local_84 = *(uint *)(iVar3 + 0x30);
          iVar7 = (local_84 & 0xfff) + 0x1000;
          iVar8 = FUN_0060a860(*(void **)((int)local_80 + 0x474),iVar7);
          if (iVar8 == 0) {
            FUN_005f2f50(local_28);
            local_4 = CONCAT31(local_4._1_3_,8);
            FUN_005f3c20(local_28);
            local_84 = CONCAT22(local_84._2_2_,(short)local_84 + -0x8000);
            FUN_005f3010(local_28,&local_84);
            uVar9 = FUN_005f3070((int)local_28);
            if ((short)uVar9 == 0) {
              FUN_005f3090(local_28,iVar3 + 0x34);
            }
            local_78 = uVar2 << 0x18 | local_78 & 0xffffff;
            local_6c = (void *)FUN_00618b70(0x74);
            local_4._0_1_ = 9;
            if (local_6c == (void *)0x0) {
              pvVar4 = (void *)0x0;
            }
            else {
              pvVar4 = FUN_00442130(local_6c,iVar7,&local_78);
            }
            local_4._0_1_ = 8;
            pcVar10 = (char *)FUN_00583c40((int)local_28);
            FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar10);
            FUN_005f59f0(*(void **)((int)local_80 + 0x474),(int)pvVar4);
            local_4 = CONCAT31(local_4._1_3_,7);
            FUN_005f2ff0(local_28);
          }
        }
        pvVar4 = *(void **)((int)local_74 + 0x10);
        local_74 = pvVar4;
      } while (pvVar4 != (void *)0x0);
      local_74 = (void *)0x0;
      param_1 = local_80;
    }
    piVar6 = FUN_004f31b0(*(int *)((int)param_1 + 0x9c));
    while (piVar6 != (int *)0x0) {
      local_84 = *(uint *)(piVar6[0xb] + 0x30);
      iVar3 = (local_84 & 0xfff) + 0x1000;
      iVar7 = FUN_0060a860(*(void **)((int)local_80 + 0x474),iVar3);
      if (iVar7 == 0) {
        FUN_005f2f50(local_28);
        local_4 = CONCAT31(local_4._1_3_,10);
        FUN_005f3c20(local_28);
        local_84 = CONCAT22(local_84._2_2_,(short)local_84 + -0x8000);
        FUN_005f3010(local_28,&local_84);
        uVar2 = FUN_005f3070((int)local_28);
        if ((short)uVar2 == 0) {
          puVar11 = FUN_004f62d0((int)piVar6);
          FUN_005f3090(local_28,(int)puVar11);
        }
        pvVar4 = (void *)FUN_00618b70(0x74);
        local_4._0_1_ = 0xb;
        local_6c = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          puVar12 = FUN_004025b0(piVar6,(uint *)&local_74);
          local_7c = local_7c | 1;
          local_4 = CONCAT31(local_4._1_3_,0xc);
          pvVar4 = FUN_00442130(pvVar4,iVar3,puVar12);
        }
        local_4 = 0xb;
        if ((local_7c & 1) != 0) {
          local_7c = local_7c & 0xfffffffe;
          FUN_00619730();
        }
        local_4._0_1_ = 10;
        pcVar10 = (char *)FUN_00583c40((int)local_28);
        FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar10);
        FUN_005f59f0(*(void **)((int)local_80 + 0x474),(int)pvVar4);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_005f2ff0(local_28);
      }
      local_60 = 0x90;
      local_5c = 0x98;
      local_4._0_1_ = 0xd;
      piVar6 = (int *)FUN_004f6010(piVar6,&local_60,1);
      local_4 = CONCAT31(local_4._1_3_,7);
      FUN_00619730();
      param_1 = local_80;
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4._0_1_ = 5;
    FUN_00619730();
    local_4._0_1_ = 1;
    FUN_004f3fb0(local_1c);
  }
  *(undefined4 *)((int)param_1 + 0xa4) = 0;
  FUN_0042d4d0((int)param_1);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

