
undefined4
FUN_004fe160(uint *param_1,undefined4 param_2,undefined4 *param_3,undefined4 *param_4,
            undefined4 *param_5,void *param_6)

{
  bool bVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006404a8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x17;
  ExceptionList = &local_c;
  *param_3 = 1;
  *param_4 = 0;
  *param_5 = 0;
  local_4 = 0;
  FUN_00520580(param_6,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x14;
  local_10 = 0x1c;
  if ((*param_1 < 0x14) || (0x1c < param_1[1])) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *param_4 = 1;
  }
  else {
    *param_3 = 0;
  }
  ExceptionList = local_c;
  return 1;
}

