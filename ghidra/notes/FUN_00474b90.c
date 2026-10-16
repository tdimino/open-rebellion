
void __fastcall FUN_00474b90(int param_1)

{
  int iVar1;
  uint uVar2;
  
  *(undefined4 *)(param_1 + 0x34) = 0;
  iVar1 = FUN_00476110(*(void **)(param_1 + 0x4c),(undefined4 *)(param_1 + 0x2c));
  if (iVar1 != 0) {
    *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) + *(int *)(iVar1 + 0xb4);
  }
  uVar2 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x4c) + 4));
  if ((uVar2 & 1) == 0) {
    *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) - *(int *)(*(int *)(param_1 + 0x4c) + 0x164)
    ;
    return;
  }
  *(int *)(param_1 + 0x34) =
       *(int *)(param_1 + 0x34) + *(int *)(*(int *)(param_1 + 0x4c) + 0x164) * -2;
  return;
}

