
int __cdecl FUN_004f8340(void *param_1,void *param_2)

{
  int iVar1;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063fdd8;
  local_c = ExceptionList;
  local_14[0] = 1;
  local_14[1] = 0xff;
  local_4 = 0;
  ExceptionList = &local_c;
  iVar1 = FUN_004f7e30(param_1,local_14,param_2);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar1;
}

