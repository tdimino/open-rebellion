
int __cdecl FUN_0061b640(uint param_1,char *param_2,uint param_3)

{
  int iVar1;
  DWORD *pDVar2;
  
  if ((param_1 < DAT_006bee20) &&
     ((*(byte *)((&DAT_006bed20)[(int)param_1 >> 5] + 4 + (param_1 & 0x1f) * 0x24) & 1) != 0)) {
    FUN_00624780(param_1);
    iVar1 = FUN_0061b6c0(param_1,param_2,param_3);
    FUN_006247f0(param_1);
    return iVar1;
  }
  pDVar2 = FUN_0061c250();
  *pDVar2 = 9;
  pDVar2 = FUN_0061c260();
  *pDVar2 = 0;
  return -1;
}

