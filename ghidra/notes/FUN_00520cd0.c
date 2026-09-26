
int __fastcall FUN_00520cd0(undefined4 param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined4 local_3c [11];
  int *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006436e8;
  pvStack_c = ExceptionList;
  iVar2 = 0;
  iVar4 = 0;
  iVar3 = 0;
  ExceptionList = &pvStack_c;
  FUN_00525bb0(local_3c,param_1);
  local_4 = 0;
  FUN_00525930((int)local_3c);
  while (local_10 != (int *)0x0) {
    iVar3 = iVar3 + 1;
    iVar1 = (**(code **)(*local_10 + 500))();
    iVar4 = iVar4 + iVar1;
    FUN_005258f0((int)local_3c);
  }
  if (iVar3 != 0) {
    iVar2 = iVar4 / iVar3;
  }
  local_4 = 0xffffffff;
  FUN_00525c50(local_3c);
  ExceptionList = pvStack_c;
  return iVar2;
}

