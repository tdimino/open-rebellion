
void FUN_004106f0(int *param_1)

{
  undefined4 local_28;
  int local_24;
  uint local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a558;
  local_c = ExceptionList;
  if (param_1 != (int *)0x0) {
    ExceptionList = &local_c;
    FUN_0051c300(&local_28);
    local_4 = 0;
    FUN_0051c410(&local_28,param_1);
    if (local_24 != 0) {
      FUN_005970d0(local_10,local_14,1);
      FUN_00597170();
    }
    (**(code **)*param_1)(1);
    local_4 = 0xffffffff;
    FUN_0051c350(&local_28);
  }
  ExceptionList = local_c;
  return;
}

