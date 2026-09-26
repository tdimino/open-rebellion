
int * __fastcall FUN_004d3db0(int param_1)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  uint *puVar5;
  int iVar6;
  int iVar7;
  undefined1 **ppuVar8;
  uint local_54;
  int *local_50;
  int local_4c;
  undefined1 *local_48;
  int local_44;
  void *pvStack_40;
  undefined4 local_3c [2];
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c37b;
  pvStack_c = ExceptionList;
  local_50 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_44 = param_1;
  FUN_004f4340(local_3c);
  iVar7 = *(int *)(param_1 + 0x2c);
  local_48 = &stack0xffffff98;
  local_4 = 0;
  local_4c = 0;
  FUN_004f26d0(&stack0xffffff98,(uint *)(param_1 + 0x50));
  pvVar2 = FUN_00403d30((void *)(iVar7 + 0x2c));
  if (pvVar2 != (void *)0x0) {
    iVar7 = *(int *)((int)pvVar2 + 0xe0);
    piVar3 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x50));
    if (piVar3 != (int *)0x0) {
      FUN_004ece30(&local_54);
      local_4._0_1_ = 1;
      FUN_005039d0(local_2c,piVar3,1);
      local_4 = CONCAT31(local_4._1_3_,2);
      while ((0 < iVar7 && (param_1 = local_44, local_4c == 0))) {
        iVar6 = 1000000;
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          piVar3 = (int *)FUN_0052bed0((int)local_2c);
          iVar4 = (**(code **)(*piVar3 + 500))();
          if (iVar4 < iVar6) {
            ppuVar8 = &local_48;
            pvVar2 = (void *)FUN_0052bed0((int)local_2c);
            puVar5 = FUN_004025b0(pvVar2,(uint *)ppuVar8);
            local_4._0_1_ = 3;
            FUN_004f26d0(&local_54,puVar5);
            local_4 = CONCAT31(local_4._1_3_,2);
            FUN_00619730();
            piVar3 = (int *)FUN_0052bed0((int)local_2c);
            iVar6 = (**(code **)(*piVar3 + 500))();
          }
          FUN_005130d0((int)local_2c);
        }
        iVar7 = iVar7 + -1;
        local_34 = 0x1c;
        local_30 = 0x20;
        if ((local_54 >> 0x18 < 0x1c) || (0x1f < local_54 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          pvStack_40 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 4;
          if (pvStack_40 == (void *)0x0) {
            pvVar2 = (void *)0x0;
          }
          else {
            pvVar2 = FUN_004f5b10(pvStack_40,&local_54,0);
          }
          local_4 = CONCAT31(local_4._1_3_,2);
          param_1 = local_44;
          if (pvVar2 != (void *)0x0) {
            FUN_004f57b0(local_3c,pvVar2);
            param_1 = local_44;
          }
        }
        else {
          local_4c = 1;
          param_1 = local_44;
        }
      }
      iVar7 = thunk_FUN_005f5060((int)local_3c);
      if ((iVar7 != 0) && (local_50 = (int *)FUN_004f5cd0(0x200), local_50 != (int *)0x0)) {
        local_50[8] = *(int *)(param_1 + 0x30);
        (**(code **)(*local_50 + 0x24))();
      }
      local_4._0_1_ = 1;
      FUN_00503ad0(local_2c);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_3c);
  ExceptionList = pvStack_c;
  return local_50;
}

