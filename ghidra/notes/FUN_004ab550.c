
undefined4 __fastcall FUN_004ab550(undefined4 *param_1)

{
  undefined4 uVar1;
  
  uVar1 = *param_1;
  if ((*(byte *)(param_1 + 1) & 0xd) != 0) {
    uVar1 = 0;
  }
  return uVar1;
}

