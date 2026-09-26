
bool __fastcall FUN_005862a0(int *param_1)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  bool bVar4;
  int local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064f698;
  pvStack_c = ExceptionList;
  bVar4 = false;
  local_14 = 1;
  local_10 = (void *)0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*param_1 + 0x1c))(&local_14);
  if ((iVar1 != 0) && (local_14 == -1)) {
    piVar2 = (int *)(**(code **)(*param_1 + 0x2c))();
    bVar4 = *piVar2 == param_1[0x11];
    if (bVar4) {
      (**(code **)(*param_1 + 0x30))();
      iVar1 = param_1[0x10];
      iVar3 = FUN_00586130((int)param_1);
      param_1[0x10] = iVar3 + iVar1;
    }
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_10;
  return bVar4;
}

