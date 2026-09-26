
void __fastcall FUN_0059e990(undefined4 *param_1)

{
  void *this;
  uint uVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00651b12;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066bd68;
  uVar1 = param_1[0x14];
  local_4 = 5;
  if (uVar1 != 0) {
    this = (void *)FUN_005ba8b0();
    FUN_005bab10(this,uVar1);
    param_1[0x14] = 0;
  }
  local_4._0_1_ = 4;
  FUN_005c5b60(param_1 + 0x10);
  local_4._0_1_ = 3;
  FUN_005c5b60(param_1 + 0xd);
  local_4._0_1_ = 2;
  FUN_005c5ff0(param_1 + 10);
  local_4._0_1_ = 1;
  FUN_005c5ff0(param_1 + 7);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005c5ff0(param_1 + 4);
  local_4 = 0xffffffff;
  FUN_005c5ff0(param_1 + 1);
  ExceptionList = pvStack_c;
  return;
}

