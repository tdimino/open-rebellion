
undefined4 __fastcall FUN_004b1650(int param_1)

{
  short sVar1;
  
  if (*(int *)(*(int *)(param_1 + 0x10) + 200) < 2) {
    sVar1 = FUN_005f50e0(*(int *)(param_1 + 0x10) + 0x2c);
    if (sVar1 != 0) {
      return 0;
    }
    if (*(int *)(*(int *)(param_1 + 0x10) + 200) < 1) {
      return 0;
    }
  }
  return 1;
}

