
void __fastcall FUN_005ea590(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0065535b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066d8e0;
  param_1[3] = &PTR_LAB_0066d8c0;
  local_4 = 3;
  param_1[0x3e] = &PTR_FUN_0066d928;
  FUN_005f4f00(param_1 + 0x3e);
  local_4._0_1_ = 2;
  FUN_005f2af0(param_1 + 0x3b);
  local_4._0_1_ = 1;
  FUN_005f2af0(param_1 + 0x38);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005cd2c0((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 3)));
  local_4 = 0xffffffff;
  FUN_005c5ff0(param_1);
  ExceptionList = pvStack_c;
  return;
}

