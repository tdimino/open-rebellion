
void __cdecl FUN_0042d650(int param_1,int param_2)

{
  int iVar1;
  int iVar2;
  LPBYTE pBVar3;
  char *pcVar4;
  
  iVar1 = FUN_006037f0(9);
  iVar2 = FUN_006037f0(0xd);
  pBVar3 = FUN_00401440();
  if (param_1 != DAT_006b14d0) {
    DAT_006b14d0 = param_1;
    if (iVar1 != 0) {
      FUN_00604460(&DAT_006be4c8,9);
    }
    pcVar4 = s_alsprite_dll_006a86e0;
    if (param_1 != 1) {
      pcVar4 = s_emsprite_dll_006a86d0;
    }
    FUN_00603810((char *)pBVar3,pcVar4,9);
  }
  if (param_2 == 0) {
    if (iVar2 != 0) {
      FUN_00604460(&DAT_006be4c8,0xd);
    }
    DAT_006b14d4 = 0;
  }
  else if (DAT_006b14d4 != param_1) {
    DAT_006b14d4 = param_1;
    if (iVar2 != 0) {
      FUN_00604460(&DAT_006be4c8,0xd);
    }
    pcVar4 = s_albrief_dll_006a86c4;
    if (param_1 != 1) {
      pcVar4 = s_embrief_dll_006a86b8;
    }
    FUN_00603810((char *)pBVar3,pcVar4,0xd);
  }
  if (pBVar3 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar3);
  }
  return;
}

