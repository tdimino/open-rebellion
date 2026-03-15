
undefined4 * __fastcall FUN_00403dd0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629653;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5270(param_1);
  local_4 = 0;
  *(undefined2 *)(param_1 + 5) = 0;
  *(undefined2 *)((int)param_1 + 0x16) = 0;
  FUN_00610b20(param_1 + 6,0,0,0,0,0,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00610b20(param_1 + 0xc,0,0,0,0,0,0);
  param_1[4] = 0;
  param_1[3] = 0;
  *param_1 = &PTR_FUN_006584e8;
  ExceptionList = local_c;
  return param_1;
}

