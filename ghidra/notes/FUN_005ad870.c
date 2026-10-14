
undefined4 __thiscall FUN_005ad870(int param_1,int param_2)

{
  int *piVar1;
  int iVar2;
  
  piVar1 = (int *)FUN_005ca4b0((void *)(param_1 + 0x988),*(uint *)(param_2 + 0xec));
  if (piVar1 == (int *)0x0) {
    return 0;
  }
  iVar2 = (**(code **)(*piVar1 + 0x10))();
  if (*(int *)(iVar2 + 0x170) != 2) {
    iVar2 = (**(code **)(*piVar1 + 0x10))();
    if (*(int *)(iVar2 + 0x170) != 6) {
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      if (*(int *)(iVar2 + 0x170) != 4) {
        return 0;
      }
    }
  }
  return 1;
}

