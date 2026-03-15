
void __fastcall FUN_00562ba0(undefined4 *param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064a478;
  local_c = ExceptionList;
  local_14 = 0x90;
  local_10 = 0x98;
  ExceptionList = &local_c;
  *param_1 = 0;
  local_4 = 0;
  FUN_00520580(param_1 + 1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1[3] = 1;
  param_1[4] = DAT_00661a88;
  param_1[5] = 1;
  param_1[6] = DAT_00661a84;
  ExceptionList = local_c;
  return;
}

