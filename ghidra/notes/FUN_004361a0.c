
int __fastcall FUN_004361a0(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = (**(code **)(**(int **)(param_1 + 0x10 + *(int *)(param_1 + 0x18) * 4) + 0x24))();
  if ((iVar1 == 1) &&
     (iVar2 = 1 - *(int *)(param_1 + 0x18), *(int *)(param_1 + 0x18) = iVar2, iVar2 != 0)) {
    iVar1 = 0;
  }
  return iVar1;
}

