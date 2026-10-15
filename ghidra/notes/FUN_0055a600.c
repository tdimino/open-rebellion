
undefined4 * __fastcall FUN_0055a600(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649a08;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0053a670(param_1);
  local_4 = 0;
  FUN_00520570(param_1 + 0x19);
  param_1[0x1b] = DAT_00661a88;
  *param_1 = &PTR_FUN_00662778;
  param_1[0xc] = &PTR_FUN_00662774;
  ExceptionList = local_c;
  return param_1;
}

