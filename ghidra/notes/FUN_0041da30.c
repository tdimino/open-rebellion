
void FUN_0041da30(void)

{
  int iVar1;
  
  iVar1 = FUN_00422ca0();
  if ((iVar1 != 0) && (DAT_006b14b4 == 0)) {
    FUN_0042d620(iVar1);
    DAT_006b14b4 = 1;
  }
  DAT_006b14bc = 0;
  if (DAT_006b14b8 != 0) {
    DAT_006b14b8 = 0;
    FUN_00401930();
    return;
  }
  return;
}

