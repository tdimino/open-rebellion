
int * __fastcall FUN_004c7a90(int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 local_24 [2];
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ad8b;
  pvStack_c = ExceptionList;
  bVar1 = false;
  piVar4 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_24);
  local_14 = 0x90;
  local_10 = 0x98;
  uVar2 = *(uint *)(param_1 + 0x40) >> 0x18;
  local_4 = 1;
  if ((0x8f < uVar2) && (uVar2 < 0x98)) {
    local_1c = 0xa4;
    uVar2 = *(uint *)(param_1 + 0x44) >> 0x18;
    local_18 = 0xa6;
    if ((0xa3 < uVar2) && (uVar2 < 0xa6)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      bVar1 = true;
      goto LAB_004c7b27;
    }
  }
  bVar1 = false;
LAB_004c7b27:
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  if (bVar1) {
    piVar4 = (int *)FUN_004f5cd0(0x214);
    if (piVar4 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004f5b10(pvVar3,(undefined4 *)(param_1 + 0x44),0);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_24,pvVar3);
        piVar4[8] = *(int *)(param_1 + 0x2c);
        (**(code **)(*piVar4 + 0x24))(local_24);
        (**(code **)(*piVar4 + 0x2c))(param_1 + 0x40);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_24);
  ExceptionList = pvStack_c;
  return piVar4;
}

