
undefined4 __thiscall
FUN_00528720(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  undefined4 local_1c;
  undefined4 local_18;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644640;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00541e70(&local_1c);
  local_4 = 0;
  local_1c = param_2;
  puVar1 = (undefined4 *)FUN_004ece30(&param_2);
  local_4._0_1_ = 1;
  FUN_004f26d0(&local_14,puVar1);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_10 = 0x215;
  local_18 = 0;
  uVar2 = (**(code **)(*(int *)this + 0x1c4))(&local_1c,param_3);
  pvStack_c = (void *)0xffffffff;
  FUN_00541ea0();
  ExceptionList = local_14;
  return uVar2;
}

