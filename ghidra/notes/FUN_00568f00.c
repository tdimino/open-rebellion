
undefined4 FUN_00568f00(void *param_1,void *param_2)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  uint local_48 [5];
  undefined4 local_34 [5];
  undefined1 local_20 [20];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b400;
  local_c = ExceptionList;
  uVar2 = 0;
  ExceptionList = &local_c;
  FUN_00568fe0(local_34);
  local_4 = 0;
  FUN_00568fe0(local_48);
  local_4._0_1_ = 1;
  if ((param_1 != (void *)0x0) && (param_2 != (void *)0x0)) {
    puVar1 = FUN_00568ec0(param_1,local_20);
    local_4._0_1_ = 2;
    FUN_00569030(local_34,puVar1);
    local_4._0_1_ = 1;
    FUN_00619730();
    puVar1 = FUN_00568ec0(param_2,local_20);
    local_4._0_1_ = 3;
    FUN_00569030(local_48,puVar1);
    local_4._0_1_ = 1;
    FUN_00619730();
    uVar2 = FUN_005690b0(local_34,local_48);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar2;
}

