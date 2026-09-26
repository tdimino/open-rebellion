
int __thiscall FUN_00552ff0(void *this,int *param_1,void *param_2,int *param_3)

{
  uint *puVar1;
  int iVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648f58;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = 0;
  puVar1 = FUN_0050c640(param_1);
  local_4 = 1;
  iVar2 = FUN_00553130(this,puVar1,param_3);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((iVar2 != 0) && (*param_3 == 0)) {
    local_14 = 1;
    local_10 = 0x23;
    local_4 = 2;
    FUN_00520580(param_2,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar2;
}

