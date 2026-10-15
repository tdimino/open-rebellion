
undefined4 * __thiscall
FUN_004bd810(undefined4 *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639da8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0047aae0(param_2,param_3,param_4);
  local_4 = 0;
  FUN_004ece30();
  *param_1 = &PTR_FUN_0065c408;
  ExceptionList = local_c;
  return param_1;
}

