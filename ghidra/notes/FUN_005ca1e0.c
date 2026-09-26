
void __fastcall FUN_005ca1e0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00654153;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066cd28;
  param_1[7] = &PTR_LAB_0066cd10;
  local_4 = 9;
  if ((undefined4 *)param_1[0x18] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x18])(1);
  }
  if ((undefined4 *)param_1[0x15] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x15])(1);
  }
  if ((undefined4 *)param_1[0x10] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x10])(1);
  }
  local_4._0_1_ = 8;
  FUN_005d2180(param_1 + 0x29);
  local_4._0_1_ = 7;
  FUN_005d2180(param_1 + 0x26);
  local_4._0_1_ = 6;
  FUN_005d2180(param_1 + 0x23);
  local_4._0_1_ = 5;
  FUN_005c5b60(param_1 + 0x20);
  local_4._0_1_ = 4;
  FUN_005c5b60(param_1 + 0x1d);
  local_4._0_1_ = 3;
  FUN_005c5ff0(param_1 + 0x11);
  local_4._0_1_ = 2;
  FUN_005bdcc0(param_1 + 0xc);
  local_4._0_1_ = 1;
  FUN_005bdd00(param_1 + 10);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005c5ff0((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 7)));
  local_4 = 0xffffffff;
  FUN_005a6280(param_1);
  ExceptionList = pvStack_c;
  return;
}

