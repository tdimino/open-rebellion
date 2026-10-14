
int __fastcall FUN_005f5e90(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = *(int *)(param_1 + 4);
  iVar2 = 0;
  if (iVar1 != 0) {
    iVar2 = *(int *)(iVar1 + 8);
    *(int *)(param_1 + 4) = iVar2;
    if (iVar2 != 0) {
      *(undefined4 *)(iVar2 + 4) = 0;
      *(undefined4 *)(iVar1 + 8) = 0;
      *(int *)(param_1 + 0xc) = *(int *)(param_1 + 0xc) + -1;
      return iVar1;
    }
    *(undefined4 *)(param_1 + 8) = 0;
    *(undefined4 *)(iVar1 + 8) = 0;
    *(int *)(param_1 + 0xc) = *(int *)(param_1 + 0xc) + -1;
    iVar2 = iVar1;
  }
  return iVar2;
}

