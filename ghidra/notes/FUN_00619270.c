
DWORD __cdecl FUN_00619270(undefined4 *param_1)

{
  uint uVar1;
  uint uVar2;
  char *pcVar3;
  DWORD DVar4;
  char *pcVar5;
  DWORD *pDVar6;
  char *pcVar7;
  int iVar8;
  DWORD local_8;
  DWORD local_4;
  
  uVar1 = param_1[4];
  if ((int)param_1[1] < 0) {
    param_1[1] = 0;
  }
  local_4 = FUN_0061bb80(uVar1,0,1);
  if ((int)local_4 < 0) {
    return 0xffffffff;
  }
  uVar2 = param_1[3];
  if ((uVar2 & 0x108) == 0) {
    return local_4 - param_1[1];
  }
  pcVar7 = (char *)*param_1;
  pcVar3 = (char *)param_1[2];
  local_8 = (int)pcVar7 - (int)pcVar3;
  if ((uVar2 & 3) == 0) {
    if ((uVar2 & 0x80) == 0) {
      pDVar6 = FUN_0061c250();
      *pDVar6 = 0x16;
      return 0xffffffff;
    }
  }
  else {
    pcVar5 = pcVar3;
    if ((*(byte *)((&DAT_006bed20)[(int)uVar1 >> 5] + 4 + (uVar1 & 0x1f) * 0x24) & 0x80) != 0) {
      for (; pcVar5 < pcVar7; pcVar5 = pcVar5 + 1) {
        if (*pcVar5 == '\n') {
          local_8 = local_8 + 1;
        }
      }
    }
  }
  if (local_4 == 0) {
    return local_8;
  }
  if ((*(byte *)(param_1 + 3) & 1) == 0) goto LAB_006193e7;
  if (param_1[1] == 0) {
    return local_4;
  }
  pcVar7 = pcVar7 + (param_1[1] - (int)pcVar3);
  iVar8 = (uVar1 & 0x1f) * 0x24;
  if ((*(byte *)(iVar8 + 4 + (&DAT_006bed20)[(int)uVar1 >> 5]) & 0x80) != 0) {
    DVar4 = FUN_0061bb80(uVar1,0,2);
    if (DVar4 == local_4) {
      pcVar5 = (char *)param_1[2];
      pcVar3 = pcVar5 + (int)pcVar7;
      for (; pcVar5 < pcVar3; pcVar5 = pcVar5 + 1) {
        if (*pcVar5 == '\n') {
          pcVar7 = pcVar7 + 1;
        }
      }
      if ((param_1[3] & 0x2000) != 0) {
LAB_006193de:
        pcVar7 = pcVar7 + 1;
      }
    }
    else {
      FUN_0061bb80(uVar1,local_4,0);
      if (((pcVar7 < (char *)0x201) && ((param_1[3] & 8) != 0)) && ((param_1[3] & 0x400) == 0)) {
        pcVar7 = (char *)0x200;
      }
      else {
        pcVar7 = (char *)param_1[6];
      }
      if ((*(byte *)(iVar8 + 4 + (&DAT_006bed20)[(int)uVar1 >> 5]) & 4) != 0) goto LAB_006193de;
    }
  }
  local_4 = local_4 - (int)pcVar7;
LAB_006193e7:
  return local_4 + local_8;
}

