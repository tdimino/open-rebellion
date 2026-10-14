
void __fastcall FUN_0060cbf0(int param_1)

{
  if ((*(int *)(param_1 + 0xe8) != 0) && (DAT_006be5c8 == 0)) {
    DAT_006be5c8 = 1;
    FUN_00600280(*(int *)(param_1 + 0xe8));
    if (*(undefined4 **)(param_1 + 0xe8) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)(param_1 + 0xe8))(1);
    }
    *(undefined4 *)(param_1 + 0xe8) = 0;
    DAT_006be5c8 = 0;
    SetFocus(*(HWND *)(param_1 + 0x18));
  }
  return;
}

