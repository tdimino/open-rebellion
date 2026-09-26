
void __fastcall FUN_00439490(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062d88e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658e10;
  local_4 = 5;
  param_1[0x51] = 0;
  param_1[0x61] = 0;
  if ((undefined *)param_1[0x5b] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x5b]);
    param_1[0x5b] = 0;
  }
  local_4._0_1_ = 4;
  FUN_00619730();
  local_4._0_1_ = 3;
  FUN_005f5ce0(param_1 + 0x55);
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_0049c6e0(param_1 + 0x4e);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0049c810((int)(param_1 + 0xf));
  local_4 = 0xffffffff;
  FUN_0049dd30(param_1 + 6);
  ExceptionList = pvStack_c;
  return;
}

