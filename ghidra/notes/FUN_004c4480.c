
void __fastcall FUN_004c4480(int param_1)

{
  int iVar1;
  
  iVar1 = FUN_0041cd80(2);
  if (iVar1 < 2) {
    *(int *)(*(int *)(param_1 + 0x168) + 0x124 + iVar1 * 4) = DAT_006b28cc + 10;
  }
  return;
}

