
undefined4 __cdecl
FUN_00553720(undefined4 param_1,int param_2,undefined4 *param_3,undefined4 *param_4)

{
  int *piVar1;
  int iVar2;
  
  *param_3 = 0;
  *param_4 = 0;
  for (piVar1 = (int *)FUN_0041c210(param_2); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[4]) {
    iVar2 = (**(code **)(*piVar1 + 0x14))();
    if (iVar2 == 0) {
      *param_4 = 1;
    }
    else {
      *param_3 = 1;
    }
  }
  return 1;
}

