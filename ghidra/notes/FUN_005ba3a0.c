
void __fastcall FUN_005ba3a0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00653553;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066c3c0;
  param_1[7] = &PTR_LAB_0066c3b8;
  local_4 = 2;
  FUN_005bdcc0(param_1 + 0x15);
  local_4._0_1_ = 1;
  FUN_005bdd00(param_1 + 0x13);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005b83b0((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 7)));
  local_4 = 0xffffffff;
  FUN_005a6280(param_1);
  ExceptionList = pvStack_c;
  return;
}

