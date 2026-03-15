
void __fastcall FUN_00596ec0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00650f4e;
  pvStack_c = ExceptionList;
  local_4 = 2;
  ExceptionList = &pvStack_c;
  FUN_005f2ff0(param_1 + 0xe);
  local_4._0_1_ = 1;
  FUN_005bdcc0(param_1 + 9);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005bdd00(param_1 + 7);
  local_4 = 0xffffffff;
  FUN_005a6280(param_1);
  ExceptionList = pvStack_c;
  return;
}

