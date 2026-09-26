
void __cdecl FUN_005c12a0(uint param_1,undefined4 param_2,undefined4 param_3)

{
  int *piVar1;
  
  if (DAT_006bc700 != (void *)0x0) {
    piVar1 = (int *)FUN_005c1cd0(DAT_006bc700,param_1);
    if (piVar1 != (int *)0x0) {
      (**(code **)(*piVar1 + 0x1c))(param_2,param_3);
    }
  }
  return;
}

