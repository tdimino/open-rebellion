
int __thiscall FUN_0050ba90(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641bf8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00504c40(local_2c,this,1);
  local_4 = 0;
  bVar1 = FUN_005131b0((int)local_2c);
  iVar2 = FUN_0050a740(this,CONCAT31(extraout_var,bVar1),param_1);
  local_4 = 0xffffffff;
  FUN_00504d40(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

