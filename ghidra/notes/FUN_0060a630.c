
bool __fastcall FUN_0060a630(int param_1)

{
  int *piVar1;
  
  piVar1 = *(int **)(param_1 + 4);
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 4))(5);
  }
  return piVar1 != (int *)0x0;
}

