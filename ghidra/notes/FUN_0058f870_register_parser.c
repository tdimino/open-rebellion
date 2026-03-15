
undefined4 __cdecl FUN_0058f870(byte *param_1,undefined4 param_2)

{
  byte bVar1;
  int iVar2;
  int iVar3;
  byte *pbVar4;
  bool bVar5;
  
  bVar5 = DAT_006bbe58 + 1 < 10;
  if (bVar5) {
    iVar2 = FUN_0058f900(param_1);
    bVar5 = iVar2 == 0;
  }
  if (bVar5) {
    iVar2 = 0x28;
    iVar3 = DAT_006bbe58 * 0x2c;
    (&DAT_006bbe88)[DAT_006bbe58 * 0xb] = param_2;
    pbVar4 = &DAT_006bbe60 + iVar3;
    bVar1 = *param_1;
    *pbVar4 = bVar1;
    while ((bVar1 != 0 && (iVar2 = iVar2 + -1, iVar2 != 0))) {
      bVar1 = param_1[1];
      pbVar4 = pbVar4 + 1;
      param_1 = param_1 + 1;
      *pbVar4 = bVar1;
    }
    iVar2 = DAT_006bbe58 * 0x2c;
    DAT_006bbe58 = DAT_006bbe58 + 1;
    (&DAT_006bbe87)[iVar2] = 0;
  }
  return 0;
}

