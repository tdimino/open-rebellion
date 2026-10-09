
void FUN_0041da80(void)

{
  if (DAT_006b14ac != (HHOOK)0x0) {
    UnhookWindowsHookEx(DAT_006b14ac);
    DAT_006b14ac = (HHOOK)0x0;
  }
  if (DAT_006b14b0 != (HHOOK)0x0) {
    UnhookWindowsHookEx(DAT_006b14b0);
    DAT_006b14b0 = (HHOOK)0x0;
  }
  DAT_006b14bc = 0;
  if (DAT_006b14b8 != 0) {
    DAT_006b14b8 = 0;
    FUN_00401930();
  }
  return;
}

