
void __fastcall FUN_00415350(int param_1)

{
  int iVar1;
  BOOL BVar2;
  
  if (*(int **)(param_1 + 0xc0) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0xc0) + 8))();
    iVar1 = *(int *)(param_1 + 0xc0);
    if (*(int *)(param_1 + 0xb8) == iVar1) {
      *(undefined4 *)(param_1 + 0xb8) = 0;
    }
    if (*(int *)(param_1 + 0xbc) == iVar1) {
      *(undefined4 *)(param_1 + 0xbc) = 0;
    }
    PostMessageA(*(HWND *)(param_1 + 0x18),0x469,0,iVar1);
    *(undefined4 *)(param_1 + 0xc0) = 0;
  }
  iVar1 = *(int *)(param_1 + 0xb8);
  if (iVar1 != 0) {
    if (*(int *)(param_1 + 0xc0) == iVar1) {
      *(undefined4 *)(param_1 + 0xc0) = 0;
    }
    BVar2 = DestroyWindow(*(HWND *)(iVar1 + 0x18));
    if ((BVar2 != 0) && (*(undefined4 **)(param_1 + 0xb8) != (undefined4 *)0x0)) {
      (**(code **)**(undefined4 **)(param_1 + 0xb8))(1);
    }
    *(undefined4 *)(param_1 + 0xb8) = 0;
  }
  iVar1 = *(int *)(param_1 + 0xbc);
  if (iVar1 != 0) {
    if (*(int *)(param_1 + 0xc0) == iVar1) {
      *(undefined4 *)(param_1 + 0xc0) = 0;
    }
    DestroyWindow(*(HWND *)(iVar1 + 0x18));
    if (*(undefined4 **)(param_1 + 0xbc) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)(param_1 + 0xbc))(1);
    }
    *(undefined4 *)(param_1 + 0xbc) = 0;
  }
  return;
}

