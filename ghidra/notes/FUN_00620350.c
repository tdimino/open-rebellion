
char * __cdecl FUN_00620350(char *param_1,uint param_2,int *param_3,char param_4)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  char *pcVar4;
  
  iVar1 = param_3[1];
  uVar3 = iVar1 - 1;
  if ((param_4 != '\0') && (iVar2 = *param_3, uVar3 == param_2)) {
    param_1[uVar3 + (iVar2 == 0x2d)] = '0';
    param_1[iVar1 + (uint)(iVar2 == 0x2d)] = '\0';
  }
  pcVar4 = param_1;
  if (*param_3 == 0x2d) {
    *param_1 = '-';
    pcVar4 = param_1 + 1;
  }
  if (param_3[1] < 1) {
    FUN_00620540(pcVar4,1);
    *pcVar4 = '0';
    pcVar4 = pcVar4 + 1;
  }
  else {
    pcVar4 = pcVar4 + param_3[1];
  }
  if (0 < (int)param_2) {
    FUN_00620540(pcVar4,1);
    *pcVar4 = DAT_006af3b0;
    iVar1 = param_3[1];
    if (iVar1 < 0) {
      if ((param_4 != '\0') || (-iVar1 <= (int)param_2)) {
        param_2 = -iVar1;
      }
      FUN_00620540(pcVar4 + 1,param_2);
      uVar3 = param_2 >> 2;
      pcVar4 = pcVar4 + 1;
      while (uVar3 != 0) {
        uVar3 = uVar3 - 1;
        builtin_strncpy(pcVar4,"0000",4);
        pcVar4 = pcVar4 + 4;
      }
      for (uVar3 = param_2 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar4 = '0';
        pcVar4 = pcVar4 + 1;
      }
    }
  }
  return param_1;
}

