
void __thiscall FUN_004acba0(void *this,undefined4 *param_1,uint param_2)

{
  undefined4 *this_00;
  uint uVar1;
  undefined4 *puVar2;
  uint *puVar3;
  void *this_01;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638668;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  this_00 = param_1;
  local_4 = 0;
  if (param_1 != (undefined4 *)0x0) {
    puVar2 = FUN_00403040(param_1,&param_1);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_10,puVar2);
    local_4._0_1_ = 0;
    FUN_00619730();
    if ((((uint)local_10 & 0xffffff) != 0) && (((uint)local_10 & 0xff000000) != 0)) {
      puVar3 = FUN_00403040(this_00,&param_1);
      local_4._0_1_ = 2;
      this_01 = FUN_004f5940(this,puVar3);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      uVar1 = param_2;
      if ((this_01 == (void *)0x0) || (param_2 == 0)) {
        FUN_005f4f10(this,(int)this_00);
        if (uVar1 != 0) {
          FUN_004ace40(this_00,uVar1);
        }
      }
      else {
        FUN_004ace40(this_01,param_2);
        (**(code **)*this_00)(1);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

