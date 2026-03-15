
void * FUN_004ac4b0(void *param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063858b;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  local_4 = 1;
  FUN_004f26d0(param_1,&local_14);
  local_10 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

