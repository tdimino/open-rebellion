
int __fastcall FUN_005c5eb0(int *param_1)

{
  int *piVar1;
  int iVar2;
  
  iVar2 = 0;
  for (piVar1 = (int *)(**(code **)(*param_1 + 0x10))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    iVar2 = iVar2 + 1;
  }
  return iVar2;
}

