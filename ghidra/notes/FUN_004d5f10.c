
int * __fastcall FUN_004d5f10(int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 local_24;
  void *local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c6cb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_0042dbe0(&local_24);
  bVar1 = false;
  local_4 = 0;
  FUN_004f4340(local_1c);
  uVar2 = *(uint *)(param_1 + 0x50) >> 0x18;
  piVar4 = (int *)0x0;
  local_4._0_1_ = 1;
  local_14 = 0xa2;
  local_10 = 0xa4;
  if ((0xa1 < uVar2) && (uVar2 < 0xa4)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar4 = (int *)FUN_004f5cd0(0x211);
    if (piVar4 != (int *)0x0) {
      local_20 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (local_20 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004f5b10(local_20,(undefined4 *)(param_1 + 0x50),0);
      }
      local_4._0_1_ = 1;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_1c,pvVar3);
        piVar4[8] = *(int *)(param_1 + 0x30);
        (**(code **)(*piVar4 + 0x24))(local_1c);
        FUN_004f26d0(piVar4 + 0x11,(undefined4 *)(param_1 + 0x5c));
        piVar4[0x12] = *(int *)(param_1 + 0x60);
      }
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return piVar4;
}

