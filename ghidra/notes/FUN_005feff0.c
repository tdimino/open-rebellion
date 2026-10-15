
void FUN_005feff0(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar2 = 0;
  iVar1 = DAT_006be4d0;
  while ((iVar1 != 0 && (iVar2 == 0))) {
    iVar2 = 0;
    if (param_1 == *(int *)(iVar1 + 0x10)) {
      iVar2 = *(int *)(iVar1 + 0xc);
    }
    iVar1 = *(int *)(iVar1 + 8);
  }
  return;
}

