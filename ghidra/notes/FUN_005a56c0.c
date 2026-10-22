
void __fastcall FUN_005a56c0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006526b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_LAB_0066be80;
  local_4 = 0;
  FUN_005c0f60(0);
  local_4 = 0xffffffff;
  FUN_0059e990(param_1);
  ExceptionList = local_c;
  return;
}

