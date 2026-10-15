
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005a9ec0(int *param_1,float param_2,float param_3)

{
  float fVar1;
  float fVar2;
  float fVar3;
  int *piVar4;
  int iVar5;
  undefined4 uVar6;
  int *piVar7;
  float *pfVar8;
  float *pfVar9;
  int *piVar10;
  float fStack_58;
  uint uStack_54;
  float afStack_50 [3];
  int iStack_44;
  int iStack_40;
  int iStack_3c;
  int iStack_38;
  int iStack_34;
  float fStack_30;
  int iStack_2c;
  int iStack_28;
  int iStack_24;
  float fStack_18;
  float fStack_8;
  
  if (param_1[0x282] == 1) {
    for (piVar4 = (int *)(**(code **)(param_1[0x268] + 0x10))(); piVar4 != (int *)0x0;
        piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
      iVar5 = (**(code **)(*piVar4 + 0x10))();
      *(undefined4 *)(iVar5 + 0x108) = 1;
      uVar6 = (**(code **)(*piVar4 + 0x10))();
      FUN_0059ed70(uVar6);
      FUN_0059f4d0(uVar6);
    }
    FUN_0059ed70();
    FUN_005a6150();
    return 1;
  }
  piVar4 = param_1 + 0x20a;
  iVar5 = 2;
  piVar7 = param_1 + 0x206;
  do {
    piVar4[-2] = piVar4[-0x14];
    piVar4[-1] = piVar4[-0x13];
    *piVar4 = piVar4[-0x12];
    piVar4[1] = piVar4[-0x11];
    piVar4[6] = piVar4[-0xc];
    piVar4[7] = piVar4[-0xb];
    piVar4[8] = piVar4[-10];
    piVar4[9] = piVar4[-9];
    piVar7[0x12] = *piVar7;
    piVar4[-0x14] = (int)DAT_006bc940;
    piVar4[-0x13] = (int)DAT_006bc944;
    piVar4[-0x12] = (int)DAT_006bc948;
    piVar4[-0x11] = (int)DAT_006bc94c;
    piVar4[-0xc] = (int)DAT_006bc940;
    piVar4[-0xb] = (int)DAT_006bc944;
    piVar4[-10] = (int)DAT_006bc948;
    piVar4[-9] = (int)DAT_006bc94c;
    *piVar7 = 0;
    piVar4 = piVar4 + 4;
    iVar5 = iVar5 + -1;
    piVar7 = piVar7 + 1;
  } while (iVar5 != 0);
  for (piVar4 = (int *)(**(code **)(*(int *)(param_1[2] + 0x70) + 0xc))(); piVar4 != (int *)0x0;
      piVar4 = (int *)(**(code **)(*piVar4 + 4))()) {
    piVar7 = (int *)(**(code **)(*piVar4 + 0xc))();
    (**(code **)(*piVar7 + 0x14))(param_3);
  }
  for (piVar4 = (int *)(**(code **)(*(int *)(param_1[3] + 0x70) + 0xc))(); piVar4 != (int *)0x0;
      piVar4 = (int *)(**(code **)(*piVar4 + 4))()) {
    piVar7 = (int *)(**(code **)(*piVar4 + 0xc))();
    (**(code **)(*piVar7 + 0x14))(param_3);
  }
  for (piVar4 = (int *)(**(code **)(param_1[0x262] + 0x10))(); piVar4 != (int *)0x0;
      piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
    piVar7 = (int *)(**(code **)(*piVar4 + 0x10))();
    if (piVar7[0x5c] == 2) {
      FUN_005a93b0(piVar7);
      (**(code **)(*piVar7 + 8))(param_3);
      pfVar9 = (float *)(piVar7 + 0xc);
      FUN_005a33b0(pfVar9);
      fVar2 = (float)param_1[0x233] * _DAT_0066c290;
      fVar1 = fVar2;
      if (fVar2 < afStack_50[2]) {
        fVar1 = afStack_50[2];
      }
      fVar3 = fVar2;
      if (fVar2 < afStack_50[1]) {
        fVar3 = afStack_50[1];
      }
      if (fVar2 < afStack_50[0]) {
        fVar2 = afStack_50[0];
      }
      *pfVar9 = fVar2;
      piVar7[0xf] = -0x40800000;
      piVar7[0xd] = (int)fVar3;
      piVar7[0xe] = (int)fVar1;
      afStack_50[2] = (float)piVar7[0xe];
      fStack_58 = (float)param_1[0x233] * _DAT_0066c294;
      afStack_50[0] = *pfVar9;
      afStack_50[1] = (float)piVar7[0xd];
      iStack_44 = piVar7[0xf];
      fVar1 = fStack_58;
      if (afStack_50[2] < fStack_58) {
        fVar1 = afStack_50[2];
      }
      fVar2 = fStack_58;
      if (afStack_50[1] < fStack_58) {
        fVar2 = afStack_50[1];
      }
      fVar3 = fStack_58;
      if (afStack_50[0] < fStack_58) {
        fVar3 = afStack_50[0];
      }
      *pfVar9 = fVar3;
      piVar7[0xd] = (int)fVar2;
      piVar7[0xf] = -0x40800000;
      piVar7[0xe] = (int)fVar1;
      FUN_005a9380(piVar7);
      iVar5 = piVar7[0x5a];
      fStack_18 = (float)piVar7[0xe];
      fVar1 = *pfVar9;
      fVar2 = (float)param_1[iVar5 * 4 + 0x1f6];
      param_1[iVar5 * 4 + 0x1f7] = (int)((float)piVar7[0xd] + (float)param_1[iVar5 * 4 + 0x1f7]);
      pfVar8 = (float *)(param_1 + iVar5 * 4 + 0x1f6);
      pfVar8[2] = fStack_18 + (float)param_1[iVar5 * 4 + 0x1f8];
      pfVar8[3] = -1.0;
      *pfVar8 = fVar1 + fVar2;
      fStack_8 = (float)piVar7[0x1d];
      fVar1 = (float)piVar7[0x1b];
      iVar5 = piVar7[0x5a];
      fVar2 = (float)param_1[iVar5 * 4 + 0x1fe];
      param_1[iVar5 * 4 + 0x1ff] = (int)((float)piVar7[0x1c] + (float)param_1[iVar5 * 4 + 0x1ff]);
      pfVar8 = (float *)(param_1 + iVar5 * 4 + 0x1fe);
      pfVar8[2] = fStack_8 + (float)param_1[iVar5 * 4 + 0x200];
      pfVar8[3] = -1.0;
      *pfVar8 = fVar1 + fVar2;
      piVar7[0x16] = (int)(param_3 + param_2);
      iStack_3c = piVar7[5];
      iStack_38 = piVar7[6];
      iStack_40 = piVar7[4];
      iStack_34 = piVar7[7];
      if (piVar7[0x54] != 0) {
        FUN_005c1320(piVar7[0x3b],1);
      }
      fStack_30 = *pfVar9;
      iStack_2c = piVar7[0xd];
      iStack_28 = piVar7[0xe];
      iStack_24 = piVar7[0xf];
      FUN_005c12a0(piVar7[0x3b],&fStack_30,&iStack_40);
    }
    else if ((piVar7[0x5c] == 7) && (iVar5 = FUN_005a6b40(piVar7,param_3), iVar5 == 1)) {
      piVar7[0x5c] = 8;
      piVar7[0x42] = 1;
      piVar10 = piVar7;
      FUN_0059ed70(piVar7);
      FUN_0059f4d0(piVar10);
      if (piVar7[0x49] == 0) {
        (**(code **)(*param_1 + 0x28))(0x2e,piVar7[0x3b],0,1);
      }
    }
    iVar5 = piVar7[0x5c];
    if (((((iVar5 == 10) || (iVar5 == 2)) || (iVar5 == 4)) || ((iVar5 == 6 || (iVar5 == 7)))) ||
       (iVar5 == 8)) {
      param_1[piVar7[0x5a] + 0x206] = param_1[piVar7[0x5a] + 0x206] + 1;
    }
  }
  pfVar9 = (float *)(param_1 + 0x206);
  pfVar8 = (float *)(param_1 + 0x1f8);
  iVar5 = 2;
  do {
    fVar1 = *pfVar9;
    if (fVar1 == 0.0) {
      pfVar8[-2] = DAT_006bc940;
      pfVar8[-1] = DAT_006bc944;
      *pfVar8 = DAT_006bc948;
      pfVar8[1] = DAT_006bc94c;
      pfVar8[6] = DAT_006bc940;
      pfVar8[7] = DAT_006bc944;
      pfVar8[8] = DAT_006bc948;
      pfVar8[9] = DAT_006bc94c;
    }
    else {
      afStack_50[1] = 0.0;
      if ((float)(uint)fVar1 != _DAT_0066c284) {
        fVar2 = _DAT_0066c298 / (float)(uint)fVar1;
        pfVar8[1] = -1.0;
        pfVar8[-2] = fVar2 * pfVar8[-2];
        pfVar8[-1] = fVar2 * pfVar8[-1];
        *pfVar8 = fVar2 * *pfVar8;
      }
      fStack_58 = *pfVar9;
      uStack_54 = 0;
      afStack_50[0] = fVar1;
      if ((float)(uint)fStack_58 != _DAT_0066c284) {
        fVar1 = _DAT_0066c298 / (float)(uint)fStack_58;
        pfVar8[9] = -1.0;
        pfVar8[6] = fVar1 * pfVar8[6];
        pfVar8[7] = fVar1 * pfVar8[7];
        pfVar8[8] = fVar1 * pfVar8[8];
      }
    }
    pfVar9 = pfVar9 + 1;
    pfVar8 = pfVar8 + 4;
    iVar5 = iVar5 + -1;
  } while (iVar5 != 0);
  FUN_005ceb50();
  FUN_005ceb50();
  if (param_1[0x206] != 0) {
    if (param_1[0x207] != 0) {
      return 0;
    }
    iVar5 = FUN_005a9330();
    if (iVar5 != 0) {
      FUN_005a9330();
      iVar5 = FUN_005ba420();
      if (iVar5 != 0) {
        return 0;
      }
    }
  }
  fStack_58 = 0.0;
  uStack_54 = 0;
  afStack_50[0] = 0.0;
  afStack_50[1] = 0.0;
  if (param_1[0x22d] != 4) {
    return 0;
  }
  param_1[0x22d] = 5;
  iVar5 = DAT_0066c244;
  param_1[0x280] = 0;
  param_1[0x39c] = iVar5;
  param_1[0x27d] = 0;
  param_1[0x27c] = 0;
  param_1[0x281] = 0;
  param_1[0x27f] = 0;
  param_1[0x27e] = 0;
  iVar5 = *(int *)(param_1[2] + 0x84);
  if (((iVar5 == 4) || (iVar5 == 3)) || (iVar5 == 5)) {
    param_1[0x27c] = 1;
  }
  iVar5 = *(int *)(param_1[3] + 0x84);
  if (((iVar5 == 4) || (iVar5 == 3)) || (iVar5 == 5)) {
    param_1[0x27e] = 1;
  }
  iVar5 = FUN_005a9330();
  if ((iVar5 != 0) && (iVar5 = FUN_005a9330(), *(int *)(iVar5 + 0x78) != 0)) {
    uStack_54 = 1;
  }
  for (piVar4 = (int *)(**(code **)(param_1[0x268] + 0x10))(); piVar4 != (int *)0x0;
      piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
    uVar6 = (**(code **)(*piVar4 + 0x10))();
    iVar5 = FUN_005ad870(uVar6);
    if (((iVar5 == 1) || (iVar5 = (**(code **)(*piVar4 + 0x10))(), *(int *)(iVar5 + 0x170) == 8)) ||
       (iVar5 = (**(code **)(*piVar4 + 0x10))(), *(int *)(iVar5 + 0x170) == 7)) {
      iVar5 = (**(code **)(*piVar4 + 0x10))();
      afStack_50[*(int *)(iVar5 + 0x168)] = 1.4013e-45;
    }
    iVar5 = (**(code **)(*piVar4 + 0x10))();
    if (*(int *)(iVar5 + 0x170) == 9) {
      iVar5 = (**(code **)(*piVar4 + 0x10))();
      (&fStack_58)[*(int *)(iVar5 + 0x168)] = 1.4013e-45;
    }
  }
  param_1[0x27c] = param_1[0x27c] & (uint)(afStack_50[0] == 0.0) & (uint)fStack_58;
  param_1[0x27e] = param_1[0x27e] & afStack_50[1] == 0.0 & uStack_54;
  if (param_1[0x207] == 0) {
    iVar5 = FUN_005a9330();
    if (iVar5 != 0) {
      FUN_005a9330();
      iVar5 = FUN_005ba420();
      if (iVar5 != 0) goto LAB_005aa630;
    }
    param_1[0x27d] = 1;
  }
LAB_005aa630:
  if (param_1[0x206] == 0) {
    param_1[0x27f] = 1;
  }
  if ((param_1[0x27f] != 0) && (param_1[0x27d] != 0)) {
    param_1[0x27f] = 0;
    param_1[0x27d] = 0;
  }
  if (param_1[0x27b] == 0) {
    if ((param_1[0x27d] == 0) || (param_1[0x27e] == 0)) {
      if (param_1[0x27c] == 0) {
        if (param_1[0x27d] == 0) {
          if (param_1[0x27f] != 0) {
            param_1[0x3a6] = 0x99;
          }
        }
        else {
          param_1[0x3a6] = 0x98;
        }
      }
      else {
        param_1[0x3a6] = 0x96;
      }
    }
    else {
      param_1[0x3a6] = 0x97;
    }
  }
  else if ((param_1[0x27f] == 0) || (param_1[0x27c] == 0)) {
    if (param_1[0x27e] == 0) {
      if (param_1[0x27f] == 0) {
        if (param_1[0x27d] != 0) {
          param_1[0x3a6] = 0x11b;
        }
      }
      else {
        param_1[0x3a6] = 0x11a;
      }
    }
    else {
      param_1[0x3a6] = 0x118;
    }
  }
  else {
    param_1[0x3a6] = 0x119;
  }
  iVar5 = param_1[0x3a6];
  if (iVar5 != 0x13d) {
    uVar6 = 1;
    FUN_005ba8b0(iVar5,1);
    FUN_005ba980(iVar5,uVar6);
  }
  (**(code **)(*param_1 + 0x8c))();
  return 0;
}

