
undefined4 __cdecl
FUN_00627f10(uint param_1,uint param_2,uint param_3,int param_4,byte param_5,short *param_6)

{
  short *psVar1;
  ushort uVar2;
  uint uVar3;
  undefined1 uVar4;
  char cVar5;
  uint uVar6;
  int iVar7;
  short *psVar8;
  short *psVar9;
  int iVar10;
  short sVar11;
  int iVar12;
  undefined1 local_1c;
  undefined1 local_1b;
  undefined1 local_1a;
  undefined1 local_19;
  undefined1 local_18;
  undefined1 local_17;
  undefined1 local_16;
  undefined1 local_15;
  undefined1 local_14;
  undefined1 local_13;
  undefined1 local_12;
  undefined1 local_11;
  undefined2 local_10;
  undefined4 uStack_e;
  undefined4 uStack_a;
  undefined1 local_6;
  char cStack_5;
  
  psVar1 = param_6;
  local_1c = 0xcc;
  local_1b = 0xcc;
  local_1a = 0xcc;
  local_19 = 0xcc;
  local_18 = 0xcc;
  local_17 = 0xcc;
  local_16 = 0xcc;
  local_15 = 0xcc;
  local_14 = 0xcc;
  local_13 = 0xcc;
  uVar6 = param_3 & 0x7fff;
  local_12 = 0xfb;
  local_11 = 0x3f;
  if ((param_3 & 0x8000) == 0) {
    *(undefined1 *)(param_6 + 1) = 0x20;
  }
  else {
    *(undefined1 *)(param_6 + 1) = 0x2d;
  }
  if ((((short)uVar6 == 0) && (param_2 == 0)) && (param_1 == 0)) {
    *param_6 = 0;
LAB_0062811f:
    *(undefined1 *)(psVar1 + 1) = 0x20;
    *(undefined1 *)((int)psVar1 + 3) = 1;
    *(undefined1 *)(psVar1 + 2) = 0x30;
    *(undefined1 *)((int)psVar1 + 5) = 0;
    return 1;
  }
  if ((short)uVar6 == 0x7fff) {
    *param_6 = 1;
    if (((param_2 != 0x80000000) || (param_1 != 0)) && ((param_2 & 0x40000000) == 0)) {
      *(undefined4 *)(param_6 + 2) = DAT_0066f46c;
      param_6[4] = DAT_0066f470;
      uVar4 = DAT_0066f472;
      *(undefined1 *)((int)param_6 + 3) = 6;
      *(undefined1 *)(param_6 + 5) = uVar4;
      return 0;
    }
    if ((((param_3 & 0x8000) != 0) && (param_2 == 0xc0000000)) && (param_1 == 0)) {
      *(undefined4 *)(param_6 + 2) = DAT_0066f464;
      sVar11 = DAT_0066f468;
      *(undefined1 *)((int)param_6 + 3) = 5;
      param_6[4] = sVar11;
      return 0;
    }
    if ((param_2 == 0x80000000) && (param_1 == 0)) {
      *(undefined4 *)(param_6 + 2) = DAT_0066f45c;
      sVar11 = DAT_0066f460;
      *(undefined1 *)((int)param_6 + 3) = 5;
      param_6[4] = sVar11;
      return 0;
    }
    *(undefined4 *)(param_6 + 2) = DAT_0066f454;
    param_6[4] = DAT_0066f458;
    uVar4 = DAT_0066f45a;
    *(undefined1 *)((int)param_6 + 3) = 6;
    *(undefined1 *)(param_6 + 5) = uVar4;
    return 0;
  }
  local_6 = (undefined1)uVar6;
  cStack_5 = (char)(uVar6 >> 8);
  local_10 = 0;
  sVar11 = (short)(((uVar6 >> 8) + (param_2 >> 0x18) * 2) * 0x4d + -0x134312f4 + uVar6 * 0x4d10 >>
                  0x10);
  uStack_a = param_2;
  uStack_e = param_1;
  FUN_006288a0((int *)&local_10,-(int)sVar11,1);
  if (0x3ffe < CONCAT11(cStack_5,local_6)) {
    sVar11 = sVar11 + 1;
    FUN_006285e0((int *)&local_10,(int *)&local_1c);
  }
  *psVar1 = sVar11;
  iVar10 = param_4;
  if (((param_5 & 1) != 0) && (iVar10 = param_4 + sVar11, param_4 + sVar11 < 1)) {
    *psVar1 = 0;
    goto LAB_0062811f;
  }
  if (0x15 < iVar10) {
    iVar10 = 0x15;
  }
  uVar2 = CONCAT11(cStack_5,local_6);
  local_6 = 0;
  cStack_5 = '\0';
  iVar7 = 8;
  iVar12 = uVar2 - 0x3ffe;
  do {
    FUN_00627620((uint *)&local_10);
    iVar7 = iVar7 + -1;
  } while (iVar7 != 0);
  if (iVar12 < 0) {
    for (uVar6 = -iVar12 & 0xff; uVar6 != 0; uVar6 = uVar6 - 1) {
      FUN_00627650((uint *)&local_10);
    }
  }
  psVar1 = psVar1 + 2;
  iVar10 = iVar10 + 1;
  psVar8 = psVar1;
  uVar6 = uStack_e;
  uVar3 = uStack_a;
  if (0 < iVar10) {
    do {
      uStack_a._2_2_ = (undefined2)(uVar3 >> 0x10);
      uStack_a._0_2_ = (undefined2)uVar3;
      uStack_e._2_2_ = (undefined2)(uVar6 >> 0x10);
      uStack_e._0_2_ = (undefined2)uVar6;
      param_1 = CONCAT22((undefined2)uStack_e,local_10);
      param_2 = CONCAT22((undefined2)uStack_a,uStack_e._2_2_);
      param_3 = CONCAT13(cStack_5,CONCAT12(local_6,uStack_a._2_2_));
      uStack_e = uVar6;
      uStack_a = uVar3;
      FUN_00627620((uint *)&local_10);
      FUN_00627620((uint *)&local_10);
      FUN_006275b0((uint *)&local_10,&param_1);
      FUN_00627620((uint *)&local_10);
      cVar5 = cStack_5 + '0';
      cStack_5 = '\0';
      *(char *)psVar8 = cVar5;
      psVar8 = (short *)((int)psVar8 + 1);
      iVar10 = iVar10 + -1;
      uVar6 = uStack_e;
      uVar3 = uStack_a;
    } while (iVar10 != 0);
  }
  psVar9 = psVar8 + -1;
  if (*(char *)((int)psVar8 + -1) < '5') {
    if (psVar1 <= psVar9) {
      do {
        if ((char)*psVar9 != '0') break;
        psVar9 = (short *)((int)psVar9 + -1);
      } while (psVar1 <= psVar9);
      if (psVar1 <= psVar9) goto LAB_00628276;
    }
    *(char *)psVar1 = '0';
    *param_6 = 0;
    *(undefined1 *)(param_6 + 1) = 0x20;
    *(undefined1 *)((int)param_6 + 3) = 1;
    *(undefined1 *)((int)param_6 + 5) = 0;
    return 1;
  }
  if (psVar1 <= psVar9) {
    do {
      if ((char)*psVar9 != '9') break;
      *(char *)psVar9 = '0';
      psVar9 = (short *)((int)psVar9 + -1);
    } while (psVar1 <= psVar9);
    if (psVar1 <= psVar9) {
      *(char *)psVar9 = (char)*psVar9 + '\x01';
      goto LAB_00628276;
    }
  }
  psVar9 = (short *)((int)psVar9 + 1);
  *param_6 = *param_6 + 1;
  *(char *)psVar9 = *(char *)psVar9 + '\x01';
LAB_00628276:
  cVar5 = ((char)psVar9 - (char)param_6) + -3;
  *(char *)((int)param_6 + 3) = cVar5;
  *(undefined1 *)((int)param_6 + cVar5 + 4) = 0;
  return 1;
}

