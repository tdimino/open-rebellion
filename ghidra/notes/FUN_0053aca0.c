
uint __fastcall FUN_0053aca0(int *param_1)

{
  uint uVar1;
  
  uVar1 = FUN_00539b60(param_1);
  if ((uVar1 != 0) && (uVar1 = (uint)~param_1[0x18] >> 2 & 1, uVar1 != 0)) {
    if (param_1[0x16] == 1) {
      uVar1 = (uint)~param_1[0x18] >> 1 & 1;
    }
    if (uVar1 != 0) {
      if (param_1[0x16] == 2) {
        uVar1 = (uint)param_1[0x18] >> 1 & 1;
      }
      if (((uVar1 != 0) && (uVar1 = (uint)(param_1[0x16] != 3), uVar1 != 0)) &&
         ((((byte)param_1[9] & 0xc0) == 0xc0 ||
          (((*(byte *)(param_1 + 0x18) & 1) != 0 || ((*(byte *)(param_1 + 0x14) & 1) == 0)))))) {
        uVar1 = (uint)(param_1[0x16] == 4);
      }
    }
  }
  return uVar1;
}

