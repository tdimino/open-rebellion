
void __thiscall FUN_00458ea0(void *this,void *param_1)

{
  int *this_00;
  uint *puVar1;
  undefined4 local_18;
  uint local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630630;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  this_00 = FUN_004a25c0(this);
  if (this_00 != (int *)0x0) {
    puVar1 = FUN_004025b0(this_00,&local_14);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_18,puVar1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  local_10 = *(undefined4 *)((int)this + 0x24);
  FUN_004ecea0(&local_18,param_1);
  FUN_005f4db0(param_1,&local_10);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

