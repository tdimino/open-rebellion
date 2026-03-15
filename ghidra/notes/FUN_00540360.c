
undefined4 * __fastcall FUN_00540360(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646a88;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(param_1);
  local_4 = 0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  *param_1 = &PTR_FUN_00661a98;
  FUN_005f5600(param_1,1);
  FUN_00540630(param_1,1);
  FUN_00540650(param_1,0);
  FUN_00540670(param_1,3);
  FUN_00540690(param_1,0);
  FUN_00540430(param_1,1);
  FUN_00540430(param_1,1);
  ExceptionList = local_c;
  return param_1;
}

