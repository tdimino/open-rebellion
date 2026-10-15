
void FUN_00417780(int param_1,uint param_2)

{
  undefined4 uVar1;
  
  if ((param_1 == 0x4c9) && ((param_2 & 0xffff) == (DAT_006b147c & 0xffff) + DAT_006b1480 * 0x10000)
     ) {
    FUN_00610c60(&DAT_006b1478);
    if ((DAT_006b1490 & 2) != 0) {
      FUN_00417520(DAT_006b147c - 1);
      return;
    }
    uVar1 = FUN_0041d3b0();
    FUN_00417520(uVar1);
  }
  return;
}

