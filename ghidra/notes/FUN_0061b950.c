
int __cdecl FUN_0061b950(uint param_1,char *param_2,DWORD param_3)

{
  int *piVar1;
  char cVar2;
  byte bVar3;
  BOOL BVar4;
  undefined *puVar5;
  DWORD *pDVar6;
  int iVar7;
  int iVar8;
  char *pcVar9;
  DWORD DVar10;
  char *pcVar11;
  char *pcVar12;
  char cStack_9;
  DWORD local_8;
  int *local_4;
  
  iVar8 = 0;
  if (param_3 != 0) {
    piVar1 = &DAT_006bed20 + ((int)param_1 >> 5);
    iVar7 = (param_1 & 0x1f) * 0x24;
    bVar3 = *(byte *)(iVar7 + 4 + (&DAT_006bed20)[(int)param_1 >> 5]);
    if ((bVar3 & 2) == 0) {
      pcVar11 = param_2;
      if (((bVar3 & 0x48) != 0) &&
         (cVar2 = *(char *)(iVar7 + (&DAT_006bed20)[(int)param_1 >> 5] + 5), cVar2 != '\n')) {
        *param_2 = cVar2;
        param_3 = param_3 - 1;
        pcVar11 = param_2 + 1;
        iVar8 = 1;
        *(undefined1 *)(iVar7 + 5 + *piVar1) = 10;
      }
      local_4 = piVar1;
      BVar4 = ReadFile(*(HANDLE *)(iVar7 + *piVar1),pcVar11,param_3,&local_8,(LPOVERLAPPED)0x0);
      if (BVar4 != 0) {
        iVar8 = iVar8 + local_8;
        bVar3 = *(byte *)(iVar7 + 4 + *piVar1);
        if ((bVar3 & 0x80) != 0) {
          if ((local_8 == 0) || (*param_2 != '\n')) {
            bVar3 = bVar3 & 0xfb;
          }
          else {
            bVar3 = bVar3 | 4;
          }
          *(byte *)(iVar7 + 4 + *piVar1) = bVar3;
          pcVar9 = param_2 + iVar8;
          pcVar11 = param_2;
          pcVar12 = param_2;
          if (param_2 < pcVar9) {
            while (cVar2 = *pcVar12, cVar2 != '\x1a') {
              if (cVar2 == '\r') {
                if (pcVar12 < pcVar9 + -1) {
                  if (pcVar12[1] == '\n') {
                    pcVar12 = pcVar12 + 2;
                    *pcVar11 = '\n';
                    goto LAB_0061bb28;
                  }
                  *pcVar11 = '\r';
                  pcVar11 = pcVar11 + 1;
                  pcVar12 = pcVar12 + 1;
                }
                else {
                  DVar10 = 0;
                  pcVar12 = pcVar12 + 1;
                  BVar4 = ReadFile(*(HANDLE *)(iVar7 + *local_4),&cStack_9,1,&local_8,
                                   (LPOVERLAPPED)0x0);
                  if (BVar4 == 0) {
                    DVar10 = GetLastError();
                  }
                  if ((DVar10 == 0) && (local_8 != 0)) {
                    if ((*(byte *)(iVar7 + 4 + *local_4) & 0x48) == 0) {
                      if ((pcVar11 == param_2) && (cStack_9 == '\n')) {
                        *pcVar11 = '\n';
                        goto LAB_0061bb28;
                      }
                      FUN_0061bc00(param_1,-1,1);
                      if (cStack_9 != '\n') goto LAB_0061bb25;
                    }
                    else {
                      if (cStack_9 == '\n') {
                        *pcVar11 = '\n';
                        goto LAB_0061bb28;
                      }
                      *pcVar11 = '\r';
                      pcVar11 = pcVar11 + 1;
                      *(char *)(iVar7 + 5 + *local_4) = cStack_9;
                    }
                  }
                  else {
LAB_0061bb25:
                    *pcVar11 = '\r';
LAB_0061bb28:
                    pcVar11 = pcVar11 + 1;
                  }
                }
              }
              else {
                *pcVar11 = cVar2;
                pcVar11 = pcVar11 + 1;
                pcVar12 = pcVar12 + 1;
              }
              if (pcVar9 <= pcVar12) {
                return (int)pcVar11 - (int)param_2;
              }
            }
            bVar3 = *(byte *)(iVar7 + 4 + *local_4);
            if ((bVar3 & 0x40) == 0) {
              *(byte *)(iVar7 + 4 + *local_4) = bVar3 | 2;
            }
          }
          iVar8 = (int)pcVar11 - (int)param_2;
        }
        return iVar8;
      }
      puVar5 = (undefined *)GetLastError();
      if (puVar5 == &DAT_00000005) {
        pDVar6 = FUN_0061c250();
        *pDVar6 = 9;
        pDVar6 = FUN_0061c260();
        *pDVar6 = 5;
        return -1;
      }
      if (puVar5 != (undefined *)0x6d) {
        FUN_0061c1d0(puVar5);
        return -1;
      }
    }
  }
  return 0;
}

