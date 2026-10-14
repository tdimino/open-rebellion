
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005b7ce0(int param_1)

{
  int *piVar1;
  uint uVar2;
  
  uVar2 = 0;
  piVar1 = (int *)(param_1 + 0x17c);
  do {
    if ((float)*piVar1 != _DAT_0066c4f0) {
      return 1;
    }
    uVar2 = uVar2 + 1;
    piVar1 = piVar1 + 0x1d;
  } while (uVar2 < 4);
  return 0;
}

