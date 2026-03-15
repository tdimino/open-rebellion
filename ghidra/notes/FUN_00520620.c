
bool __thiscall FUN_00520620(void *this,undefined4 param_1)

{
  bool bVar1;
  undefined4 local_30 [6];
  undefined4 local_18;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643548;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00582a00(local_30);
  local_4 = 0;
  local_18 = param_1;
  bVar1 = FUN_005f63d0(this,local_30);
  local_4 = 0xffffffff;
  FUN_00582a70(local_30);
  ExceptionList = pvStack_c;
  return bVar1;
}

