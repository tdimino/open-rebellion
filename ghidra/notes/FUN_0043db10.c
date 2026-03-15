
undefined4 * __fastcall FUN_0043db10(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e0e9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00539370(param_1);
  local_4 = 0;
  FUN_004ece30(param_1 + 0xb);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0xc);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0xd);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004ece30(param_1 + 0xe);
  *param_1 = &PTR_FUN_00659598;
  ExceptionList = local_c;
  return param_1;
}

