
void __fastcall FUN_0042db70(int param_1)

{
  if (*(int *)(param_1 + 0xa8) == 0) {
    if ((*(int *)(param_1 + 0xa0) != 0) && (*(int *)(param_1 + 0xac) == 0)) {
      PostMessageA(*(HWND *)(param_1 + 0x18),0x111,0x15e,0);
    }
  }
  else if (*(int *)(param_1 + 0xa4) == 0) {
    PostMessageA(*(HWND *)(param_1 + 0x18),0x482,0,*(int *)(param_1 + 0xa8));
    *(undefined4 *)(param_1 + 0xa0) = 0;
    return;
  }
  *(undefined4 *)(param_1 + 0xa0) = 0;
  return;
}

