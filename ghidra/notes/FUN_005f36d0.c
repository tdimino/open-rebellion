
undefined4 __cdecl FUN_005f36d0(char *param_1,char *param_2,undefined4 *param_3)

{
  char cVar1;
  undefined4 *puVar2;
  ushort uVar3;
  uint uVar4;
  uint uVar5;
  char *pcVar6;
  int iVar7;
  char *pcVar8;
  char *pcVar9;
  
  puVar2 = param_3;
  *param_3 = 0;
  param_3 = (undefined4 *)0x1;
  if ((param_1 != (char *)0x0) || (param_2 != (char *)0x0)) {
    uVar4 = FUN_005f3630(param_1);
    uVar5 = FUN_005f3630(param_2);
    uVar3 = FUN_00479ac0();
    pcVar6 = (char *)FUN_00618b70((uVar4 & 0xffff) + (uVar5 & 0xffff) + (uint)uVar3);
    *puVar2 = pcVar6;
    if (pcVar6 == (char *)0x0) {
      param_3 = (undefined4 *)0x0;
    }
    else {
      uVar4 = 0xffffffff;
      pcVar8 = &DAT_006b120c;
      do {
        pcVar9 = pcVar8;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar9 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar9;
      } while (cVar1 != '\0');
      uVar4 = ~uVar4;
      pcVar8 = pcVar9 + -uVar4;
      for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined4 *)pcVar6 = *(undefined4 *)pcVar8;
        pcVar8 = pcVar8 + 4;
        pcVar6 = pcVar6 + 4;
      }
      for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar6 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar6 = pcVar6 + 1;
      }
      if (param_1 != (char *)0x0) {
        uVar4 = 0xffffffff;
        do {
          pcVar6 = param_1;
          if (uVar4 == 0) break;
          uVar4 = uVar4 - 1;
          pcVar6 = param_1 + 1;
          cVar1 = *param_1;
          param_1 = pcVar6;
        } while (cVar1 != '\0');
        uVar4 = ~uVar4;
        iVar7 = -1;
        pcVar8 = (char *)*puVar2;
        do {
          pcVar9 = pcVar8;
          if (iVar7 == 0) break;
          iVar7 = iVar7 + -1;
          pcVar9 = pcVar8 + 1;
          cVar1 = *pcVar8;
          pcVar8 = pcVar9;
        } while (cVar1 != '\0');
        pcVar6 = pcVar6 + -uVar4;
        pcVar8 = pcVar9 + -1;
        for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
          *(undefined4 *)pcVar8 = *(undefined4 *)pcVar6;
          pcVar6 = pcVar6 + 4;
          pcVar8 = pcVar8 + 4;
        }
        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
          *pcVar8 = *pcVar6;
          pcVar6 = pcVar6 + 1;
          pcVar8 = pcVar8 + 1;
        }
      }
      if (param_2 != (char *)0x0) {
        uVar4 = 0xffffffff;
        do {
          pcVar6 = param_2;
          if (uVar4 == 0) break;
          uVar4 = uVar4 - 1;
          pcVar6 = param_2 + 1;
          cVar1 = *param_2;
          param_2 = pcVar6;
        } while (cVar1 != '\0');
        uVar4 = ~uVar4;
        iVar7 = -1;
        pcVar8 = (char *)*puVar2;
        do {
          pcVar9 = pcVar8;
          if (iVar7 == 0) break;
          iVar7 = iVar7 + -1;
          pcVar9 = pcVar8 + 1;
          cVar1 = *pcVar8;
          pcVar8 = pcVar9;
        } while (cVar1 != '\0');
        pcVar6 = pcVar6 + -uVar4;
        pcVar8 = pcVar9 + -1;
        for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
          *(undefined4 *)pcVar8 = *(undefined4 *)pcVar6;
          pcVar6 = pcVar6 + 4;
          pcVar8 = pcVar8 + 4;
        }
        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
          *pcVar8 = *pcVar6;
          pcVar6 = pcVar6 + 1;
          pcVar8 = pcVar8 + 1;
        }
        return 1;
      }
    }
  }
  return param_3;
}

