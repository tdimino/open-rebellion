
int __cdecl FUN_0055b480(void *param_1,undefined4 param_2,undefined4 param_3)

{
  int iVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649bf8;
  local_c = ExceptionList;
  local_14 = 0xa4;
  local_10 = 0xa6;
  local_4 = 0;
  ExceptionList = &local_c;
  iVar1 = FUN_004f6c00(param_1,&local_14,param_2,param_3);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar1;
}

