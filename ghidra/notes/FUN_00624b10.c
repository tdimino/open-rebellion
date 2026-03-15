
LPSTR __cdecl FUN_00624b10(LPSTR param_1,WCHAR param_2)

{
  LPSTR pCVar1;
  bool bVar2;
  
  InterlockedIncrement((LONG *)&lpAddend_006bfe48);
  bVar2 = DAT_006bfe44 != 0;
  if (bVar2) {
    InterlockedDecrement((LONG *)&lpAddend_006bfe48);
    FUN_0061d9d0(0x13);
  }
  pCVar1 = FUN_00624b80(param_1,param_2);
  if (!bVar2) {
    InterlockedDecrement((LONG *)&lpAddend_006bfe48);
    return pCVar1;
  }
  FUN_0061da50(0x13);
  return pCVar1;
}

