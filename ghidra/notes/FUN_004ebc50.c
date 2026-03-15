
void __fastcall FUN_004ebc50(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_0063ea13;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065ca10;
  local_4 = 1;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  FUN_00619730();
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ecd30(param_1);
  ExceptionList = local_c;
  return;
}

