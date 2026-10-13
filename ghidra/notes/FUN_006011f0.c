
undefined4 __fastcall FUN_006011f0(int param_1)

{
  int iVar1;
  undefined4 uVar2;

  uVar2 = 0;
  iVar1 = FUN_006012f0(*(undefined4 *)(param_1 + 0x18));
  if (iVar1 != 0) {
    if ((*(int *)(iVar1 + 0xc) != 0) && (*(int *)(*(int *)(iVar1 + 0xc) + 0x1c) != 0)) {
      FUN_0060f0b0(*(undefined4 *)(param_1 + 0x18));
    }
    FUN_005f6790(iVar1);
    if (DAT_006be4e4 == 0) {
      UnhookWindowsHookEx(DAT_006be5b0);
      DAT_006be5b0 = (HHOOK)0x0;
    }
    uVar2 = 1;
  }
  return uVar2;
}
