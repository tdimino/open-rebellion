
int * __fastcall FUN_004d3a90(int param_1)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  uint *puVar6;
  undefined4 unaff_EBX;
  int iVar7;
  int iVar8;
  undefined1 **ppuVar9;
  uint local_58;
  int local_54;
  int local_50;
  int *local_4c;
  undefined1 *local_48;
  void *local_44;
  undefined4 local_40;
  undefined4 local_3c [2];
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c33b;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_4c = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_50 = param_1;
  FUN_004f4340(local_3c);
  puVar6 = (uint *)(param_1 + 0x54);
  iVar8 = 0;
  iVar7 = 0;
  local_4 = 0;
  local_54 = 0;
  local_44 = (void *)0x90;
  local_40 = 0x98;
  if ((0x8f < *puVar6 >> 0x18) && (*puVar6 >> 0x18 < 0x98)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar5 = *(int *)(param_1 + 0x2c);
    local_48 = &stack0xffffff90;
    FUN_004f26d0(&stack0xffffff90,puVar6);
    pvVar2 = FUN_00403d30((void *)(iVar5 + 0x2c));
    if (pvVar2 != (void *)0x0) {
      iVar8 = -*(int *)((int)pvVar2 + 0xe0);
    }
  }
  else {
    local_44 = (void *)0x8;
    local_40 = 0x10;
    if ((*puVar6 >> 0x18 < 8) || (0xf < *puVar6 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      iVar5 = *(int *)(param_1 + 0x2c);
      local_48 = &stack0xffffff90;
      FUN_004f26d0(&stack0xffffff90,puVar6);
      pvVar2 = FUN_00419e40((void *)(iVar5 + 0x78));
      if (pvVar2 != (void *)0x0) {
        iVar8 = *(int *)((int)pvVar2 + 0xac);
      }
    }
  }
  iVar5 = *(int *)(param_1 + 0x2c);
  local_48 = &stack0xffffff90;
  FUN_004f26d0(&stack0xffffff90,(uint *)(param_1 + 0x50));
  pvVar2 = FUN_00403d30((void *)(iVar5 + 0x2c));
  if (pvVar2 != (void *)0x0) {
    iVar7 = *(int *)((int)pvVar2 + 0x11c);
  }
  if (iVar7 <= iVar8) {
    iVar8 = iVar7;
  }
  piVar3 = FUN_004f3220(*(int *)(local_50 + 0x30),(uint *)(param_1 + 0x50));
  piVar4 = local_4c;
  if (piVar3 != (int *)0x0) {
    FUN_004ece30(&local_58);
    local_4._0_1_ = 1;
    FUN_005039d0(local_2c,piVar3,1);
    local_4 = CONCAT31(local_4._1_3_,2);
    while ((0 < iVar8 && (local_54 == 0))) {
      iVar7 = 1000000;
      FUN_00513120((int)local_2c);
      while (local_10 != 0) {
        piVar4 = (int *)FUN_0052bed0((int)local_2c);
        iVar5 = (**(code **)(*piVar4 + 500))();
        if (iVar5 < iVar7) {
          ppuVar9 = &local_48;
          pvVar2 = (void *)FUN_0052bed0((int)local_2c);
          puVar6 = FUN_004025b0(pvVar2,(uint *)ppuVar9);
          local_4._0_1_ = 3;
          FUN_004f26d0(&local_58,puVar6);
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00619730();
          piVar4 = (int *)FUN_0052bed0((int)local_2c);
          iVar7 = (**(code **)(*piVar4 + 500))();
        }
        FUN_005130d0((int)local_2c);
      }
      iVar8 = iVar8 + -1;
      local_34 = 0x1c;
      local_30 = 0x20;
      if ((local_58 >> 0x18 < 0x1c) || (0x1f < local_58 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        local_44 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 4;
        if (local_44 == (void *)0x0) {
          pvVar2 = (void *)0x0;
        }
        else {
          pvVar2 = FUN_004f5b10(local_44,&local_58,0);
        }
        local_4 = CONCAT31(local_4._1_3_,2);
        if (pvVar2 != (void *)0x0) {
          FUN_004f57b0(local_3c,pvVar2);
        }
      }
      else {
        local_54 = 1;
      }
    }
    iVar8 = thunk_FUN_005f5060((int)local_3c);
    piVar4 = local_4c;
    if ((iVar8 != 0) && (piVar4 = (int *)FUN_004f5cd0(0x201), piVar4 != (int *)0x0)) {
      piVar4[8] = *(int *)(local_50 + 0x30);
      (**(code **)(*piVar4 + 0x24))();
      (**(code **)(*piVar4 + 0x2c))(unaff_EBX);
    }
    local_4._0_1_ = 1;
    FUN_00503ad0(local_2c);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_3c);
  ExceptionList = pvStack_c;
  return piVar4;
}

