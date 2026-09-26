
undefined4 __cdecl FUN_006245e0(uint param_1,HANDLE param_2)

{
  DWORD *pDVar1;
  int iVar2;
  
  if (param_1 < DAT_006bee20) {
    iVar2 = (param_1 & 0x1f) * 0x24;
    if (*(int *)((&DAT_006bed20)[(int)param_1 >> 5] + iVar2) == -1) {
      if (DAT_006acaf4 == 1) {
        if (param_1 == 0) {
          SetStdHandle(0xfffffff6,param_2);
        }
        else {
          if (param_1 == 1) {
            SetStdHandle(0xfffffff5,param_2);
            *(HANDLE *)(DAT_006bed20 + 0x24) = param_2;
            return 0;
          }
          if (param_1 == 2) {
            SetStdHandle(0xfffffff4,param_2);
            *(HANDLE *)(DAT_006bed20 + 0x48) = param_2;
            return 0;
          }
        }
      }
      *(HANDLE *)((&DAT_006bed20)[(int)param_1 >> 5] + iVar2) = param_2;
      return 0;
    }
  }
  pDVar1 = FUN_0061c250();
  *pDVar1 = 9;
  pDVar1 = FUN_0061c260();
  *pDVar1 = 0;
  return 0xffffffff;
}

