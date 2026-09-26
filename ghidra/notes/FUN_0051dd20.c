
bool FUN_0051dd20(int *param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006431c8;
  local_c = ExceptionList;
  if (param_1 != (int *)0x0) {
    local_14 = 1;
    local_10 = 0xffffffff;
    local_4 = 0;
    ExceptionList = &local_c;
    (**(code **)(*param_1 + 0x20))(&local_14);
    FUN_0051ebb0();
    if (param_1 != (int *)0x0) {
      (**(code **)*param_1)(1);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return param_1 != (int *)0x0;
}

