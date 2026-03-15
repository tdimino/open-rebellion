
undefined4 * __fastcall FUN_00440630(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e763;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0051f8a0(param_1);
  local_4 = 0;
  FUN_0042dbe0(param_1 + 10);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004ece30(param_1 + 0xb);
  *param_1 = &PTR_FUN_00659aa8;
  param_1[8] = 0;
  param_1[9] = 0;
  ExceptionList = local_c;
  return param_1;
}

