
undefined * __fastcall FUN_00528820(int param_1)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_1 + 0x24);
  if ((DAT_006b9020 & 1) == 0) {
    DAT_006b9020 = DAT_006b9020 | 1;
    FUN_004f4340((undefined4 *)&DAT_006b9010);
    FUN_00618c20(0x528880);
  }
  if ((uVar1 & 0x30) == 0) {
    return (undefined *)(*(int *)(param_1 + 0x54) + 0x18);
  }
  return &DAT_006b9010;
}

