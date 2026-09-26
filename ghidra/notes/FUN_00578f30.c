
undefined4 __fastcall FUN_00578f30(int *param_1)

{
  int iVar1;
  uint *puVar2;
  uint *puVar3;
  void *local_4c;
  void *local_48;
  uint auStack_44 [14];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064d1c8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004fd450(auStack_44 + 7,(int)(param_1 + 8));
  local_48 = (void *)0x0;
  auStack_44[3] = 1;
  auStack_44[4] = -1;
  auStack_44[1] = 1;
  auStack_44[2] = 0x12;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  iVar1 = FUN_00504e60((uint *)(param_1 + 0xf),&local_48,auStack_44 + 1,auStack_44 + 3);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4c = (void *)0x0;
  auStack_44[5] = 1;
  auStack_44[6] = 0xffffffff;
  local_4._0_1_ = 3;
  if (iVar1 != 0) {
    if (auStack_44[4] == -1) {
      auStack_44[1] = 1;
      auStack_44[2] = 0x22;
      local_4._0_1_ = 4;
      iVar1 = FUN_00504e60((uint *)(param_1 + 0x12),&local_4c,auStack_44 + 1,auStack_44 + 5);
      local_4._0_1_ = 3;
      FUN_00619730();
      if (iVar1 == 0) goto LAB_005790ab;
      if (local_4c == (void *)0x0) {
        FUN_00520580(auStack_44 + 3,auStack_44 + 5);
      }
    }
    if ((iVar1 != 0) && (auStack_44[4] == -1)) {
      iVar1 = (**(code **)(*param_1 + 0x1c))(auStack_44 + 3);
      if ((iVar1 != 0) && (auStack_44[4] == -1)) {
        puVar2 = FUN_004025b0(local_4c,auStack_44 + 1);
        local_4._0_1_ = 5;
        puVar3 = FUN_004025b0(local_48,auStack_44);
        local_4._0_1_ = 6;
        FUN_00515440(puVar3,puVar2,auStack_44 + 7);
        local_4._0_1_ = 5;
        FUN_00619730();
        local_4._0_1_ = 3;
        FUN_00619730();
      }
    }
  }
LAB_005790ab:
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004fd4d0(auStack_44 + 7);
  ExceptionList = pvStack_c;
  return 0;
}

