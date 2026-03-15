
uint __fastcall FUN_004abbf0(int param_1)

{
  int iVar1;
  HDC hDC;
  uint uVar2;
  uint uVar3;
  
  uVar2 = 0;
  uVar3 = 0;
  if (*(int **)(param_1 + 0x14) != (int *)0x0) {
    iVar1 = FUN_005fc0f0(*(int **)(param_1 + 0x14));
    uVar2 = iVar1 + 4;
  }
  if ((*(int *)(param_1 + 0x38) != 0) && (*(int *)(param_1 + 0x40) != 0)) {
    hDC = GetDC(*(HWND *)(*(int *)(param_1 + 0x40) + 0x18));
    FUN_00601b80(*(void **)(param_1 + 0x38),hDC);
    ReleaseDC(*(HWND *)(*(int *)(param_1 + 0x40) + 0x18),hDC);
    uVar3 = (*(int *)(*(int *)(param_1 + 0x38) + 0x20) - *(int *)(*(int *)(param_1 + 0x38) + 0x18))
            + 4;
  }
  if (uVar2 <= uVar3) {
    uVar2 = uVar3;
  }
  return uVar2;
}

