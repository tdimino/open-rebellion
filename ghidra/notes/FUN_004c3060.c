
void __fastcall FUN_004c3060(void *param_1)

{
  int iVar1;
  
  if (DAT_006b28c4 != 1) {
    if (DAT_006b28c4 == 2) {
      FUN_004c30c0(param_1,DAT_006b28c8);
    }
    else if (DAT_006b28c4 == 3) {
      iVar1 = FUN_004f3dd0(1,1);
      if (iVar1 == 0) {
        FUN_0041d830((uint)param_1);
      }
      else {
        FUN_0041d830(*(uint *)(iVar1 + 0xbc));
      }
    }
  }
  DAT_006b28c4 = 0;
  DAT_006b28c8 = 0;
  return;
}

