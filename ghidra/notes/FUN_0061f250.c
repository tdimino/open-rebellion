
undefined4 * FUN_0061f250(void)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  undefined4 *puVar4;
  
  puVar4 = (undefined4 *)0x0;
  FUN_0061d9d0(2);
  iVar3 = 0;
  if (0 < DAT_006bfe40) {
    do {
      iVar1 = *(int *)(DAT_006bee24 + iVar3 * 4);
      if (iVar1 == 0) {
        iVar3 = iVar3 * 4;
        uVar2 = FUN_00619170(0x38);
        *(undefined4 *)(DAT_006bee24 + iVar3) = uVar2;
        if (*(int *)(DAT_006bee24 + iVar3) != 0) {
          InitializeCriticalSection((LPCRITICAL_SECTION)(*(int *)(DAT_006bee24 + iVar3) + 0x20));
          EnterCriticalSection((LPCRITICAL_SECTION)(*(int *)(DAT_006bee24 + iVar3) + 0x20));
          puVar4 = *(undefined4 **)(DAT_006bee24 + iVar3);
        }
        break;
      }
      if ((*(byte *)(iVar1 + 0xc) & 0x83) == 0) {
        FUN_0061dab0(iVar3,iVar1);
        iVar1 = *(int *)(DAT_006bee24 + iVar3 * 4);
        if ((*(byte *)(iVar1 + 0xc) & 0x83) == 0) {
          puVar4 = *(undefined4 **)(DAT_006bee24 + iVar3 * 4);
          break;
        }
        FUN_0061db20(iVar3,iVar1);
      }
      iVar3 = iVar3 + 1;
    } while (iVar3 < DAT_006bfe40);
  }
  if (puVar4 != (undefined4 *)0x0) {
    puVar4[1] = 0;
    puVar4[3] = 0;
    puVar4[2] = 0;
    *puVar4 = 0;
    puVar4[7] = 0;
    puVar4[4] = 0xffffffff;
  }
  FUN_0061da50(2);
  return puVar4;
}

