
int __fastcall FUN_005c6530(int param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = 0;
  for (piVar1 = (int *)(**(code **)(*(int *)(param_1 + 0x1c) + 0x10))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    (**(code **)(*piVar1 + 0x10))();
    iVar2 = FUN_005ba330();
    iVar3 = iVar3 + iVar2;
  }
  return iVar3;
}

