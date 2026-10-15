
BOOL FUN_00442c60(void)

{
  BOOL BVar1;
  
  BVar1 = 1;
  if (DAT_006b28f8 != (HHOOK)0x0) {
    BVar1 = UnhookWindowsHookEx(DAT_006b28f8);
    DAT_006b28f8 = (HHOOK)0x0;
  }
  return BVar1;
}

