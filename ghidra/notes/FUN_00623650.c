
undefined4 __cdecl FUN_00623650(char *param_1,undefined2 *param_2,int param_3)

{
  int iVar1;
  uint uVar2;
  BOOL BVar3;
  
  if (DAT_006bec04 == (code *)0x0) {
    iVar1 = FUN_00623fb0();
    if (iVar1 == 0) {
      DAT_006bec04 = FUN_00623ff0;
    }
    else {
      DAT_006bec04 = GetLocaleInfoA_exref;
    }
  }
  if (param_1 != (char *)0x0) {
    DAT_006bebf4 = param_1;
    if (*param_1 != '\0') {
      FUN_006237f0(0x6af7a0,0x40,(int *)&DAT_006bebf4);
    }
    DAT_006bebf8 = param_1 + 0x40;
    if ((DAT_006bebf8 != (char *)0x0) && (*DAT_006bebf8 != '\0')) {
      FUN_006237f0(0x6af9a8,0x16,(int *)&DAT_006bebf8);
    }
    DAT_006bebfc = 0;
    if ((DAT_006bebf4 != (char *)0x0) && (*DAT_006bebf4 != '\0')) {
      if ((DAT_006bebf8 == (char *)0x0) || (*DAT_006bebf8 == '\0')) {
        FUN_00623bc0();
      }
      else {
        FUN_00623850();
      }
      goto LAB_00623715;
    }
    if ((DAT_006bebf8 != (char *)0x0) && (*DAT_006bebf8 != '\0')) {
      FUN_00623d00();
      goto LAB_00623715;
    }
  }
  FUN_00623de0();
LAB_00623715:
  if ((((DAT_006bebfc != 0) && (uVar2 = FUN_00623e00((byte *)(param_1 + 0x80)), uVar2 != 0)) &&
      (BVar3 = IsValidCodePage(uVar2 & 0xffff), BVar3 != 0)) &&
     (BVar3 = IsValidLocale(DAT_006bebe4,1), BVar3 != 0)) {
    if (param_2 != (undefined2 *)0x0) {
      *param_2 = (undefined2)DAT_006bebe4;
      param_2[1] = (undefined2)DAT_006bec00;
      param_2[2] = (short)uVar2;
    }
    if (param_3 == 0) {
      return 1;
    }
    iVar1 = (*DAT_006bec04)(DAT_006bebe4,0x1001,param_3,0x40);
    if ((iVar1 != 0) &&
       (iVar1 = (*DAT_006bec04)(DAT_006bec00,0x1002,param_3 + 0x40,0x40), iVar1 != 0)) {
      FUN_00626a20(uVar2,(char *)(param_3 + 0x80),10);
      return 1;
    }
  }
  return 0;
}

