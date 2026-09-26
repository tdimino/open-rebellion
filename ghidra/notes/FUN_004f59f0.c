
undefined4 FUN_004f59f0(void *param_1,void *param_2)

{
  void *this;
  undefined4 *puVar1;
  undefined4 uVar2;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063fa10;
  local_c = ExceptionList;
  uVar2 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_004ece30(&local_14);
  this = param_2;
  local_4._0_1_ = 1;
  if ((param_1 != (void *)0x0) && (param_2 != (void *)0x0)) {
    puVar1 = FUN_00403040(param_1,&param_1);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_10,puVar1);
    local_4._0_1_ = 1;
    FUN_00619730();
    puVar1 = FUN_00403040(this,&param_1);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_14,puVar1);
    local_4._0_1_ = 1;
    FUN_00619730();
    if (local_10 < local_14) {
      uVar2 = 1;
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar2;
}

