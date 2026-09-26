
uint FUN_0055b760(undefined4 param_1,undefined4 param_2,void *param_3)

{
  bool bVar1;
  uint uVar2;
  undefined3 extraout_var;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649c80;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x17;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar2 = FUN_0055b900();
  if (uVar2 != 0) {
    bVar1 = FUN_00520620(&DAT_006bb4e8,param_2);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      local_14 = 1;
      local_10 = 0xffffffff;
      local_4 = 1;
      FUN_00520580(param_3,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

