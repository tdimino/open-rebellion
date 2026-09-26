
void __fastcall FUN_005c2c80(int param_1)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  
  for (piVar1 = *(int **)(param_1 + 0x5fc); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[2]) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0x14))();
    (**(code **)(*piVar2 + 0x18))();
  }
  for (piVar1 = *(int **)(param_1 + 0x60c); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[2]) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0x14))();
    FUN_005d3cc0(piVar2);
  }
  piVar1 = *(int **)(param_1 + 0x5ec);
  while (piVar1 != (int *)0x0) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0x14))();
    iVar3 = FUN_005d41a0(piVar2);
    if (iVar3 == 0) {
      piVar1 = (int *)piVar1[2];
    }
    else {
      iVar3 = *piVar1;
      piVar1 = (int *)piVar1[2];
      iVar3 = (**(code **)(iVar3 + 0xc))();
      FUN_005f5970((void *)(param_1 + 0x5e4),iVar3);
    }
  }
  if (*(int *)(param_1 + 0x654) != 0) {
    FUN_005da830(*(int *)(param_1 + 0x654));
  }
  return;
}

