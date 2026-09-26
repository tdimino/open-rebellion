
void __fastcall FUN_005a6590(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0065285c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066bed8;
  local_4 = 3;
  if ((undefined4 *)param_1[0x11] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x11])(1);
  }
  param_1[0x11] = 0;
  DAT_006bc470 = 0;
  local_4._0_1_ = 2;
  FUN_005f5300(param_1 + 0xe);
  local_4._0_1_ = 1;
  FUN_005f5300(param_1 + 0xb);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f5300(param_1 + 8);
  local_4 = 0xffffffff;
  FUN_005f5300(param_1 + 5);
  ExceptionList = local_c;
  return;
}

