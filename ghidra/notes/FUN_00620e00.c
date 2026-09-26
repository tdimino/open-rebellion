
uint __cdecl FUN_00620e00(uint param_1)

{
  uint uVar1;
  byte bVar2;
  int iVar3;
  uint local_8 [2];
  
  uVar1 = param_1;
  if (param_1 < 0x100) {
    if (((&DAT_006be971)[param_1] & 0x20) == 0x20) {
      return (uint)(byte)(&DAT_006bea78)[param_1];
    }
  }
  else {
    bVar2 = (byte)(param_1 >> 8);
    param_1 = CONCAT31(CONCAT21(param_1._2_2_,(char)param_1),bVar2);
    if (((&DAT_006be971)[bVar2] & 4) != 0) {
      iVar3 = FUN_00620ea0(DAT_006beb7c,0x200,(char *)&param_1,(LPCWSTR)&DAT_00000002,
                           (LPWSTR)local_8,2,DAT_006beb78,1);
      if (iVar3 != 0) {
        return (local_8[0] & 0xff) * 0x100 + (local_8[0] >> 8 & 0xff);
      }
    }
  }
  return uVar1;
}

