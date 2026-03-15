
void __fastcall FUN_004c5f40(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_0063aa43;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065c610;
  param_1[0xd] = 0;
  puVar2 = param_1 + 0xe;
  for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  local_4 = 1;
  FUN_00619730();
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ecd30(param_1);
  ExceptionList = local_c;
  return;
}

