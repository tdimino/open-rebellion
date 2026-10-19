
undefined4 FUN_004fd330(void)

{
  undefined4 uVar1;
  
  if (DAT_006b2b04 != (code *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x004fd339. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (*DAT_006b2b04)();
    return uVar1;
  }
  return 0;
}

