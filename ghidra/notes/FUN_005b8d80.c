
void __fastcall FUN_005b8d80(int param_1)

{
  int *piVar1;
  int *piVar2;
  
  for (piVar1 = (int *)(**(code **)(**(int **)(param_1 + 0x1c) + 0x10))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0x10))();
    (**(code **)(*piVar2 + 0x1c))();
  }
  for (piVar1 = (int *)(**(code **)(**(int **)(param_1 + 0x20) + 0x10))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0x10))();
    (**(code **)(*piVar2 + 0x1c))();
  }
  (**(code **)(**(int **)(param_1 + 0x1c) + 4))();
  (**(code **)(**(int **)(param_1 + 0x20) + 4))();
  return;
}

