
void __fastcall FUN_004ad730(int param_1)

{
  int *piVar1;
  
  piVar1 = *(int **)(param_1 + 8);
  if ((piVar1 != (int *)0x0) && (piVar1[0x1b] == 0)) {
    (**(code **)(*piVar1 + 0xc))();
  }
  return;
}

