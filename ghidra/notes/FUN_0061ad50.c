
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

char * __cdecl FUN_0061ad50(byte *param_1,char *param_2,undefined4 *param_3,uint *param_4)

{
  char cVar1;
  byte bVar2;
  byte *pbVar3;
  int iVar4;
  char *pcVar5;
  uint uVar6;
  uint uVar7;
  byte *pbVar8;
  char *pcVar9;
  bool bVar10;
  char local_88 [136];
  
  if (param_1 == (byte *)0x0) {
    return (char *)0x0;
  }
  if ((*param_1 == 0x43) && (param_1[1] == 0)) {
    *param_2 = 'C';
    param_2[1] = '\0';
    if (param_3 != (undefined4 *)0x0) {
      *(undefined2 *)param_3 = 0;
      *(undefined2 *)((int)param_3 + 2) = 0;
      *(undefined2 *)(param_3 + 1) = 0;
    }
    pcVar5 = param_2;
    if (param_4 != (uint *)0x0) {
      *param_4 = 0;
      return param_2;
    }
  }
  else {
    pbVar3 = &DAT_006acbe8;
    pbVar8 = param_1;
    do {
      bVar2 = *pbVar3;
      bVar10 = bVar2 < *pbVar8;
      if (bVar2 != *pbVar8) {
LAB_0061aded:
        iVar4 = (1 - (uint)bVar10) - (uint)(bVar10 != 0);
        goto LAB_0061adf2;
      }
      if (bVar2 == 0) break;
      bVar2 = pbVar3[1];
      bVar10 = bVar2 < pbVar8[1];
      if (bVar2 != pbVar8[1]) goto LAB_0061aded;
      pbVar3 = pbVar3 + 2;
      pbVar8 = pbVar8 + 2;
    } while (bVar2 != 0);
    iVar4 = 0;
LAB_0061adf2:
    if (iVar4 != 0) {
      pbVar3 = &DAT_006acb60;
      pbVar8 = param_1;
      do {
        bVar2 = *pbVar3;
        bVar10 = bVar2 < *pbVar8;
        if (bVar2 != *pbVar8) {
LAB_0061ae25:
          iVar4 = (1 - (uint)bVar10) - (uint)(bVar10 != 0);
          goto LAB_0061ae2a;
        }
        if (bVar2 == 0) break;
        bVar2 = pbVar3[1];
        bVar10 = bVar2 < pbVar8[1];
        if (bVar2 != pbVar8[1]) goto LAB_0061ae25;
        pbVar3 = pbVar3 + 2;
        pbVar8 = pbVar8 + 2;
      } while (bVar2 != 0);
      iVar4 = 0;
LAB_0061ae2a:
      if (iVar4 != 0) {
        iVar4 = FUN_0061afa0(local_88,(char *)param_1);
        if (iVar4 != 0) {
          return (char *)0x0;
        }
        iVar4 = FUN_00623650(local_88,(undefined2 *)&DAT_006be758,(int)local_88);
        if (iVar4 == 0) {
          return (char *)0x0;
        }
        DAT_006be760 = _DAT_006be75c & 0xffff;
        FUN_0061b0b0(&DAT_006acbe8,local_88);
        if (*param_1 == 0) {
          uVar6 = 0xffffffff;
          pcVar5 = &DAT_006acbe8;
          do {
            pcVar9 = pcVar5;
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            pcVar9 = pcVar5 + 1;
            cVar1 = *pcVar5;
            pcVar5 = pcVar9;
          } while (cVar1 != '\0');
          uVar6 = ~uVar6;
          pbVar8 = (byte *)(pcVar9 + -uVar6);
          pbVar3 = &DAT_006acb60;
          for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(undefined4 *)pbVar3 = *(undefined4 *)pbVar8;
            pbVar8 = pbVar8 + 4;
            pbVar3 = pbVar3 + 4;
          }
        }
        else {
          uVar6 = 0xffffffff;
          do {
            pbVar8 = param_1;
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            pbVar8 = param_1 + 1;
            bVar2 = *param_1;
            param_1 = pbVar8;
          } while (bVar2 != 0);
          uVar6 = ~uVar6;
          pbVar8 = pbVar8 + -uVar6;
          pbVar3 = &DAT_006acb60;
          for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(undefined4 *)pbVar3 = *(undefined4 *)pbVar8;
            pbVar8 = pbVar8 + 4;
            pbVar3 = pbVar3 + 4;
          }
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
          *pbVar3 = *pbVar8;
          pbVar8 = pbVar8 + 1;
          pbVar3 = pbVar3 + 1;
        }
      }
    }
    if (param_3 != (undefined4 *)0x0) {
      *param_3 = DAT_006be758;
      *(undefined2 *)(param_3 + 1) = DAT_006be75c;
    }
    if (param_4 != (uint *)0x0) {
      *param_4 = DAT_006be760;
    }
    uVar6 = 0xffffffff;
    pcVar5 = &DAT_006acbe8;
    do {
      pcVar9 = pcVar5;
      if (uVar6 == 0) break;
      uVar6 = uVar6 - 1;
      pcVar9 = pcVar5 + 1;
      cVar1 = *pcVar5;
      pcVar5 = pcVar9;
    } while (cVar1 != '\0');
    uVar6 = ~uVar6;
    pcVar5 = &DAT_006acbe8;
    pcVar9 = pcVar9 + -uVar6;
    for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
      *(undefined4 *)param_2 = *(undefined4 *)pcVar9;
      pcVar9 = pcVar9 + 4;
      param_2 = param_2 + 4;
    }
    for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
      *param_2 = *pcVar9;
      pcVar9 = pcVar9 + 1;
      param_2 = param_2 + 1;
    }
  }
  return pcVar5;
}

