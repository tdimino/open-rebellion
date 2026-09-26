
undefined4 * __fastcall FUN_005336b0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645819;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f2710(param_1);
  local_4 = 0;
  *(undefined2 *)(param_1 + 0x16) = 0;
  *(undefined2 *)((int)param_1 + 0x5a) = 0;
  *(undefined2 *)(param_1 + 0x17) = 0;
  *(undefined2 *)((int)param_1 + 0x5e) = 0;
  *(undefined2 *)(param_1 + 0x18) = 0;
  *(undefined2 *)((int)param_1 + 0x62) = 0;
  *(undefined2 *)(param_1 + 0x19) = 0;
  *(undefined2 *)((int)param_1 + 0x66) = 0;
  FUN_004ece30(param_1 + 0x1a);
  local_4._0_1_ = 1;
  FUN_0042dbe0(param_1 + 0x1b);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0x1c);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004ece30(param_1 + 0x1d);
  param_1[0x1e] = 0;
  *param_1 = &PTR_FUN_00660d60;
  param_1[0xc] = &PTR_FUN_00660d5c;
  ExceptionList = local_c;
  return param_1;
}

