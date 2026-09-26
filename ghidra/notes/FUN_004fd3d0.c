
undefined4 FUN_004fd3d0(void)

{
  undefined4 uVar1;
  
  if (DAT_006b2af0 != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x004fd3d9. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*DAT_006b2af0)();
    return uVar1;
  }
  return 1;
}

