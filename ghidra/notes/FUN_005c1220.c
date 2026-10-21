
void __cdecl FUN_005c1220(uint param_1)

{
  int *piVar1;
  
  if (DAT_006bc700 != (void *)0x0) {
    piVar1 = (int *)FUN_005c1cd0(DAT_006bc700,param_1);
    if (piVar1 != (int *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x005c123c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*piVar1 + 8))();
      return;
    }
  }
  return;
}

