
uint __thiscall FUN_005201c0(void *this,void *param_1)

{
  int *this_00;
  uint uVar1;
  undefined4 local_1c;
  undefined4 local_18;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00643498;
  pvStack_c = ExceptionList;
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_00 = (int *)(**(code **)(*(int *)this + 0x48))();
  uVar1 = 0;
  if (this_00 != (int *)0x0) {
    FUN_00541e70(&local_1c);
    local_4 = 1;
    FUN_0051fcb0(this,&local_1c);
    FUN_0054f3e0(this_00,&local_1c);
    uVar1 = FUN_005535b0(this_00,(int)this + 0x2c,*(int **)((int)this + 0x40),param_1);
    local_4 = 0xffffffff;
    FUN_00541ea0();
    (**(code **)*this_00)(1);
  }
  if (uVar1 == 0) {
    local_1c = 1;
    local_18 = 1;
    local_4 = 2;
    FUN_00520580(param_1,&local_1c);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return uVar1;
}

