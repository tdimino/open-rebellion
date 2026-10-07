
void __fastcall FUN_004cc990(int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  
  puVar1 = (undefined4 *)(param_1 + 0x1c);
  iVar2 = FUN_005f4960(puVar1);
  if (iVar2 != 1) {
    iVar2 = FUN_005f4960(puVar1);
    if (iVar2 != 2) {
      *puVar1 = 1;
      *(undefined4 *)(param_1 + 0x20) = 0;
    }
  }
  return;
}

