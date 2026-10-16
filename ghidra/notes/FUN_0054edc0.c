
int __fastcall FUN_0054edc0(int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0041c210(param_1);
  if (iVar2 != 0) {
    iVar1 = *(int *)(iVar2 + 0x40);
    iVar3 = FUN_004fd340();
    if (iVar3 < iVar1) {
      iVar2 = 0;
    }
  }
  return iVar2;
}

