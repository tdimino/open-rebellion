
undefined4 __thiscall FUN_005f32b0(void *this,int param_1)

{
  char cVar1;
  ushort uVar2;
  char *pcVar3;
  undefined4 uVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  char *pcVar8;
  LPCSTR *ppCVar9;
  LPCSTR *ppCVar10;
  char *pcVar11;
  undefined4 local_4;
  
  uVar2 = FUN_00479ac0();
  uVar5 = 0xffffffff;
  ppCVar9 = &lpString_006a7f40;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar1 = *(char *)ppCVar9;
    ppCVar9 = (LPCSTR *)((int)ppCVar9 + 1);
  } while (cVar1 != '\0');
  pcVar3 = (char *)FUN_00618b70((uint)uVar2 + (~uVar5 - 1) * param_1);
  uVar5 = 0xffffffff;
  pcVar8 = &DAT_006b120c;
  do {
    pcVar11 = pcVar8;
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    pcVar11 = pcVar8 + 1;
    cVar1 = *pcVar8;
    pcVar8 = pcVar11;
  } while (cVar1 != '\0');
  uVar5 = ~uVar5;
  pcVar8 = pcVar11 + -uVar5;
  pcVar11 = pcVar3;
  for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    *(undefined4 *)pcVar11 = *(undefined4 *)pcVar8;
    pcVar8 = pcVar8 + 4;
    pcVar11 = pcVar11 + 4;
  }
  for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
    *pcVar11 = *pcVar8;
    pcVar8 = pcVar8 + 1;
    pcVar11 = pcVar11 + 1;
  }
  if (0 < param_1) {
    do {
      uVar5 = 0xffffffff;
      ppCVar9 = &lpString_006a7f40;
      do {
        ppCVar10 = ppCVar9;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        ppCVar10 = (LPCSTR *)((int)ppCVar9 + 1);
        cVar1 = *(char *)ppCVar9;
        ppCVar9 = ppCVar10;
      } while (cVar1 != '\0');
      uVar5 = ~uVar5;
      iVar7 = -1;
      pcVar8 = pcVar3;
      do {
        pcVar11 = pcVar8;
        if (iVar7 == 0) break;
        iVar7 = iVar7 + -1;
        pcVar11 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar11;
      } while (cVar1 != '\0');
      pcVar8 = (char *)((int)ppCVar10 - uVar5);
      pcVar11 = pcVar11 + -1;
      for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(undefined4 *)pcVar11 = *(undefined4 *)pcVar8;
        pcVar8 = pcVar8 + 4;
        pcVar11 = pcVar11 + 4;
      }
      param_1 = param_1 + -1;
      for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar11 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar11 = pcVar11 + 1;
      }
    } while (param_1 != 0);
  }
  uVar4 = FUN_005f36d0(*(char **)((int)this + 4),pcVar3,&local_4);
  *(undefined4 *)((int)this + 8) = uVar4;
  FUN_00618b60(pcVar3);
  FUN_00618b60(*(undefined **)((int)this + 4));
  *(undefined4 *)((int)this + 4) = local_4;
  return *(undefined4 *)((int)this + 8);
}

