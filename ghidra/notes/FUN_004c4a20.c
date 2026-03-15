
void * __cdecl FUN_004c4a20(void *param_1,uint param_2)

{
  undefined1 local_24 [4];
  uint local_20;
  undefined4 local_1c;
  uint local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a6c8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00610b20(local_24,0,0,0,0,0,0);
  local_14 = local_14 & 0xfffffff9 | 1;
  local_20 = param_2 & 0xffff;
  local_4 = 0;
  local_1c = 9;
  FUN_00610c30((int)local_24);
  FUN_00610b70(param_1,(int)local_24);
  ExceptionList = local_c;
  return param_1;
}

