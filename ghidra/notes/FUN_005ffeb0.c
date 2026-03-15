
void __fastcall FUN_005ffeb0(int param_1)

{
  int iVar1;
  int X;
  
  iVar1 = *(int *)(param_1 + 0x20);
  if (iVar1 == 0) {
    iVar1 = GetSystemMetrics(0);
    X = iVar1 / 2 - *(int *)(param_1 + 0x30) / 2;
    iVar1 = GetSystemMetrics(1);
    iVar1 = iVar1 / 2 - *(int *)(param_1 + 0x34) / 2;
  }
  else {
    X = *(int *)(iVar1 + 0x38) / 2 - *(int *)(param_1 + 0x30) / 2;
    iVar1 = *(int *)(iVar1 + 0x3c) / 2 - *(int *)(param_1 + 0x34) / 2;
  }
  SetWindowPos(*(HWND *)(param_1 + 0x18),(HWND)0x0,X,iVar1,0,0,5);
  return;
}

