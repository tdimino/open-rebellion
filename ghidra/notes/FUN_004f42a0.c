
undefined4 * __fastcall FUN_004f42a0(void *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063f697;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x28);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4._0_1_ = 1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00540a50(puVar1);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_00540c20(puVar1,(undefined4 *)&stack0x00000004);
    FUN_004f4270(param_1,puVar1);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return puVar1;
}

