
int __cdecl FUN_00442c90(int param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if (param_1 != 0) {
    iVar1 = FUN_00442cd0(param_1);
    if (iVar1 != 0) {
      iVar1 = FUN_00442c90(iVar1);
      *(int *)(param_1 + 0x58) = iVar1;
    }
    iVar1 = FUN_00442c90(*(int *)(param_1 + 8));
    if (iVar1 == 0) {
      iVar1 = *(int *)(param_1 + 0x58);
    }
  }
  return iVar1;
}

