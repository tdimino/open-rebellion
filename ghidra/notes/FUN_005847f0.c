
void __fastcall FUN_005847f0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0064f353;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066a190;
  param_1[7] = &PTR_LAB_0066a1a8;
  local_4 = 2;
  FUN_005f2ff0(param_1 + 0xd);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f6370((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 7)));
  local_4 = 0xffffffff;
  FUN_005f55f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

