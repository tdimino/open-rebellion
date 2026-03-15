
void __fastcall FUN_004313c0(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062ce5e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00658d30;
  param_1[0x11] = 0;
  puVar2 = param_1 + 0x12;
  for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  local_4 = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ecd30(param_1);
  ExceptionList = local_c;
  return;
}

