
int __fastcall FUN_004e19c0(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = *(int *)(param_1 + 0x2c);
  iVar2 = ((((*(int *)(param_1 + 0x24) * *(int *)(iVar1 + 0x184)) / 100) * 0x50) / 100 -
          *(int *)(iVar1 + 0x250)) - *(int *)(iVar1 + 0x1d8);
  if (*(int *)(iVar1 + 0x184) - *(int *)(iVar1 + 0x188) < iVar2) {
    iVar2 = 0;
  }
  return iVar2;
}

