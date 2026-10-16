
void __fastcall FUN_005f5300(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00655778;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066dce0;
  local_4 = 0;
  FUN_005f5540(param_1);
  local_4 = 0xffffffff;
  FUN_005f4f00(param_1);
  ExceptionList = pvStack_c;
  return;
}

