
void * __thiscall FUN_0041c150(void *this,int *param_1)

{
  void *this_00;
  int *piVar1;
  undefined4 *puVar2;
  int local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b6e8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  this_00 = *(void **)((int)this + 8);
  local_4 = 0;
  puVar2 = FUN_0041c2c0(this_00,local_10);
  local_4._0_1_ = 1;
  FUN_004f26d0(&local_14,puVar2);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  piVar1 = param_1;
  while ((this_00 != (void *)0x0 && (*piVar1 != local_14))) {
    this_00 = *(void **)((int)this_00 + 8);
    puVar2 = FUN_0041c2c0(this_00,&param_1);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_14,puVar2);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return this_00;
}

