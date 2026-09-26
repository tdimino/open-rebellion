
undefined4 FUN_00402cf0(int param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00629378;
  local_c = ExceptionList;
  uVar1 = 0;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00619730();
  if (param_1 != 0x32000242) {
    FUN_00619730();
    if (param_1 != 0x35000281) goto LAB_00402d59;
  }
  uVar1 = 1;
LAB_00402d59:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

