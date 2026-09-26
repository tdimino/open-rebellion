
undefined4 __cdecl FUN_0061afa0(char *param_1,char *param_2)

{
  char cVar1;
  size_t _Count;
  int iVar2;
  uint uVar3;
  uint uVar4;
  char *pcVar5;
  char *pcVar6;
  
  pcVar5 = param_1;
  for (iVar2 = 0x22; iVar2 != 0; iVar2 = iVar2 + -1) {
    pcVar5[0] = '\0';
    pcVar5[1] = '\0';
    pcVar5[2] = '\0';
    pcVar5[3] = '\0';
    pcVar5 = pcVar5 + 4;
  }
  if (*param_2 == '\0') {
    return 0;
  }
  if ((*param_2 != '.') || (param_2[1] == '\0')) {
    iVar2 = 0;
    _Count = _strcspn(param_2,"_.,");
    if (_Count == 0) {
      return 0xffffffff;
    }
    do {
      cVar1 = param_2[_Count];
      if (((iVar2 != 0) || (0x3f < (int)_Count)) || (pcVar5 = param_1, cVar1 == '.')) {
        if (((iVar2 == 1) && ((int)_Count < 0x40)) && (cVar1 != '_')) {
          pcVar5 = param_1 + 0x40;
        }
        else {
          if (iVar2 != 2) {
            return 0xffffffff;
          }
          if ((cVar1 != '\0') && (cVar1 != ',')) {
            return 0xffffffff;
          }
          pcVar5 = param_1 + 0x80;
        }
      }
      _strncpy(pcVar5,param_2,_Count);
      if ((cVar1 == ',') || (cVar1 == '\0')) {
        return 0;
      }
      param_2 = param_2 + _Count + 1;
      iVar2 = iVar2 + 1;
      _Count = _strcspn(param_2,"_.,");
      if (_Count == 0) {
        return 0xffffffff;
      }
    } while( true );
  }
  uVar3 = 0xffffffff;
  pcVar5 = param_2 + 1;
  do {
    pcVar6 = pcVar5;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar6 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar6;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  pcVar5 = pcVar6 + -uVar3;
  pcVar6 = param_1 + 0x80;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar6 = pcVar6 + 4;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar6 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar6 = pcVar6 + 1;
  }
  return 0;
}

