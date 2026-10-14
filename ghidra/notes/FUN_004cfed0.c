
undefined4 __fastcall FUN_004cfed0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  iVar1 = FUN_005f4960();
  if (iVar1 == 1) {
    if (*(int *)(param_1 + 0x38) != 0x3ec) {
      if (*(int *)(param_1 + 0x38) != 0x3f4) {
        *(undefined4 *)(param_1 + 0x38) = 0x3f4;
        return 0;
      }
      uVar2 = FUN_004cff20();
      return uVar2;
    }
    uVar2 = FUN_004d0010();
  }
  return uVar2;
}

