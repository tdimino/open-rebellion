
void __fastcall FUN_005b8d10(int param_1)

{
  int *piVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  
  piVar2 = (int *)(**(code **)(**(int **)(param_1 + 0x1c) + 0x10))();
  while (piVar1 = piVar2, piVar1 != (int *)0x0) {
    piVar2 = (int *)(**(code **)(*piVar1 + 8))();
    uVar3 = (**(code **)(*piVar1 + 4))();
    iVar4 = FUN_005b8ce0(uVar3);
    if (iVar4 == 0) {
      iVar4 = (**(code **)(*piVar1 + 0x10))();
      FUN_005c5f20(*(void **)(param_1 + 0x20),iVar4);
      iVar4 = (**(code **)(*piVar1 + 0x10))();
      *(undefined4 *)(iVar4 + 0x170) = 5;
      uVar3 = (**(code **)(*piVar1 + 4))();
      FUN_005c60d0(*(void **)(param_1 + 0x1c),uVar3);
    }
  }
  return;
}

