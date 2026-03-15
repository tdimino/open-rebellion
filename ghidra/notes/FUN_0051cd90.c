
void __fastcall FUN_0051cd90(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006430e4;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065ecf4;
  local_4 = 2;
  FUN_0051e5b0((int)param_1);
  local_4._0_1_ = 1;
  FUN_00539830((int)(param_1 + 0x22));
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00539830((int)(param_1 + 0x1b));
  local_4 = 0xffffffff;
  FUN_00539830((int)(param_1 + 0x14));
  ExceptionList = local_c;
  return;
}

