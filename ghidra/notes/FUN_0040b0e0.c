
uint __fastcall FUN_0040b0e0(void *param_1)

{
  bool bVar1;
  int iVar2;
  void *pvVar3;
  uint *puVar4;
  uint uVar5;
  undefined *puVar6;
  undefined4 uVar7;
  undefined3 extraout_var;
  int iVar8;
  undefined3 extraout_var_00;
  int *piVar9;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  uint uVar10;
  undefined1 auStack_1c4 [8];
  undefined4 uStack_1bc;
  float local_1b8;
  float local_1b4;
  int iStack_1b0;
  undefined4 uStack_1a4;
  undefined4 uVar11;
  undefined1 *local_17c;
  void *local_178;
  int local_174;
  int local_170;
  int local_16c;
  int local_168;
  undefined4 local_164;
  uint local_160;
  uint local_15c;
  undefined4 local_158;
  undefined1 *local_154;
  undefined4 local_150;
  undefined4 local_14c [7];
  int local_130;
  undefined4 local_12c [7];
  int local_110;
  undefined1 *local_10c;
  undefined4 local_108 [7];
  int local_ec;
  undefined4 local_e8 [7];
  int local_cc;
  undefined4 local_c8 [7];
  int local_ac;
  undefined4 local_a8 [7];
  int local_8c;
  undefined4 local_88 [7];
  int local_6c;
  undefined4 local_68 [7];
  int iStack_4c;
  undefined4 local_48 [7];
  int local_2c;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a058;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)param_1 + 8) = 0;
  FUN_004fd400(local_28);
  local_4 = 0;
  iVar2 = FUN_00506e10();
  uVar10 = (uint)(iVar2 != 0);
  if (uVar10 != 0) {
    pvVar3 = (void *)FUN_00516ee0(iVar2);
    FUN_004fd490(local_28,pvVar3);
    if (uVar10 != 0) {
      FUN_004ece30(&local_17c);
      local_4._0_1_ = 1;
      FUN_0041e1d0(&local_17c);
      iVar2 = FUN_00505f60((uint *)&local_17c);
      *(int *)((int)param_1 + 8) = iVar2;
      uVar10 = (uint)(iVar2 != 0);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  }
  if ((*(int *)((int)param_1 + 4) != 0) && (*(void **)((int)param_1 + 8) != (void *)0x0)) {
    FUN_00509460(*(void **)((int)param_1 + 8),1,(int *)&local_178);
    if (local_178 != (void *)0x0) {
      puVar4 = FUN_004025b0(local_178,(uint *)&local_17c);
      local_4._0_1_ = 2;
      FUN_004f26d0((void *)((int)param_1 + 0x18),puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    FUN_00509530(*(void **)((int)param_1 + 8),1,(int *)&local_178);
    if (local_178 != (void *)0x0) {
      puVar4 = FUN_004025b0(local_178,(uint *)&local_17c);
      local_4._0_1_ = 3;
      FUN_004f26d0((void *)((int)param_1 + 0x1c),puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    FUN_00509460(*(void **)((int)param_1 + 8),2,(int *)&local_178);
    if (local_178 != (void *)0x0) {
      puVar4 = FUN_004025b0(local_178,(uint *)&local_17c);
      local_4._0_1_ = 4;
      FUN_004f26d0((void *)((int)param_1 + 0x20),puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    FUN_00509530(*(void **)((int)param_1 + 8),2,(int *)&local_178);
    if (local_178 != (void *)0x0) {
      puVar4 = FUN_004025b0(local_178,(uint *)&local_17c);
      local_4._0_1_ = 5;
      FUN_004f26d0((void *)((int)param_1 + 0x24),puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    local_15c = 0;
    local_160 = 0;
    local_164 = 0;
    local_158 = 0;
    local_16c = 0;
    local_174 = 0;
    local_168 = 0;
    local_170 = 0;
    uStack_1a4 = 0x40b2f1;
    FUN_0041e1f0(1,&local_15c,&local_164,&local_16c,&local_168);
    uStack_1a4 = 0x40b30f;
    FUN_0041e1f0(2,&local_160,&local_158,&local_174,&local_170);
    FUN_0040d690(*(int *)((int)param_1 + 8));
    FUN_0040d690(*(int *)((int)param_1 + 8));
    FUN_0040d6a0(*(int *)((int)param_1 + 8));
    uVar5 = FUN_0040d750(*(int *)((int)param_1 + 8));
    uStack_1a4 = 0x40b34a;
    FUN_0040d660(uVar5);
    local_17c = (undefined1 *)&uStack_1a4;
    FUN_004fcd90(&uStack_1a4,(undefined4 *)((int)param_1 + 0x14));
    local_10c = &stack0xfffffe58;
    local_4._0_1_ = 6;
    iStack_1b0 = 0x40b379;
    FUN_004fcd90(&stack0xfffffe58,(undefined4 *)((int)param_1 + 0x10));
    local_1b4 = (float)local_170;
    iStack_1b0 = local_16c;
    local_4._0_1_ = 7;
    local_1b8 = (float)local_168;
    uStack_1bc = 0x40b3a3;
    puVar6 = FUN_004f62d0(*(int *)((int)param_1 + 8));
    local_154 = auStack_1c4;
    FUN_005f2f90(auStack_1c4,(int)puVar6);
    local_4._0_1_ = 0;
    uVar7 = FUN_00509610(*(int *)((int)param_1 + 8));
    bVar1 = FUN_0041cdd0();
    iVar2 = CONCAT31(extraout_var,bVar1);
    iVar8 = FUN_0041cdb0();
    bVar1 = FUN_0040d660(iVar8);
    FUN_00595d60(CONCAT31(extraout_var_00,bVar1),iVar2,uVar7);
    FUN_004ffe70(local_12c,*(undefined4 *)((int)param_1 + 8),3);
    local_4 = CONCAT31(local_4._1_3_,9);
    FUN_00513120((int)local_12c);
    while (local_110 != 0) {
      uVar11 = 1;
      uVar7 = FUN_0052bed0((int)local_12c);
      FUN_005001b0(local_48,uVar7,uVar11);
      local_4 = CONCAT31(local_4._1_3_,10);
      FUN_00513120((int)local_48);
      while (local_2c != 0) {
        piVar9 = (int *)FUN_0052bed0((int)local_48);
        bVar1 = FUN_0040b8a0(param_1,piVar9);
        uVar10 = CONCAT31(extraout_var_01,bVar1);
        FUN_005130d0((int)local_48);
      }
      uVar11 = 1;
      uVar7 = FUN_0052bed0((int)local_12c);
      FUN_005003d0(local_a8,uVar7,uVar11);
      local_4 = CONCAT31(local_4._1_3_,0xb);
      FUN_00513120((int)local_a8);
      while (local_8c != 0) {
        piVar9 = (int *)FUN_0052bed0((int)local_a8);
        bVar1 = FUN_0040bcd0(param_1,piVar9);
        uVar10 = CONCAT31(extraout_var_02,bVar1);
        FUN_005130d0((int)local_a8);
      }
      pvVar3 = (void *)FUN_00509670(*(void **)((int)param_1 + 8),0);
      if (pvVar3 != (void *)0x0) {
        FUN_0040d760(pvVar3,&local_17c);
        local_4 = CONCAT31(local_4._1_3_,0xc);
        iVar2 = FUN_004ece60((uint *)&local_17c);
        if (iVar2 != 0) {
          local_154 = (undefined1 *)0x18;
          local_150 = 0x1c;
          if (((uint)local_17c >> 0x18 < 0x18) || (0x1b < (uint)local_17c >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (((bVar1) && (piVar9 = (int *)FUN_00506260((uint *)&local_17c), piVar9 != (int *)0x0))
             && ((*(byte *)(piVar9 + 0x14) & 4) == 0)) {
            bVar1 = FUN_0040bcd0(param_1,piVar9);
            uVar10 = CONCAT31(extraout_var_03,bVar1);
          }
        }
        local_4 = CONCAT31(local_4._1_3_,0xb);
        FUN_00619730();
      }
      uVar11 = 1;
      uVar7 = FUN_0052bed0((int)local_12c);
      FUN_00502db0(local_14c,uVar7,uVar11);
      local_4 = CONCAT31(local_4._1_3_,0xd);
      FUN_00513120((int)local_14c);
      while (local_130 != 0) {
        uVar11 = 1;
        uVar7 = FUN_0052bed0((int)local_14c);
        FUN_005039d0(local_e8,uVar7,uVar11);
        local_4 = CONCAT31(local_4._1_3_,0xe);
        FUN_00513120((int)local_e8);
        while (local_cc != 0) {
          piVar9 = (int *)FUN_0052bed0((int)local_e8);
          uVar10 = FUN_0040be90(param_1,piVar9);
          FUN_005130d0((int)local_e8);
        }
        uVar11 = 1;
        uVar7 = FUN_0052bed0((int)local_14c);
        FUN_004f25c0(local_108,uVar7,uVar11);
        local_4 = CONCAT31(local_4._1_3_,0xf);
        FUN_00513120((int)local_108);
        while (local_ec != 0) {
          piVar9 = (int *)FUN_0052bed0((int)local_108);
          uVar10 = FUN_0040c200(param_1,piVar9);
          FUN_005130d0((int)local_108);
        }
        uVar11 = 1;
        uVar7 = FUN_0052bed0((int)local_14c);
        FUN_00504040(local_88,uVar7,uVar11);
        local_4 = CONCAT31(local_4._1_3_,0x10);
        FUN_00513120((int)local_88);
        while (local_6c != 0) {
          piVar9 = (int *)FUN_0052bed0((int)local_88);
          uVar10 = FUN_0040c4b0(param_1,piVar9);
          FUN_005130d0((int)local_88);
        }
        uVar11 = 1;
        uVar7 = FUN_0052bed0((int)local_14c);
        FUN_00504c40(local_c8,uVar7,uVar11);
        local_4 = CONCAT31(local_4._1_3_,0x11);
        FUN_00513120((int)local_c8);
        while (local_ac != 0) {
          piVar9 = (int *)FUN_0052bed0((int)local_c8);
          uVar10 = FUN_0040c5f0(param_1,piVar9);
          FUN_005130d0((int)local_c8);
        }
        local_4._1_3_ = (uint3)((uint)local_4 >> 8);
        local_4._0_1_ = 0x10;
        FUN_00504d40(local_c8);
        local_4._0_1_ = 0xf;
        FUN_00504140(local_88);
        local_4._0_1_ = 0xe;
        FUN_004f26c0(local_108);
        local_4 = CONCAT31(local_4._1_3_,0xd);
        FUN_00503ad0(local_e8);
        FUN_005130d0((int)local_14c);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 0xb;
      FUN_00502eb0(local_14c);
      local_4._0_1_ = 10;
      FUN_005004d0(local_a8);
      local_4 = CONCAT31(local_4._1_3_,9);
      FUN_00500230(local_48);
      FUN_005130d0((int)local_12c);
    }
    FUN_005039d0(local_68,*(undefined4 *)((int)param_1 + 8),1);
    local_4 = CONCAT31(local_4._1_3_,0x12);
    FUN_00513120((int)local_68);
    while (iStack_4c != 0) {
      piVar9 = (int *)FUN_0052bed0((int)local_68);
      uVar10 = FUN_0040be90(param_1,piVar9);
      FUN_005130d0((int)local_68);
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4._0_1_ = 9;
    FUN_00503ad0(local_68);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_004fff70(local_12c);
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return uVar10;
}

