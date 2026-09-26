
void FUN_00623850(void)

{
  char cVar1;
  int iVar2;
  char *pcVar3;
  
  iVar2 = -1;
  pcVar3 = DAT_006bebf4;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 + -1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 1;
  } while (cVar1 != '\0');
  DAT_006bebf0 = (uint)(iVar2 == -5);
  iVar2 = -1;
  pcVar3 = DAT_006bebf8;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 + -1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 1;
  } while (cVar1 != '\0');
  DAT_006bebe8 = (uint)(iVar2 == -5);
  DAT_006bebe4 = 0;
  if (DAT_006bebf0 == 0) {
    DAT_006bebec = FUN_00624160(DAT_006bebf4);
  }
  else {
    DAT_006bebec = 2;
  }
  EnumSystemLocalesA((LOCALE_ENUMPROCA)&lpLocaleEnumProc_006238e0,1);
  if ((((DAT_006bebfc & 0x100) == 0) || ((DAT_006bebfc & 0x200) == 0)) || ((DAT_006bebfc & 7) == 0))
  {
    DAT_006bebfc = 0;
  }
  return;
}

