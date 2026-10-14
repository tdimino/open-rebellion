
undefined4 __cdecl FUN_00403eb0(int *param_1)

{
  int iVar1;
  
  if (param_1 != (int *)0x0) {
    iVar1 = (**(code **)(*param_1 + 0x2c))();
    *(int **)(&DAT_006b059c + iVar1 * 4) = param_1;
  }
  return 0;
}

