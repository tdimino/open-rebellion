
void __fastcall FUN_004d2970(int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  
  puVar1 = (undefined4 *)(param_1 + 0x1c);
  iVar2 = FUN_005f4960(puVar1);
  if (iVar2 != 1) {
    iVar2 = FUN_005f4960(puVar1);
    if ((iVar2 != 2) && (0 < *(int *)(*(int *)(param_1 + 0x2c) + 0x264))) {
      *puVar1 = 1;
      *(undefined4 *)(param_1 + 0x20) = 0;
    }
  }
  return;
}

