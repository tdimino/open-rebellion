
undefined4 FUN_00623f20(int param_1)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  char *pcVar4;
  undefined1 *puStack_88;
  undefined4 uStack_84;
  undefined1 local_78 [112];
  int iStack_8;
  
  puStack_88 = local_78;
  uStack_84 = 0x78;
  iVar2 = (*DAT_006bec04)((ushort)param_1 & 0x3ff | 0x400,1);
  if (iVar2 == 0) {
    return 0;
  }
  iVar2 = FUN_00624120((char *)&puStack_88);
  if ((param_1 != iVar2) && (iStack_8 != 0)) {
    iVar2 = FUN_00624160(DAT_006bebf4);
    uVar3 = 0xffffffff;
    pcVar4 = DAT_006bebf4;
    do {
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      cVar1 = *pcVar4;
      pcVar4 = pcVar4 + 1;
    } while (cVar1 != '\0');
    if (iVar2 == ~uVar3 - 1) {
      return 0;
    }
  }
  return 1;
}

