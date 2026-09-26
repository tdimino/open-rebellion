
int FUN_00576bc0(undefined4 param_1,undefined4 param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064cb90;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x17;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  bVar1 = FUN_00576fd0();
  if (CONCAT31(extraout_var,bVar1) != 0) {
    bVar2 = FUN_00520620(&DAT_006bbb28,param_2);
    if (CONCAT31(extraout_var_00,bVar2) != 0) {
      local_14 = 1;
      local_10 = 0xffffffff;
      local_4 = 1;
      FUN_00520580(param_3,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar1);
}

