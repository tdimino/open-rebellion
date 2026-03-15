
int * __thiscall FUN_00441120(void *this,void *param_1)

{
  int *piVar1;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e958;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0049ecb0(local_2c);
  local_4 = 0;
  FUN_0049edc0(local_2c,param_1);
  piVar1 = FUN_00441190(this,param_1,(int)local_2c);
  local_4 = 0xffffffff;
  FUN_0049ed50(local_2c);
  ExceptionList = local_c;
  return piVar1;
}

