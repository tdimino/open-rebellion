
undefined4 __fastcall FUN_005d2190(int *param_1)

{
  int *piVar1;
  undefined4 uVar2;
  
  piVar1 = (int *)(**(code **)(*param_1 + 0x10))();
  if (piVar1 != (int *)0x0) {
                    /* WARNING: Could not recover jumptable at 0x005d219d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar2 = (**(code **)(*piVar1 + 0x10))();
    return uVar2;
  }
  return 0;
}

