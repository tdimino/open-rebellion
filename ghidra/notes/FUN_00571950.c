
void __cdecl FUN_00571950(uint param_1,void *param_2,undefined4 *param_3)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064c2e8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = 0;
  FUN_005868c0(param_1,param_2,param_3);
  ExceptionList = local_c;
  return;
}

