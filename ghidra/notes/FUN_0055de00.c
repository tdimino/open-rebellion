
void __fastcall FUN_0055de00(int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649e38;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580((void *)(param_1 + 4),(undefined4 *)&stack0x00000004);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

