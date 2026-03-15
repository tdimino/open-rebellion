
void __cdecl FUN_0061c1d0(undefined *param_1)

{
  DWORD *pDVar1;
  undefined **ppuVar2;
  int iVar3;
  
  pDVar1 = FUN_0061c260();
  iVar3 = 0;
  *pDVar1 = (DWORD)param_1;
  ppuVar2 = (undefined **)&DAT_006acf30;
  do {
    if (param_1 == *ppuVar2) {
      pDVar1 = FUN_0061c250();
      *pDVar1 = (&DAT_006acf34)[iVar3 * 2];
      return;
    }
    ppuVar2 = ppuVar2 + 2;
    iVar3 = iVar3 + 1;
  } while (ppuVar2 < &PTR_FUN_006ad098);
  if (((undefined *)0x12 < param_1) && (param_1 < (undefined *)0x25)) {
    pDVar1 = FUN_0061c250();
    *pDVar1 = 0xd;
    return;
  }
  if (((undefined *)0xbb < param_1) && (param_1 < &DAT_000000cb)) {
    pDVar1 = FUN_0061c250();
    *pDVar1 = 8;
    return;
  }
  pDVar1 = FUN_0061c250();
  *pDVar1 = 0x16;
  return;
}

