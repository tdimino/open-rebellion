
void __fastcall FUN_00613e00(int param_1)

{
  int *piVar1;
  DWORD DVar2;
  int iVar3;
  undefined4 *puVar4;
  
  if (*(int *)(param_1 + 0x34) != 0) {
    if (*(int *)(param_1 + 0x3c) != 0) {
      if (*(HANDLE *)(param_1 + 0x48) != (HANDLE)0x0) {
        SetEvent(*(HANDLE *)(param_1 + 0x48));
      }
      DVar2 = WaitForSingleObject(*(HANDLE *)(param_1 + 0x3c),0x5dc);
      if ((DVar2 == 0x102) && (*(int *)(param_1 + 0x70) != 0)) {
        WaitForSingleObject(*(HANDLE *)(param_1 + 0x3c),0x1194);
      }
      CloseHandle(*(HANDLE *)(param_1 + 0x3c));
      *(undefined4 *)(param_1 + 0x3c) = 0;
    }
    puVar4 = (undefined4 *)(param_1 + 0x40);
    iVar3 = 3;
    do {
      if ((HANDLE)*puVar4 != (HANDLE)0x0) {
        CloseHandle((HANDLE)*puVar4);
        *puVar4 = 0;
      }
      puVar4 = puVar4 + 1;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
    piVar1 = *(int **)(param_1 + 0x38);
    if (piVar1 != (int *)0x0) {
      iVar3 = (**(code **)(*piVar1 + 8))(piVar1);
      if (iVar3 == 0) {
        *(undefined4 *)(param_1 + 0x38) = 0;
      }
    }
  }
  return;
}

