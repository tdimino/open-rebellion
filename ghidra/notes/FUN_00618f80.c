
undefined4 __cdecl FUN_00618f80(int *param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 0xffffffff;
  if ((*(byte *)(param_1 + 3) & 0x83) != 0) {
    uVar2 = FUN_0061b320(param_1);
    FUN_0061ef30(param_1);
    iVar1 = FUN_0061b540(param_1[4]);
    if (iVar1 < 0) {
      param_1[3] = 0;
      return 0xffffffff;
    }
    if ((undefined *)param_1[7] != (undefined *)0x0) {
      FUN_00618e60((undefined *)param_1[7]);
      param_1[7] = 0;
    }
  }
  param_1[3] = 0;
  return uVar2;
}

