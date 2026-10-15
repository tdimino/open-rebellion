
int __thiscall FUN_00508fb0(void *this,undefined4 param_1,undefined4 param_2)

{
  void *pvVar1;
  int iVar2;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641898;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar1 = FUN_00503a50(local_2c,this,param_2,param_1);
  local_4 = 0;
  iVar2 = FUN_00513180((int)pvVar1);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

