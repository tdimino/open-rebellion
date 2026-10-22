
undefined4 FUN_005a5fb0(void)

{
  undefined4 uVar1;
  
  if (DAT_006bc464 != (int *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x005a5fbc. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar1 = (**(code **)(*DAT_006bc464 + 8))();
    return uVar1;
  }
  return 0;
}

