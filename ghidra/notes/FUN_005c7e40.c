
undefined4 __fastcall FUN_005c7e40(int param_1)

{
  int *piVar1;
  int iVar2;
  
  piVar1 = (int *)(**(code **)(*(int *)(param_1 + 0x58) + 0x10))();
  if (piVar1 == (int *)0x0) {
    return 0;
  }
  do {
    iVar2 = (**(code **)(*piVar1 + 0x10))();
    if (*(int *)(iVar2 + 0x150) != 0) {
      return 1;
    }
    piVar1 = (int *)(**(code **)(*piVar1 + 8))();
  } while (piVar1 != (int *)0x0);
  return 0;
}

