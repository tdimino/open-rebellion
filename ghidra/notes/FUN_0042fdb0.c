
void __thiscall FUN_0042fdb0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *this_00;
  void *pvStack_18;
  undefined4 local_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cb83;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_14);
  local_4 = 0;
  FUN_005f4d90(param_1,(int)this + 8);
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&pvStack_18);
  param_1 = (void *)0x0;
  if (pvStack_18 != (void *)0x0) {
    do {
      puStack_10 = (undefined4 *)FUN_00618b70(0xcc);
      local_4._0_1_ = 1;
      if (puStack_10 == (undefined4 *)0x0) {
        this_00 = (undefined4 *)0x0;
      }
      else {
        this_00 = FUN_00432d80(puStack_10);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (this_00 != (undefined4 *)0x0) {
        FUN_00432fd0(this_00,pvVar1,*(undefined4 *)((int)this + 0x10),
                     *(undefined4 *)((int)this + 0xc));
        FUN_0042ff30(this,this_00);
      }
      param_1 = (void *)((int)param_1 + 1);
    } while (param_1 < pvStack_18);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

