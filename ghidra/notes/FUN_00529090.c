
bool __thiscall FUN_00529090(void *this,uint *param_1,int param_2,void *param_3,int *param_4)

{
  uint uVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00644708;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar1 = FUN_00528d30(this,param_1,param_2,0,param_3,param_4);
  ExceptionList = local_c;
  return uVar1 != 0;
}

