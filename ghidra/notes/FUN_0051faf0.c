
void __fastcall FUN_0051faf0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0064338e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065ed28;
  local_4 = 2;
  if ((undefined4 *)param_1[0x10] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x10])(1);
    param_1[0x10] = 0;
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(param_1 + 0xb);
  local_4 = 0xffffffff;
  FUN_004f5cc0(param_1);
  ExceptionList = pvStack_c;
  return;
}

