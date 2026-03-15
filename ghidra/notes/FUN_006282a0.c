
uint __cdecl FUN_006282a0(LPSTR param_1,LPCWSTR param_2,uint param_3)

{
  uint uVar1;
  bool bVar2;
  
  InterlockedIncrement((LONG *)&lpAddend_006bfe48);
  bVar2 = DAT_006bfe44 != 0;
  if (bVar2) {
    InterlockedDecrement((LONG *)&lpAddend_006bfe48);
    FUN_0061d9d0(0x13);
  }
  uVar1 = FUN_00628320(param_1,param_2,param_3);
  if (!bVar2) {
    InterlockedDecrement((LONG *)&lpAddend_006bfe48);
    return uVar1;
  }
  FUN_0061da50(0x13);
  return uVar1;
}

