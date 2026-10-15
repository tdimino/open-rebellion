
void __fastcall FUN_004c2270(int param_1)

{
  int *piVar1;
  int iVar2;
  
  iVar2 = 0;
  piVar1 = (int *)(*(int *)(param_1 + 0x168) + 0x108);
  do {
    if (*piVar1 != 0) {
      return;
    }
    iVar2 = iVar2 + 1;
    piVar1 = piVar1 + 1;
  } while (iVar2 < 4);
  iVar2 = FUN_0041cd80(4);
  if (iVar2 < 4) {
    *(int *)(*(int *)(param_1 + 0x168) + 0x108 + iVar2 * 4) = DAT_006b28cc + 10;
  }
  return;
}

