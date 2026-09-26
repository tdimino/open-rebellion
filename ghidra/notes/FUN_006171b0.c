
undefined4 __fastcall FUN_006171b0(int param_1)

{
  byte bVar1;
  char *pcVar2;
  uint uVar3;
  char *pcVar4;
  DWORD DVar5;
  int iVar6;
  uint uVar7;
  
  if (*(uint *)(param_1 + 0x4c) == 0xffffffff) {
    return 0xffffffff;
  }
  if (*(int *)(param_1 + 8) == 0) {
    pcVar2 = *(char **)(param_1 + 0x18);
    if (*(char **)(param_1 + 0x1c) < pcVar2) {
      uVar7 = 0;
    }
    else {
      uVar7 = (int)*(char **)(param_1 + 0x1c) - (int)pcVar2;
    }
    if ((uVar7 != 0) &&
       (uVar3 = FUN_0061b640(*(uint *)(param_1 + 0x4c),pcVar2,uVar7), uVar3 != uVar7)) {
      if ((int)uVar3 < 1) {
        return 0xffffffff;
      }
      if (*(int *)(param_1 + 0x20) != 0) {
        *(int *)(param_1 + 0x1c) = *(int *)(param_1 + 0x1c) - uVar3;
      }
      FUN_0061be90(*(undefined4 **)(param_1 + 0x18),
                   (undefined4 *)(uVar3 + (int)*(undefined4 **)(param_1 + 0x18)),uVar7 - uVar3);
      return 0xffffffff;
    }
    pcVar2 = *(char **)(param_1 + 0x2c);
    pcVar4 = *(char **)(param_1 + 0x28);
    *(undefined4 *)(param_1 + 0x18) = 0;
    *(undefined4 *)(param_1 + 0x1c) = 0;
    *(undefined4 *)(param_1 + 0x20) = 0;
    if (pcVar4 < pcVar2) {
      iVar6 = (int)pcVar2 - (int)pcVar4;
    }
    else {
      iVar6 = 0;
    }
    if (0 < iVar6) {
      uVar7 = *(uint *)(param_1 + 0x4c);
      bVar1 = *(byte *)((&DAT_006bed20)[(int)uVar7 >> 5] + 4 + (uVar7 & 0x1f) * 0x24);
      if ((bVar1 & 0x80) != 0) {
        for (; pcVar4 < pcVar2; pcVar4 = pcVar4 + 1) {
          if (*pcVar4 == '\n') {
            iVar6 = iVar6 + 1;
          }
        }
        if ((bVar1 & 2) != 0) {
          iVar6 = iVar6 + 1;
        }
      }
      DVar5 = FUN_0061bb80(uVar7,-iVar6,1);
      if (DVar5 == 0xffffffff) {
        return 0xffffffff;
      }
    }
    *(undefined4 *)(param_1 + 0x24) = 0;
    *(undefined4 *)(param_1 + 0x28) = 0;
    *(undefined4 *)(param_1 + 0x2c) = 0;
    *(undefined4 *)(param_1 + 0xc) = 0xffffffff;
  }
  return 0;
}

