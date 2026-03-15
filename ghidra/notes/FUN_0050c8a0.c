
int __cdecl FUN_0050c8a0(void *param_1,void *param_2)

{
  int iVar1;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00641d98;
  local_c = ExceptionList;
  local_14[0] = 0x90;
  local_14[1] = 0x98;
  local_4 = 0;
  ExceptionList = &local_c;
  iVar1 = FUN_004f7e30(param_1,local_14,param_2);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar1;
}

