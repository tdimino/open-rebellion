
bool FUN_00555f00(void *param_1,uint *param_2,int param_3,void *param_4,int *param_5)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649598;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_5 = 0;
  bVar1 = FUN_00529090(param_1,param_2,param_3,param_4,param_5);
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar1) != 0;
}

