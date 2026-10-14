
undefined4 * __fastcall FUN_00562ae0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a43b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00520570(param_1 + 1);
  local_4 = 0;
  FUN_00562ba0(param_1);
  ExceptionList = local_c;
  return param_1;
}

