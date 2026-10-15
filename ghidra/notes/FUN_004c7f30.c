
void __fastcall FUN_004c7f30(int param_1)

{
  int *piVar1;
  
  piVar1 = *(int **)(param_1 + 4);
  if ((piVar1 != (int *)0x0) && ((*(byte *)(piVar1 + 0x1a) & 1) != 0)) {
    (**(code **)(*piVar1 + 0x10))();
  }
  return;
}

