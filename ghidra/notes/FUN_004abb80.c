
int __fastcall FUN_004abb80(int param_1)

{
  int iVar1;
  HDC hDC;
  
  iVar1 = 0;
  if ((*(int *)(param_1 + 0x38) != 0) && (*(int *)(param_1 + 0x40) != 0)) {
    hDC = GetDC(*(HWND *)(*(int *)(param_1 + 0x40) + 0x18));
    FUN_00601b80(*(void **)(param_1 + 0x38),hDC);
    ReleaseDC(*(HWND *)(*(int *)(param_1 + 0x40) + 0x18),hDC);
    iVar1 = *(int *)(*(int *)(param_1 + 0x38) + 0x1c) - *(int *)(*(int *)(param_1 + 0x38) + 0x14);
  }
  return *(int *)(param_1 + 0x74) + 0xc + iVar1;
}

