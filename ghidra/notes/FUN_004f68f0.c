
undefined4
FUN_004f68f0(undefined4 param_1,undefined4 param_2,undefined4 *param_3,undefined4 *param_4,
            undefined4 *param_5,void *param_6)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063fc78;
  local_c = ExceptionList;
  local_14 = 1;
  ExceptionList = &local_c;
  *param_3 = 0;
  *param_4 = 0;
  *param_5 = 0;
  local_10 = 0x27;
  local_4 = 0;
  FUN_00520580(param_6,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

