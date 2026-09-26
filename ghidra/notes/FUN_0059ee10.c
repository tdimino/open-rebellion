
void __fastcall FUN_0059ee10(int param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00651c02;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined4 *)(param_1 + 4) = 0;
  local_4 = 3;
  if (DAT_006bc464 != (undefined4 *)0x0) {
    (**(code **)*DAT_006bc464)(1);
  }
  DAT_006bc464 = (undefined4 *)0x0;
  local_4._0_1_ = 2;
  FUN_00618d20(param_1 + 0x114,0xc,4,FUN_005c5b60);
  local_4._0_1_ = 1;
  FUN_005f5e30((undefined4 *)(param_1 + 0xfc));
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f5e30((undefined4 *)(param_1 + 0xec));
  local_4 = 0xffffffff;
  FUN_005f5e30((undefined4 *)(param_1 + 0xdc));
  ExceptionList = pvStack_c;
  return;
}

