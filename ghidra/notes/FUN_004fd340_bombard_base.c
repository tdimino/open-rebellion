
undefined4 FUN_004fd340(void)

{
  undefined4 uVar1;
  
  if (DAT_006b2b08 != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x004fd349. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*DAT_006b2b08)();
    return uVar1;
  }
  return 0;
}

