
void __fastcall FUN_005540c0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006491de;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00662158;
  param_1[0xc] = &PTR_FUN_00662154;
  local_4 = 2;
  if (((param_1[9] & 0x30) == 0) && ((undefined4 *)param_1[0x1a] != (undefined4 *)0x0)) {
    (*(code *)**(undefined4 **)param_1[0x1a])(1);
    param_1[0x1a] = 0;
  }
  local_4._0_1_ = 1;
  thunk_FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  thunk_FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f5f60(param_1);
  ExceptionList = pvStack_c;
  return;
}

