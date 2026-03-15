
undefined4 __fastcall FUN_005f6b60(int param_1)

{
  int iVar1;
  HWND hWnd;
  undefined4 uVar2;
  
  uVar2 = 0;
  *(undefined4 *)(param_1 + 0x10) = 0;
  if (*(int *)(param_1 + 0xc) == 2) {
    if (*(int *)(param_1 + 4) != 0) {
      DestroyWindow(*(HWND *)(*(int *)(param_1 + 4) + 0x18));
      if (*(undefined4 **)(param_1 + 4) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)(param_1 + 4))(1);
      }
      *(undefined4 *)(param_1 + 4) = 0;
      return 1;
    }
  }
  else if ((*(int *)(param_1 + 0xc) == 1) && (iVar1 = *(int *)(param_1 + 8), iVar1 != 0)) {
    hWnd = *(HWND *)(iVar1 + 0x18);
    FUN_005f70d0(iVar1);
    DestroyWindow(hWnd);
    if (*(undefined4 **)(param_1 + 8) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)(param_1 + 8))(1);
    }
    *(undefined4 *)(param_1 + 8) = 0;
    uVar2 = 1;
  }
  return uVar2;
}

