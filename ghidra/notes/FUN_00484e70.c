
undefined4 * __thiscall FUN_00484e70(undefined4 *param_1,undefined4 param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006346f8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004884f0(param_2);
  local_4 = 0;
  FUN_004be580();
  *param_1 = &PTR_FUN_0065aa70;
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  ExceptionList = local_c;
  return param_1;
}

