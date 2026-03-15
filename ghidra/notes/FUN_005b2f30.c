
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005b2f30(int *param_1)

{
  undefined4 *puVar1;
  float fVar2;
  undefined1 *puVar3;
  int iVar4;
  int unaff_EBP;
  int iVar5;
  float10 fVar6;
  float unaff_retaddr;
  float fVar7;
  undefined4 *puStack_c0;
  float fStack_bc;
  float fStack_b8;
  int *piStack_b4;
  undefined4 *puStack_b0;
  float fStack_a8;
  float fStack_a4;
  float fStack_a0;
  float fStack_9c;
  float fStack_98;
  float fStack_94;
  float fStack_90;
  undefined4 uStack_8c;
  float fStack_84;
  float fStack_80;
  float fStack_7c;
  float fStack_78;
  undefined1 auStack_74 [4];
  float fStack_70;
  float fStack_68;
  float fStack_64;
  undefined **ppuStack_60;
  undefined4 auStack_5c [3];
  undefined1 auStack_50 [4];
  float fStack_4c;
  undefined1 auStack_40 [4];
  undefined1 auStack_3c [16];
  undefined1 auStack_2c [8];
  undefined1 auStack_24 [24];
  void *pvStack_c;
  undefined1 *puStack_8;
  float fStack_4;
  
  fStack_4 = -NAN;
  puStack_8 = &LAB_00653346;
  pvStack_c = ExceptionList;
  iVar5 = 0;
  ExceptionList = &pvStack_c;
  param_1[0x105] = 0;
  fVar6 = (float10)(**(code **)(*param_1 + 0x90))();
  fStack_b8 = (float)fVar6;
  if (fVar6 == (float10)_DAT_0066c4f0) {
    ExceptionList = pvStack_c;
    return;
  }
  fStack_84 = 0.0;
  puStack_c0 = (undefined4 *)0x0;
  fStack_a8 = 0.0;
  piStack_b4 = (int *)0x0;
  param_1[0x101] = -0x800001;
  param_1[0x102] = 0x7f7fffff;
  FUN_005a33b0(auStack_2c,param_1 + 0x2f);
  puStack_b0 = (undefined4 *)0x0;
  param_1[0x2f] = param_1[0xc];
  param_1[0x30] = param_1[0xd];
  param_1[0x31] = param_1[0xe];
  param_1[0x32] = param_1[0xf];
  (**(code **)(*param_1 + 0x7c))(auStack_3c);
  FUN_005f5830(&ppuStack_60);
  ppuStack_60 = &PTR_FUN_0066c668;
  puStack_8 = (undefined1 *)0x0;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puStack_c0 = (undefined4 *)FUN_00618b70(0x48);
    puStack_8._0_1_ = 1;
    if (puStack_c0 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puStack_c0);
    }
    puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
  }
  iVar4 = FUN_005a6480(DAT_006bc470,param_1[0x5d]);
  if ((iVar4 != 0) &&
     (fVar6 = (float10)(**(code **)(**(int **)(iVar4 + 0x54) + 0x34))(), fVar6 <= (float10)fStack_bc
     )) {
    fVar6 = (float10)(**(code **)(**(int **)(iVar4 + 0x54) + 0x34))();
    fStack_bc = (float)fVar6;
  }
  iVar4 = FUN_005b39e0(auStack_40,&DAT_006bc940);
  if (iVar4 == 0) {
    iVar4 = FUN_005aaf90();
    piStack_b4 = FUN_005cdbf0((void *)(iVar4 + 0x8f8),param_1,
                              fStack_bc * unaff_retaddr * _DAT_0066c4fc);
    if (piStack_b4 != (int *)0x0) {
      iVar4 = (**(code **)(*piStack_b4 + 0xc))();
      while (iVar4 != 0) {
        iVar5 = *(int *)(iVar4 + 0x10);
        puVar1 = (undefined4 *)(iVar5 + 0x30);
        FUN_005a33b0(&fStack_a8,puVar1);
        FUN_005a33b0(&fStack_84,param_1 + 0xc);
        fStack_a4 = fStack_a4 - fStack_80;
        fStack_a8 = fStack_a8 - fStack_84;
        fStack_9c = -1.0;
        fStack_a0 = fStack_a0 - fStack_7c;
        if (fStack_a4 <= _DAT_0066c4f0) {
          if (_DAT_0066c4f0 <= fStack_a4) {
            FUN_005d25b0(&ppuStack_60,iVar5);
          }
          else {
            fStack_b8 = 1.4013e-45;
            FUN_005a33b0(auStack_74,puVar1);
            fVar7 = (float)param_1[2] + *(float *)(iVar5 + 8) + fStack_70;
            if (fVar7 < (float)param_1[0x101]) {
              fVar7 = (float)param_1[0x101];
            }
            param_1[0x101] = (int)fVar7;
          }
        }
        else {
          unaff_EBP = 1;
          FUN_005a33b0(auStack_50,puVar1);
          fVar7 = (fStack_4c - *(float *)(iVar5 + 8)) - (float)param_1[2];
          if ((float)param_1[0x102] < fVar7) {
            fVar7 = (float)param_1[0x102];
          }
          param_1[0x102] = (int)fVar7;
          if (fVar7 < (float)param_1[0xd]) {
            puStack_b0 = (undefined4 *)0x1;
          }
        }
        iVar4 = FUN_0041c210(iVar4);
        iVar5 = unaff_EBP;
      }
    }
    unaff_EBP = FUN_0041c210((int)&ppuStack_60);
    if (unaff_EBP != 0) {
      do {
        iVar4 = *(int *)(unaff_EBP + 0x10);
        if ((uint)param_1[0x3b] < *(uint *)(iVar4 + 0xec)) {
          if (fStack_b8 == 0.0) {
            iVar5 = 1;
            FUN_005a33b0(auStack_74,(undefined4 *)(iVar4 + 0x30));
            fVar7 = (fStack_70 - *(float *)(iVar4 + 8)) - (float)param_1[2];
            if (fVar7 <= (float)param_1[0x102]) goto LAB_005b3335;
            fVar7 = (float)param_1[0x102];
            param_1[0x102] = (int)fVar7;
            if ((float)param_1[0xd] <= fVar7) goto LAB_005b3408;
LAB_005b3400:
            puStack_b0 = (undefined4 *)0x1;
          }
          else {
            if (iVar5 == 0) {
              fStack_b8 = 1.4013e-45;
              FUN_005a33b0(auStack_50,(undefined4 *)(iVar4 + 0x30));
              fVar7 = (float)param_1[2] + *(float *)(iVar4 + 8) + fStack_4c;
              if (fVar7 < (float)param_1[0x101]) goto LAB_005b33a5;
              goto LAB_005b33ad;
            }
LAB_005b3352:
            fStack_64 = (float)param_1[0x101];
            if ((float)param_1[0xd] - fStack_64 <= (float)param_1[0x102] - (float)param_1[0xd]) {
              puVar1 = (undefined4 *)param_1[0x101];
              fVar7 = (float)param_1[2] + *(float *)(iVar4 + 8) + *(float *)(iVar4 + 0x34);
              puStack_c0 = puVar1;
              goto LAB_005b339c;
            }
            fVar7 = (*(float *)(iVar4 + 0x34) - *(float *)(iVar4 + 8)) - (float)param_1[2];
            if ((float)param_1[0x102] < fVar7) {
              fVar7 = (float)param_1[0x102];
            }
            param_1[0x102] = (int)fVar7;
            if (fVar7 < (float)param_1[0xd]) goto LAB_005b3400;
          }
        }
        else if (iVar5 == 0) {
          fStack_b8 = 1.4013e-45;
          FUN_005a33b0(&fStack_84,(undefined4 *)(iVar4 + 0x30));
          puVar1 = (undefined4 *)param_1[0x101];
          fVar7 = (float)param_1[2] + *(float *)(iVar4 + 8) + fStack_80;
LAB_005b339c:
          if (fVar7 < (float)puVar1) {
LAB_005b33a5:
            fVar7 = (float)param_1[0x101];
          }
LAB_005b33ad:
          param_1[0x101] = (int)fVar7;
        }
        else {
          if (fStack_b8 != 0.0) goto LAB_005b3352;
          fVar7 = (*(float *)(iVar4 + 0x34) - *(float *)(iVar4 + 8)) - (float)param_1[2];
          if ((float)param_1[0x102] < fVar7) {
            fVar7 = (float)param_1[0x102];
          }
LAB_005b3335:
          iVar5 = 1;
          param_1[0x102] = (int)fVar7;
          if (fVar7 < (float)param_1[0xd]) goto LAB_005b3400;
        }
LAB_005b3408:
        unaff_EBP = FUN_0041c210(unaff_EBP);
      } while (unaff_EBP != 0);
      unaff_EBP = 0;
    }
  }
  (**(code **)(*param_1 + 0x7c))(&fStack_98);
  fVar7 = (float)puStack_c0 * _DAT_0066c4c4;
  fStack_84 = (float)param_1[0xd];
  if ((piStack_b4 == (int *)0x0) || (unaff_EBP == 0)) {
    if (piStack_b4 == (int *)0x0) {
      if (unaff_EBP == 0) goto LAB_005b360f;
      fVar2 = (float)param_1[0xd] - (float)param_1[0x101];
      if (_DAT_0066c4f0 < fVar2) {
LAB_005b35e9:
        fVar7 = fVar7 * fStack_4 * _DAT_0066c4fc;
        if (fVar7 <= fVar2) {
          fVar2 = fVar7;
        }
      }
      else {
        if (fVar2 < _DAT_0066c4f0) {
          fVar2 = -fVar2;
          goto LAB_005b35e9;
        }
        unaff_EBP = 0;
      }
      fVar2 = (float)param_1[0xd] + fVar2;
    }
    else {
      fVar2 = (float)param_1[0x102] - (float)param_1[0xd];
      if (fVar2 <= _DAT_0066c4f0) {
        if (fVar2 < _DAT_0066c4f0) {
          fVar2 = -fVar2;
          goto LAB_005b3568;
        }
        piStack_b4 = (int *)0x0;
      }
      else {
LAB_005b3568:
        fVar7 = fVar7 * fStack_4 * _DAT_0066c4fc;
        if (fVar7 <= fVar2) {
          fVar2 = (float)param_1[0xd] - fVar7;
          goto LAB_005b3607;
        }
      }
LAB_005b3589:
      fVar2 = (float)param_1[0xd] - fVar2;
    }
  }
  else {
    if ((float)param_1[0x102] - (float)param_1[0x101] < (float)param_1[2] + (float)param_1[2]) {
      puStack_b0 = (undefined4 *)0x1;
      goto LAB_005b360f;
    }
    fVar2 = ((float)param_1[0x102] - (float)param_1[0x101]) * _DAT_0066c4f8 - (float)param_1[0xd];
    if (_DAT_0066c4f0 < fVar2) goto LAB_005b35e9;
    if (_DAT_0066c4f0 <= fVar2) goto LAB_005b360f;
    fVar2 = -fVar2;
    fVar7 = fVar7 * fStack_4 * _DAT_0066c4fc;
    if (fVar2 < fVar7) goto LAB_005b3589;
    fVar2 = (float)param_1[0xd] - fVar7;
  }
LAB_005b3607:
  param_1[0xf] = -0x40800000;
  param_1[0xd] = (int)fVar2;
LAB_005b360f:
  if (((fStack_98 != _DAT_0066c4f0) && (unaff_EBP == 0)) && (piStack_b4 == (int *)0x0)) {
    if (fStack_98 <= _DAT_0066c4f0) {
      fVar2 = (float)puStack_c0 * _DAT_0066c4c4 * fStack_4 * _DAT_0066c4fc;
      fVar7 = -fStack_98;
      if (fVar2 <= -fStack_98) {
        fVar7 = fVar2;
      }
      if ((fStack_bc == 1.4013e-45) && ((float)param_1[0xd] - (float)param_1[0x101] <= fVar7)) {
        fVar7 = (float)param_1[0xd] - (float)param_1[0x101];
      }
      fVar2 = (float)param_1[0xd] - fVar7;
    }
    else {
      fVar2 = (float)puStack_c0 * _DAT_0066c4c4 * fStack_4 * _DAT_0066c4fc;
      fVar7 = fStack_98;
      if (fVar2 <= fStack_98) {
        fVar7 = fVar2;
      }
      if ((iVar5 == 1) && ((float)param_1[0x102] - (float)param_1[0xd] <= fVar7)) {
        fVar7 = (float)param_1[0x102] - (float)param_1[0xd];
      }
      fVar2 = (float)param_1[0xd] + fVar7;
    }
    param_1[0xf] = -0x40800000;
    param_1[0xd] = (int)fVar2;
    if (fVar7 == _DAT_0066c4f0) {
      puStack_b0 = (undefined4 *)0x1;
    }
  }
  if (((float)param_1[0xd] != fStack_84) &&
     (puStack_b0 = (undefined4 *)FUN_005b2e60(fStack_b8), puStack_b0 != (undefined4 *)0x0)) {
    param_1[0xf] = -0x40800000;
    param_1[0xd] = (int)fStack_84;
  }
  fStack_78 = (float)param_1[4];
  fVar7 = (float)param_1[8] * fStack_78 + (float)param_1[10] * (float)param_1[6];
  fStack_a8 = (float)param_1[0xd];
  fStack_a4 = (float)param_1[0xe];
  fStack_a0 = (float)param_1[0xf];
  fStack_98 = _DAT_0066c4f0;
  fStack_90 = _DAT_0066c500;
  if (fVar7 < _DAT_0066c4f0) {
    fVar7 = 0.0;
  }
  if ((((fStack_9c != DAT_006bc940) || (_DAT_0066c4f0 != DAT_006bc944)) ||
      (fStack_94 != DAT_006bc948)) &&
     ((fStack_90 = SQRT(fStack_9c * fStack_9c + fStack_94 * fStack_94),
      fStack_90 <= (float)puStack_c0 * fStack_4 * _DAT_0066c4fc && (_DAT_0066c508 <= fVar7)))) {
    param_1[0xf] = -0x40800000;
    param_1[0xd] = param_1[0xd];
    param_1[0xc] = (int)((float)param_1[0xc] + fStack_9c);
    uStack_8c = 1;
    param_1[0xe] = (int)(fStack_94 + (float)param_1[0xe]);
    (**(code **)(*param_1 + 0x7c))(&fStack_78);
  }
  (**(code **)(*param_1 + 0x80))(auStack_24);
  fVar6 = (float10)FUN_005cd2d0((int)param_1);
  puVar3 = puStack_8;
  (**(code **)(*param_1 + 4))((float)fVar6,puStack_8);
  fStack_68 = (float)param_1[0x19];
  if (((float)puStack_c0 != _DAT_0066c4f0) &&
     ((((float)param_1[0x17] != DAT_006bc940 || (_DAT_0066c4f0 != DAT_006bc944)) ||
      (fStack_68 != DAT_006bc948)))) {
    param_1[0x1c] = 0;
    param_1[0x1e] = -0x40800000;
    fStack_84 = 1.4013e-45;
    fStack_80 = (float)param_1[4] * fStack_b8 * (float)puStack_c0;
    fStack_78 = (float)param_1[6] * fStack_b8 * (float)puStack_c0;
    param_1[0x1b] = (int)fStack_80;
    param_1[0x1d] = (int)fStack_78;
    FUN_005cd640(param_1,(float)puVar3);
  }
  if ((fStack_84 == 1.4013e-45) && (iVar5 = FUN_005b2e60(puStack_b0), iVar5 != 0)) {
    param_1[0xc] = (int)fStack_a4;
    param_1[0xd] = (int)fStack_a0;
    param_1[0xe] = (int)fStack_9c;
    param_1[0xf] = (int)fStack_98;
  }
  param_1[0x105] = (int)fStack_a8;
  if (puStack_b0 != (undefined4 *)0x0) {
    (**(code **)*puStack_b0)(1);
  }
  fStack_4 = -NAN;
  FUN_005d2640(auStack_5c);
  ExceptionList = pvStack_c;
  return;
}

