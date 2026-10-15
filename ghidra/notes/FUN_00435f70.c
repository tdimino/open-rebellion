
undefined4 __thiscall FUN_00435f70(int param_1,undefined4 *param_2)

{
  int iVar1;
  
  iVar1 = (**(code **)(**(int **)(param_1 + 0x10 + *(int *)(param_1 + 0x18) * 4) + 0x10))();
  if ((iVar1 != 0) &&
     (iVar1 = 1 - *(int *)(param_1 + 0x18), *(int *)(param_1 + 0x18) = iVar1, iVar1 == 0)) {
    *(undefined4 *)(param_1 + 4) = 1;
    *param_2 = 1;
    return 1;
  }
  return 0;
}

