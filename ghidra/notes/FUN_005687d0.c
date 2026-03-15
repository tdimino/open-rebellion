
void __fastcall FUN_005687d0(int param_1)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  char *pcVar5;
  char *pcVar6;
  
  if (((*(int *)(param_1 + 4) != 0) && (*(int *)(param_1 + 8) != 0)) &&
     (*(int *)(param_1 + 0x10) == 0)) {
    iVar2 = FUN_00618b70(*(int *)(param_1 + 8) * *(int *)(param_1 + 4));
    *(int *)(param_1 + 0x10) = iVar2;
    if ((iVar2 != 0) && (iVar2 = 0, 0 < *(int *)(param_1 + 4))) {
      do {
        uVar3 = 0xffffffff;
        pcVar5 = &DAT_006b120c;
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
        pcVar6 = (char *)(iVar2 * *(int *)(param_1 + 8) + *(int *)(param_1 + 0x10));
        for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
          *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
          pcVar5 = pcVar5 + 4;
          pcVar6 = pcVar6 + 4;
        }
        iVar2 = iVar2 + 1;
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *pcVar6 = *pcVar5;
          pcVar5 = pcVar5 + 1;
          pcVar6 = pcVar6 + 1;
        }
      } while (iVar2 < *(int *)(param_1 + 4));
    }
  }
  return;
}

