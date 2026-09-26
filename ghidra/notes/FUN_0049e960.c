
int * __fastcall FUN_0049e960(int param_1)

{
  void *pvVar1;
  int *piVar2;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006372d3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  piVar2 = (int *)0x0;
  local_4 = 0;
  if (((*(uint *)(param_1 + 0x3c) & 0xff000000) != 0) &&
     ((*(uint *)(param_1 + 0x40) & 0xff000000) != 0)) {
    piVar2 = (int *)FUN_004f5cd0(0x214);
    if (piVar2 != (int *)0x0) {
      pvVar1 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 1;
      if (pvVar1 == (void *)0x0) {
        pvVar1 = (void *)0x0;
      }
      else {
        pvVar1 = FUN_004f5b10(pvVar1,(undefined4 *)(param_1 + 0x40),0);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (pvVar1 != (void *)0x0) {
        FUN_004f57b0(local_14,pvVar1);
        piVar2[8] = *(int *)(param_1 + 0x2c);
        (**(code **)(*piVar2 + 0x24))(local_14);
        (**(code **)(*piVar2 + 0x2c))(param_1 + 0x3c);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return piVar2;
}

