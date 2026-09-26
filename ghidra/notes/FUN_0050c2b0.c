
int __thiscall FUN_0050c2b0(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  bool bVar3;
  void *pvVar4;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641cd8;
  pvStack_c = ExceptionList;
  iVar2 = 1;
  ExceptionList = &pvStack_c;
  FUN_0055b260(local_2c,this,1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while ((local_10 != 0 && (bVar3 = iVar2 != 0, iVar2 = 0, bVar3))) {
    pvVar4 = param_1;
    pvVar1 = (void *)FUN_0052bed0((int)local_2c);
    iVar2 = FUN_0050c560(this,pvVar1,pvVar4);
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_0055b2e0(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

