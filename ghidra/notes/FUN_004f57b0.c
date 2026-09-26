
uint __thiscall FUN_004f57b0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  uint uVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f9a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  pvVar1 = param_1;
  local_4 = 0;
  if (param_1 != (void *)0x0) {
    puVar2 = FUN_00403040(param_1,&param_1);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_10,puVar2);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if ((((uint)local_10 & 0xffffff) != 0) && (((uint)local_10 & 0xff000000) != 0)) {
      FUN_005f4f10(this,(int)pvVar1);
    }
  }
  uVar3 = (uint)local_10 & 0xffffff;
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar3;
}

