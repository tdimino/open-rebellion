
void __fastcall FUN_004bf390(int param_1)

{
  int iVar1;
  
  if (*(int *)(param_1 + 0x3c) == 1) {
    FUN_004bf490();
    *(undefined4 *)(param_1 + 0x3c) = 2;
  }
  else {
    if (*(int *)(param_1 + 0x3c) != 2) {
      *(undefined4 *)(param_1 + 0x3c) = 1;
      return;
    }
    iVar1 = 1;
    if (*(int *)(param_1 + 0x60) != 0) {
      iVar1 = FUN_004e5440();
    }
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x3c) = 0;
      *(undefined4 *)(param_1 + 0x38) = 0x3fc;
      return;
    }
  }
  return;
}

