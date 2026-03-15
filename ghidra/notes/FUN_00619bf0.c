
uint __cdecl FUN_00619bf0(uint param_1)

{
  bool bVar1;
  
  if (DAT_006be840 == 0) {
    if ((0x60 < (int)param_1) && ((int)param_1 < 0x7b)) {
      return param_1 - 0x20;
    }
  }
  else {
    InterlockedIncrement((LONG *)&lpAddend_006bfe48);
    bVar1 = DAT_006bfe44 != 0;
    if (bVar1) {
      InterlockedDecrement((LONG *)&lpAddend_006bfe48);
      FUN_0061d9d0(0x13);
    }
    param_1 = FUN_00619c80(param_1);
    if (bVar1) {
      FUN_0061da50(0x13);
      return param_1;
    }
    InterlockedDecrement((LONG *)&lpAddend_006bfe48);
  }
  return param_1;
}

