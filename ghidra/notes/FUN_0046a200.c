
void __fastcall FUN_0046a200(int param_1)

{
  int *piVar1;
  int iVar2;
  
  if ((*(int *)(param_1 + 0x118) != 1) && (*(int *)(param_1 + 0x118) == 2)) {
    piVar1 = (int *)FUN_00604500((void *)(param_1 + 0x6c),0x96);
    iVar2 = (**(code **)(**(int **)(param_1 + 0x170) + 0xc))();
    if (iVar2 == 0) {
      (**(code **)(*piVar1 + 0x10))();
    }
    else {
      (**(code **)(*piVar1 + 0xc))();
    }
    piVar1 = (int *)FUN_00604500((void *)(param_1 + 0x6c),0x9a);
    if (piVar1 != (int *)0x0) {
      iVar2 = (**(code **)(**(int **)(param_1 + 0x170) + 0x10))();
      if (iVar2 == 0) {
        (**(code **)(*piVar1 + 0x10))();
        return;
      }
      (**(code **)(*piVar1 + 0xc))();
    }
  }
  return;
}

