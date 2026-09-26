
undefined4 * __fastcall FUN_00584750(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f30e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(param_1);
  local_4 = 0;
  FUN_005f6360(param_1 + 7);
  local_4._0_1_ = 1;
  FUN_00520570(param_1 + 9);
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined2 *)(param_1 + 0xc) = 0;
  *(undefined2 *)((int)param_1 + 0x32) = 0;
  FUN_005f35b0(param_1 + 0xd,s_<not_set>_006ab3c0);
  param_1[7] = &PTR_LAB_0066a1a8;
  *param_1 = &PTR_FUN_0066a190;
  param_1[0xb] = 0;
  ExceptionList = local_c;
  return param_1;
}

