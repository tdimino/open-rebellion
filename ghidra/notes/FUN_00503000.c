
int FUN_00503000(undefined4 param_1,undefined4 param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00640a80;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x17;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar2 = FUN_00503930();
  if (iVar2 != 0) {
    bVar1 = FUN_00520620(&DAT_006b2b40,param_2);
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
  return iVar2;
}

