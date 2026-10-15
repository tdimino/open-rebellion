
void __fastcall FUN_00404b40(int param_1)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  
  iVar1 = *(int *)(param_1 + 0x10);
  if ((iVar1 != 0) && (iVar3 = 0, 0 < *(int *)(param_1 + 0x48))) {
    do {
      piVar2 = *(int **)(*(int *)(iVar1 + 0x20) + iVar3 * 4);
      if (piVar2 != (int *)0x0) {
        (**(code **)(*piVar2 + 8))(0);
      }
      iVar3 = iVar3 + 1;
    } while (iVar3 < *(int *)(param_1 + 0x48));
  }
  if (*(code **)(param_1 + 0x50) != (code *)0x0) {
    (**(code **)(param_1 + 0x50))(*(undefined4 *)(*(int *)(param_1 + 0x10) + 0x18));
  }
  return;
}

