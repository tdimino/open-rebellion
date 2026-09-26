
undefined4 __fastcall FUN_005650f0(void *param_1)

{
  uint *puVar1;
  void *this;
  undefined1 local_2c [4];
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ac40;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_28,(int)param_1 + 0x20);
  local_4 = 0;
  puVar1 = FUN_005651a0(param_1,local_2c);
  local_4._0_1_ = 1;
  this = (void *)FUN_00505f60(puVar1);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  if (this != (void *)0x0) {
    FUN_0050d5a0(this,0xb,1,(int)local_28);
    FUN_0050d5a0(this,0xb,2,(int)local_28);
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return 0;
}

