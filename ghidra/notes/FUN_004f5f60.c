
void __fastcall FUN_004f5f60(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063fac5;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065d260;
  param_1[0xc] = &PTR_FUN_0065d258;
  local_4 = 3;
  if ((undefined4 *)param_1[0xd] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0xd])(1);
    param_1[0xd] = 0;
  }
  if ((param_1[9] & 0x30) == 0) {
    FUN_004fc5e0((int)param_1);
  }
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f6370((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0xc)));
  local_4 = 0xffffffff;
  FUN_00539ef0(param_1);
  ExceptionList = pvStack_c;
  return;
}

