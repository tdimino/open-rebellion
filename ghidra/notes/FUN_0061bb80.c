
DWORD __cdecl FUN_0061bb80(uint param_1,LONG param_2,DWORD param_3)

{
  DWORD DVar1;
  DWORD *pDVar2;
  
  if ((param_1 < DAT_006bee20) &&
     ((*(byte *)((&DAT_006bed20)[(int)param_1 >> 5] + 4 + (param_1 & 0x1f) * 0x24) & 1) != 0)) {
    FUN_00624780(param_1);
    DVar1 = FUN_0061bc00(param_1,param_2,param_3);
    FUN_006247f0(param_1);
    return DVar1;
  }
  pDVar2 = FUN_0061c250();
  *pDVar2 = 9;
  pDVar2 = FUN_0061c260();
  *pDVar2 = 0;
  return 0xffffffff;
}

