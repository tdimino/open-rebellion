
int __thiscall FUN_00509020(void *this,undefined4 param_1,undefined4 param_2)

{
  void *pvVar1;
  int iVar2;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006418b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar1 = FUN_00504cc0(local_2c,this,param_2,param_1);
  local_4 = 0;
  iVar2 = FUN_00513180((int)pvVar1);
  local_4 = 0xffffffff;
  FUN_00504d40(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

