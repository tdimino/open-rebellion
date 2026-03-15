
int __thiscall FUN_00520720(void *this,undefined4 param_1)

{
  int iVar1;
  undefined4 local_30 [6];
  undefined4 local_18;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643588;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00582a00(local_30);
  local_4 = 0;
  local_18 = param_1;
  iVar1 = FUN_005f63f0(this,local_30);
  local_4 = 0xffffffff;
  FUN_00582a70(local_30);
  ExceptionList = pvStack_c;
  return iVar1;
}

