
void __fastcall FUN_0059f380(int param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 *puVar3;
  void *pvVar4;
  longlong lVar5;
  
  piVar1 = (int *)FUN_005ba8b0();
  FUN_005bab50(piVar1);
  FUN_005bbe70();
  FUN_00619730();
  iVar2 = FUN_005aaf90();
  FUN_005aab10(iVar2);
  iVar2 = FUN_005aaf90();
  FUN_005aab70(iVar2);
  puVar3 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x30))();
  while (puVar3 != (undefined4 *)0x0) {
    FUN_0059f350(puVar3);
    puVar3 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x30))();
  }
  if (*(int *)(param_1 + 0x10c) != 0) {
    iVar2 = 0;
    pvVar4 = (void *)FUN_005aaf90();
    for (piVar1 = (int *)FUN_005a7460(pvVar4,iVar2); piVar1 != (int *)0x0;
        piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      *(undefined4 *)(iVar2 + 0x100) = 1;
    }
  }
  if (*(int *)(param_1 + 0x110) != 0) {
    iVar2 = 1;
    pvVar4 = (void *)FUN_005aaf90();
    for (piVar1 = (int *)FUN_005a7460(pvVar4,iVar2); piVar1 != (int *)0x0;
        piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      *(undefined4 *)(iVar2 + 0x100) = 1;
    }
  }
  iVar2 = FUN_005aaf90();
  FUN_005aabc0(iVar2);
  iVar2 = FUN_005aaf90();
  if (*(int *)(iVar2 + 0x9ec) == 1) {
    FUN_005aaf90();
    iVar2 = FUN_005a9330();
    if ((iVar2 != 0) && (iVar2 = FUN_005ba420(iVar2), iVar2 != 0)) {
      lVar5 = __ftol();
      FUN_005c0de0((int)lVar5);
    }
  }
  iVar2 = FUN_005aaf90();
  FUN_005c0f80((uint)(*(int *)(iVar2 + 0x8c8) != 0));
  piVar1 = (int *)FUN_005ba8b0();
  FUN_005bab50(piVar1);
  return;
}

