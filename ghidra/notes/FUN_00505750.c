
undefined4 __cdecl FUN_00505750(uint *param_1)

{
  undefined4 uVar1;
  undefined4 local_20;
  uint local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640fb0;
  local_c = ExceptionList;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_00520570(local_1c + 2);
  local_1c[0] = 1;
  local_1c[1] = 0x12;
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  FUN_005057f0(param_1,&local_20,local_1c,local_1c + 2);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  uVar1 = local_20;
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

