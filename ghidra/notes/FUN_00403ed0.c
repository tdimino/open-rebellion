
undefined4 __cdecl FUN_00403ed0(int param_1)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  if ((0 < param_1) && (param_1 < 0x21)) {
    uVar1 = (**(code **)(**(int **)(&DAT_006b059c + param_1 * 4) + 0x28))();
  }
  return uVar1;
}

