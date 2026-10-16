
int __fastcall FUN_004cc220(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = *(int *)(param_1 + 0x2c);
  iVar2 = (((*(int *)(param_1 + 0x24) * *(int *)(iVar1 + 0x184)) / 100) * 0x5a) / 100 -
          *(int *)(iVar1 + 0x24c);
  if (*(int *)(iVar1 + 0x184) - *(int *)(iVar1 + 0x188) < iVar2) {
    iVar2 = 0;
  }
  return iVar2;
}

