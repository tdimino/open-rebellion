
undefined4 FUN_0061aad0(int param_1,byte *param_2)

{
  char cVar1;
  undefined4 uVar2;
  undefined *puVar3;
  char *pcVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  char *pcVar9;
  char *pcVar10;
  uint local_90;
  undefined2 local_8c;
  uint local_88;
  char local_84 [132];
  
  pcVar4 = FUN_0061ad50(param_2,local_84,&local_90,&local_88);
  if (pcVar4 == (char *)0x0) {
    return 0;
  }
  uVar6 = 0xffffffff;
  pcVar4 = local_84;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar1 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar1 != '\0');
  pcVar4 = (char *)FUN_00619170(~uVar6);
  uVar6 = DAT_006be850;
  if (pcVar4 == (char *)0x0) {
    return 0;
  }
  uVar2 = *(undefined4 *)(&DAT_006be838 + param_1 * 4);
  puVar3 = (undefined *)(&DAT_006acb1c)[param_1 * 3];
  uVar7 = 0xffffffff;
  pcVar9 = local_84;
  do {
    pcVar10 = pcVar9;
    if (uVar7 == 0) break;
    uVar7 = uVar7 - 1;
    pcVar10 = pcVar9 + 1;
    cVar1 = *pcVar9;
    pcVar9 = pcVar10;
  } while (cVar1 != '\0');
  uVar7 = ~uVar7;
  pcVar9 = pcVar10 + -uVar7;
  pcVar10 = pcVar4;
  for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
    *(undefined4 *)pcVar10 = *(undefined4 *)pcVar9;
    pcVar9 = pcVar9 + 4;
    pcVar10 = pcVar10 + 4;
  }
  for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
    *pcVar10 = *pcVar9;
    pcVar9 = pcVar9 + 1;
    pcVar10 = pcVar10 + 1;
  }
  (&DAT_006acb1c)[param_1 * 3] = pcVar4;
  *(uint *)(&DAT_006be838 + param_1 * 4) = local_90 & 0xffff;
  *(uint *)(&DAT_006bebc0 + param_1 * 6) = local_90;
  *(undefined2 *)(&DAT_006bebc4 + param_1 * 6) = local_8c;
  if (param_1 == 2) {
    DAT_006be850 = local_88;
  }
  iVar5 = (**(code **)(&DAT_006acb20 + param_1 * 0xc))();
  if (iVar5 != 0) {
    (&DAT_006acb1c)[param_1 * 3] = puVar3;
    FUN_00618e60(pcVar4);
    *(undefined4 *)(&DAT_006be838 + param_1 * 4) = uVar2;
    DAT_006be850 = uVar6;
    return 0;
  }
  if (puVar3 != &DAT_006acb10) {
    FUN_00618e60(puVar3);
  }
  return (&DAT_006acb1c)[param_1 * 3];
}

