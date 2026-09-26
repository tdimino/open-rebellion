
undefined4 * __fastcall FUN_0058e5a0(undefined4 *param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650166;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00595280(param_1);
  local_4 = 0;
  FUN_00520570(param_1 + 2);
  local_4._0_1_ = 1;
  FUN_005f52c0(param_1 + 4);
  *param_1 = &PTR_LAB_0066a958;
  local_14 = 0;
  local_10 = 0;
  local_4._0_1_ = 3;
  FUN_00520580(param_1 + 2,&local_14);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

