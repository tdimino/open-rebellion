
int __cdecl FUN_0061b3a0(int param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int local_4;
  
  iVar4 = 0;
  local_4 = 0;
  iVar3 = 0;
  FUN_0061d9d0(2);
  if (0 < DAT_006bfe40) {
    do {
      iVar2 = *(int *)(DAT_006bee24 + iVar4 * 4);
      if ((iVar2 != 0) && ((*(byte *)(iVar2 + 0xc) & 0x83) != 0)) {
        FUN_0061dab0(iVar4,iVar2);
        piVar1 = *(int **)(DAT_006bee24 + iVar4 * 4);
        if ((piVar1[3] & 0x83U) != 0) {
          if (param_1 == 1) {
            iVar2 = FUN_0061b2e0(piVar1);
            if (iVar2 != -1) {
              local_4 = local_4 + 1;
            }
          }
          else if (((param_1 == 0) && ((piVar1[3] & 2U) != 0)) &&
                  (iVar2 = FUN_0061b2e0(piVar1), iVar2 == -1)) {
            iVar3 = -1;
          }
        }
        FUN_0061db20(iVar4,*(int *)(DAT_006bee24 + iVar4 * 4));
      }
      iVar4 = iVar4 + 1;
    } while (iVar4 < DAT_006bfe40);
  }
  FUN_0061da50(2);
  if (param_1 != 1) {
    local_4 = iVar3;
  }
  return local_4;
}

