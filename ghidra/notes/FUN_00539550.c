
undefined4 * __fastcall FUN_00539550(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006460c3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00583fe0();
  local_4 = 0;
  FUN_005f2f50();
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005f2f50();
  *param_1 = &PTR_FUN_00661360;
  ExceptionList = local_c;
  return param_1;
}

