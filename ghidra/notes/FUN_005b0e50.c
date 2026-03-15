
undefined4 * __fastcall FUN_005b0e50(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653136;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005d2530(param_1 + 6);
  local_4 = 0;
  FUN_005d24e0(param_1 + 0xb);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005d2490(param_1 + 0x10);
  param_1[0x15] = 0;
  param_1[0x17] = 0;
  param_1[0x16] = 0;
  param_1[3] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  param_1[0x18] = 0;
  param_1[0x1a] = 0;
  param_1[0x1c] = 0;
  param_1[0x1b] = 0;
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  param_1[0x19] = 1;
  ExceptionList = local_c;
  return param_1;
}

