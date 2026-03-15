
int __cdecl FUN_00529a40(void *param_1,void *param_2)

{
  int iVar1;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00644838;
  local_c = ExceptionList;
  local_14[0] = 0xa0;
  local_14[1] = 0xb0;
  local_4 = 0;
  ExceptionList = &local_c;
  iVar1 = FUN_004f7e30(param_1,local_14,param_2);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar1;
}

