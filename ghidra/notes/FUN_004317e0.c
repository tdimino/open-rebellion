
int __fastcall FUN_004317e0(int param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  
  iVar4 = 0;
  piVar3 = (int *)(param_1 + 0x48);
  iVar2 = 10;
  do {
    if (((*(uint *)(*piVar3 + 0x30) & 1) == 0) && ((*(uint *)(*piVar3 + 0x30) & 0x20) != 0)) {
      iVar1 = FUN_00433de0();
      iVar4 = iVar4 + iVar1;
    }
    piVar3 = piVar3 + 1;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  if (0 < iVar4) {
    iVar4 = iVar4 / (*(int *)(param_1 + 0x84) + *(int *)(param_1 + 0x80));
  }
  return iVar4;
}

