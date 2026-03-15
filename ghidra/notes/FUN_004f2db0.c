
int * __cdecl FUN_004f2db0(int param_1)

{
  int *piVar1;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063f198;
  local_c = ExceptionList;
  local_14[0] = 0x14;
  local_14[1] = 0x1c;
  local_4 = 0;
  ExceptionList = &local_c;
  piVar1 = FUN_0053ef50(local_14,param_1,1);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return piVar1;
}

