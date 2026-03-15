
int FUN_00519d00(void)

{
  bool bVar1;
  short sVar2;
  short sVar3;
  short sVar4;
  short sVar5;
  short sVar6;
  short sVar7;
  short sVar8;
  int iVar9;
  void *pvVar10;
  uint *puVar11;
  void *pvVar12;
  undefined4 *puVar13;
  int *piVar14;
  undefined2 extraout_var;
  int iVar15;
  int iVar16;
  uint uVar17;
  undefined2 extraout_var_00;
  int iVar18;
  undefined2 extraout_var_01;
  undefined2 extraout_var_02;
  undefined2 extraout_var_03;
  undefined2 extraout_var_04;
  undefined2 extraout_var_05;
  undefined2 extraout_var_06;
  undefined2 extraout_var_07;
  int iVar19;
  uint local_54;
  uint local_50;
  int local_4c;
  int local_48;
  uint local_44;
  uint local_40;
  uint local_3c;
  undefined4 local_38;
  uint local_34;
  uint local_30;
  undefined1 *local_2c;
  undefined4 local_28;
  undefined4 local_24 [2];
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642d80;
  local_c = ExceptionList;
  local_30 = 0;
  local_3c = 0;
  local_40 = 0;
  ExceptionList = &local_c;
  iVar9 = FUN_0053f640();
  if (iVar9 != 0) {
    FUN_004f4340(local_24);
    local_4 = 0;
    pvVar10 = (void *)FUN_005069d0();
    while (pvVar10 != (void *)0x0) {
      puVar11 = FUN_004025b0(pvVar10,&local_44);
      local_4._0_1_ = 2;
      FUN_004f44b0(local_24,puVar11,0);
      local_4._0_1_ = 0;
      FUN_00619730();
      local_2c = (undefined1 *)0x80;
      local_28 = 0x90;
      local_4._0_1_ = 1;
      pvVar10 = (void *)FUN_004f6010(pvVar10,(uint *)&local_2c,1);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    FUN_004f3f70(local_1c);
    local_4 = CONCAT31(local_4._1_3_,3);
    local_44 = 0;
    for (pvVar10 = (void *)FUN_0051cad0(0x90); pvVar10 != (void *)0x0;
        pvVar10 = *(void **)((int)pvVar10 + 0x10)) {
      local_54 = local_54 & 0xffffff | 0x80000000;
      local_54 = local_54 ^ (*(uint *)((int)pvVar10 + 0x40) ^ local_54) & 0xffffff;
      local_4._1_3_ = (uint3)(local_4 >> 8);
      local_4._0_1_ = 4;
      iVar9 = 0;
      pvVar12 = FUN_004f5940(local_24,&local_54);
      if (pvVar12 != (void *)0x0) {
        iVar9 = FUN_00506a40(&local_54);
      }
      if (iVar9 != 0) {
        local_44 = local_44 + 1;
        iVar9 = FUN_00526550(iVar9);
        local_2c = &stack0xffffff98;
        if (iVar9 == 1) {
          local_2c = &stack0xffffff98;
          FUN_00402e40(pvVar10,(uint *)&stack0xffffff98);
          puVar13 = local_1c;
        }
        else {
          FUN_00402e40(pvVar10,(uint *)&stack0xffffff98);
          puVar13 = (undefined4 *)(local_4c + 0x130);
        }
        puVar13 = FUN_004f42a0(puVar13);
        piVar14 = (int *)FUN_00402e40(pvVar10,&local_50);
        local_48 = 0x92000121;
        iVar9 = *piVar14;
        local_4._0_1_ = 5;
        FUN_00619730();
        local_4._0_1_ = 4;
        FUN_00619730();
        if (iVar9 == -0x6dfffedf) {
          local_3c = puVar13[6];
        }
        else {
          piVar14 = (int *)FUN_00402e40(pvVar10,&local_34);
          local_38 = 0x90000109;
          iVar9 = *piVar14;
          local_4._0_1_ = 6;
          FUN_00619730();
          local_4._0_1_ = 4;
          FUN_00619730();
          if (iVar9 == -0x6ffffef7) {
            local_40 = puVar13[6];
          }
        }
      }
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
    }
    sVar2 = FUN_004f4290((int)local_1c);
    iVar19 = CONCAT22(extraout_var,sVar2);
    iVar9 = FUN_005597e0(1,0);
    iVar15 = FUN_005597e0(2,0);
    iVar16 = FUN_005597e0(1,1);
    local_34 = FUN_005597e0(2,1);
    local_54 = FUN_0053e190(iVar19,iVar9);
    uVar17 = FUN_0053e190(iVar19,iVar16);
    local_50 = uVar17;
    iVar9 = FUN_0053e190(iVar19,iVar15);
    local_48 = FUN_0053e190(iVar19,local_34);
    iVar16 = local_4c;
    iVar15 = (((iVar19 - local_54) - uVar17) - iVar9) - local_48;
    sVar2 = FUN_004f4290(local_4c + 0x130);
    local_34 = CONCAT22(extraout_var_00,sVar2);
    local_4c = iVar9 + -1;
    iVar19 = iVar19 + -1;
    pvVar10 = (void *)FUN_005f54d0(local_1c,local_40);
    FUN_005f5600(pvVar10,0);
    FUN_004f4270((void *)(iVar16 + 0x140),pvVar10);
    pvVar10 = (void *)FUN_005f54d0((void *)(iVar16 + 0x130),local_3c);
    FUN_005f5600(pvVar10,0);
    FUN_004f4270((void *)(iVar16 + 0x140),pvVar10);
    iVar9 = FUN_0053e290(local_34 - 2);
    pvVar10 = (void *)thunk_FUN_005f5060(iVar16 + 0x130);
    for (; uVar17 = local_30, iVar9 != 0; iVar9 = iVar9 + -1) {
      if (pvVar10 == (void *)0x0) goto LAB_0051a087;
      pvVar10 = *(void **)((int)pvVar10 + 0x10);
    }
    if (pvVar10 != (void *)0x0) {
      uVar17 = *(uint *)((int)pvVar10 + 0x18);
      puVar13 = (undefined4 *)FUN_00559950(pvVar10,&local_34);
      local_4._0_1_ = 7;
      FUN_004f26d0((void *)(iVar16 + 0x150),puVar13);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
    }
LAB_0051a087:
    pvVar10 = (void *)FUN_005f54d0((void *)(iVar16 + 0x130),uVar17);
    FUN_005f5600(pvVar10,0);
    FUN_004f4270((void *)(iVar16 + 0x140),pvVar10);
joined_r0x0051a0ac:
    if (iVar19 != 0) {
      iVar19 = iVar19 + -1;
      iVar18 = FUN_0053e290(iVar19);
      iVar9 = thunk_FUN_005f5060((int)local_1c);
      for (; iVar18 != 0; iVar18 = iVar18 + -1) {
        if (iVar9 == 0) goto joined_r0x0051a0ac;
        iVar9 = *(int *)(iVar9 + 0x10);
      }
      if (iVar9 != 0) {
        pvVar10 = (void *)FUN_005f54d0(local_1c,*(uint *)(iVar9 + 0x18));
        if (local_54 == 0) {
          if (local_50 == 0) {
            if (local_4c == 0) {
              if (local_48 == 0) {
                FUN_004f4270((void *)(iVar16 + 0x120),pvVar10);
                iVar15 = iVar15 + -1;
              }
              else {
                FUN_004f4270((void *)(iVar16 + 0xf0),pvVar10);
                local_48 = local_48 + -1;
              }
            }
            else {
              FUN_004f4270((void *)(iVar16 + 0xe0),pvVar10);
              local_4c = local_4c + -1;
            }
          }
          else {
            FUN_004f4270((void *)(iVar16 + 0x110),pvVar10);
            local_50 = local_50 - 1;
          }
        }
        else {
          FUN_004f4270((void *)(iVar16 + 0x100),pvVar10);
          local_54 = local_54 - 1;
        }
      }
      goto joined_r0x0051a0ac;
    }
    sVar2 = FUN_004f4290(iVar16 + 0x100);
    sVar3 = FUN_004f4290(iVar16 + 0x110);
    sVar4 = FUN_004f4290(iVar16 + 0xe0);
    sVar5 = FUN_004f4290(iVar16 + 0xf0);
    sVar6 = FUN_004f4290(iVar16 + 0x120);
    sVar7 = FUN_004f4290(iVar16 + 0x130);
    sVar8 = FUN_004f4290(iVar16 + 0x140);
    uVar17 = local_44;
    if (((((local_54 == 0) && (local_50 == 0)) && (local_4c == 0)) &&
        ((local_48 == 0 && (iVar15 == 0)))) &&
       (CONCAT22(extraout_var_01,sVar2) + CONCAT22(extraout_var_02,sVar3) +
        CONCAT22(extraout_var_03,sVar4) + CONCAT22(extraout_var_04,sVar5) +
        CONCAT22(extraout_var_05,sVar6) + CONCAT22(extraout_var_06,sVar7) +
        CONCAT22(extraout_var_07,sVar8) == local_44)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    iVar9 = FUN_005305c0(*(void **)(iVar16 + 0xc4),local_44);
    if ((iVar9 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar9 = FUN_005305c0(*(void **)(iVar16 + 200),uVar17);
    if ((iVar9 == 0) || (!bVar1)) {
      iVar9 = 0;
    }
    else {
      iVar9 = 1;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_004f3fb0(local_1c);
    local_4 = 0xffffffff;
    FUN_004f4380(local_24);
  }
  ExceptionList = local_c;
  return iVar9;
}

