
undefined4 __fastcall FUN_004edc80(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = *(int *)(param_1 + 0x2c);
  uVar2 = 0;
  if (iVar1 != 0) {
    if (*(int *)(iVar1 + 0x40) == 0) {
      if (*(int *)(iVar1 + 0x44) == 0) {
        return 0;
      }
    }
    else if (*(int *)(iVar1 + 0x44) == 0) {
      return 1;
    }
    if (*(int *)(iVar1 + 0x40) == 0) {
      uVar2 = 2;
    }
  }
  return uVar2;
}

