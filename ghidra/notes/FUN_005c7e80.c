
undefined4 __fastcall FUN_005c7e80(int *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  
  if ((param_1[0x20] != 0) && (param_1[0x21] == 0)) {
    param_1[0xf] = 0;
    (**(code **)(*param_1 + 4))();
    FUN_005aaf90();
    iVar1 = FUN_005ae1d0();
    if (iVar1 == 0) {
      for (piVar2 = (int *)(**(code **)(param_1[0x19] + 0x10))(); piVar2 != (int *)0x0;
          piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
        piVar3 = (int *)(**(code **)(*piVar2 + 0x10))();
        iVar1 = FUN_005b1b70(piVar3);
        if (iVar1 == 1) {
          return 1;
        }
      }
      for (piVar2 = (int *)(**(code **)(param_1[0x16] + 0x10))(); piVar2 != (int *)0x0;
          piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
        piVar3 = (int *)(**(code **)(*piVar2 + 0x10))();
        iVar1 = FUN_005b1b70(piVar3);
        if (iVar1 == 1) {
          return 1;
        }
      }
      iVar1 = (**(code **)(*param_1 + 4))();
      if (iVar1 == 1) {
        FUN_005aaf90();
        iVar1 = FUN_005a9330();
        if ((iVar1 != 0) &&
           ((iVar4 = FUN_005ba420(iVar1), iVar4 != 0 || (*(int *)(iVar1 + 0x74) != 0)))) {
          return 1;
        }
      }
    }
  }
  return 0;
}

