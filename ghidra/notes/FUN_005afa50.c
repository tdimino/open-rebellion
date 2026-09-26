
void __fastcall FUN_005afa50(int param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006530c3;
  pvStack_c = ExceptionList;
  local_4 = 7;
  ExceptionList = &pvStack_c;
  if (*(undefined4 **)(param_1 + 0x24c) != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)**(undefined4 **)(param_1 + 0x24c))(1);
  }
  *(undefined4 *)(param_1 + 0x24c) = 0;
  if (*(undefined4 **)(param_1 + 0x250) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(param_1 + 0x250))(1);
  }
  *(undefined4 *)(param_1 + 0x250) = 0;
  local_4._0_1_ = 6;
  FUN_00618d20(param_1 + 0x7c,0xc,0x26,FUN_005f2ff0);
  local_4._0_1_ = 5;
  FUN_005c5ff0((undefined4 *)(param_1 + 0x70));
  local_4._0_1_ = 4;
  FUN_005c5ff0((undefined4 *)(param_1 + 100));
  local_4._0_1_ = 3;
  FUN_005c5ff0((undefined4 *)(param_1 + 0x58));
  local_4._0_1_ = 2;
  FUN_005c5ff0((undefined4 *)(param_1 + 0x4c));
  local_4._0_1_ = 1;
  FUN_005bdf10((undefined4 *)(param_1 + 0x38));
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005bdaa0((undefined4 *)(param_1 + 0x2c));
  local_4 = 0xffffffff;
  FUN_005f56a0((undefined4 *)(param_1 + 4));
  ExceptionList = pvStack_c;
  return;
}

