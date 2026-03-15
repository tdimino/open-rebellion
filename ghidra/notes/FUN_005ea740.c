
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005ea740(int *param_1)

{
  float fVar1;
  int *piVar2;
  int *piVar3;
  float *pfVar4;
  int iVar5;
  uint uVar6;
  undefined4 *puVar7;
  float10 fVar8;
  float fStack_b8;
  float fStack_b4;
  float fStack_b0;
  float fStack_ac;
  int iStack_a8;
  int *piStack_a4;
  float fStack_a0;
  float fStack_9c;
  float fStack_98;
  float fStack_94;
  float fStack_90;
  float fStack_8c;
  uint uStack_88;
  undefined4 uStack_84;
  uint uStack_80;
  undefined4 uStack_7c;
  uint uStack_78;
  undefined4 uStack_74;
  int iStack_70;
  int iStack_6c;
  int iStack_68;
  int iStack_64;
  float fStack_60;
  float fStack_5c;
  float fStack_58;
  float fStack_50;
  float fStack_4c;
  float fStack_48;
  undefined4 *puStack_40;
  float fStack_3c;
  float fStack_38;
  float fStack_34;
  float fStack_2c;
  float fStack_28;
  float fStack_24;
  undefined1 auStack_1c [16];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655383;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  piVar2 = (int *)(**(code **)(*param_1 + 0x10))();
  if (piVar2 != (int *)0x0) {
    piStack_a4 = piVar2;
    (**(code **)(*piVar2 + 0x10))();
    param_1[0x41] = 0;
    param_1[0x42] = 0;
    param_1[0x43] = 0;
    param_1[0x44] = -0x40800000;
    param_1[0x49] = 0x7f7fffff;
    param_1[4] = 0x7f7fffff;
    iStack_a8 = 0;
    param_1[0x2a] = 0x42c60000;
    param_1[0x2b] = 0x42c60000;
    param_1[0x2c] = 0x42c60000;
    param_1[0x2d] = -0x40800000;
    do {
      piVar3 = (int *)(**(code **)(*piVar2 + 0x10))();
      fVar8 = (float10)(**(code **)(*piVar3 + 0x90))();
      if ((float10)_DAT_0066d874 < fVar8) {
        fVar8 = (float10)(**(code **)(*piVar3 + 0x90))();
        if (fVar8 < (float10)(float)param_1[0x49]) {
          fVar8 = (float10)(**(code **)(*piVar3 + 0x90))();
          param_1[0x49] = (int)(float)fVar8;
        }
        FUN_005a33b0(&fStack_98,piVar3 + 0x27);
        if (fStack_8c == _DAT_0066d878) {
          fStack_8c = SQRT(fStack_90 * fStack_90 + fStack_94 * fStack_94 + fStack_98 * fStack_98);
        }
        if (param_1[0x2d] == -0x40800000) {
          fStack_9c = (float)param_1[0x2c];
          fStack_a0 = (float)param_1[0x2b];
          param_1[0x2d] =
               (int)SQRT((float)param_1[0x2a] * (float)param_1[0x2a] +
                         fStack_9c * fStack_9c + fStack_a0 * fStack_a0);
        }
        if (fStack_8c < (float)param_1[0x2d]) {
          FUN_005a33b0(&iStack_70,piVar3 + 0x27);
          param_1[0x2a] = iStack_70;
          param_1[0x2b] = iStack_6c;
          param_1[0x2c] = iStack_68;
          param_1[0x2d] = iStack_64;
        }
        iStack_a8 = iStack_a8 + 1;
        pfVar4 = (float *)(**(code **)(*param_1 + 0x2c))(auStack_1c,piVar3);
        param_1[0x41] = (int)(*pfVar4 + (float)param_1[0x41]);
        param_1[0x42] = (int)(pfVar4[1] + (float)param_1[0x42]);
        fVar1 = pfVar4[2];
        param_1[0x44] = -0x40800000;
        param_1[0x43] = (int)(fVar1 + (float)param_1[0x43]);
        iVar5 = (**(code **)(*piVar3 + 0x20))();
        if (iVar5 != 0) goto LAB_005eacaa;
        if ((((piVar3[0x105] != 1) ||
             (FUN_005a33b0(&fStack_60,param_1 + 0x1e), fStack_60 != DAT_006bc940)) ||
            (fStack_5c != DAT_006bc944)) ||
           ((fStack_58 != DAT_006bc948 || ((uint)param_1[0x4d] < 5)))) {
          if (piVar3[0x105] == 0) {
            iVar5 = FUN_005aaf90();
            uStack_88 = *(uint *)(iVar5 + 0x87c);
            uStack_84 = 0;
            if (_DAT_0066d858 < (float)uStack_88 - (float)piVar3[0x104]) {
              iVar5 = FUN_005c61d0(param_1 + 0x3b,piVar3[0x3b]);
              if (iVar5 != 0) {
                FUN_005c6200(param_1 + 0x3b,piVar3[0x3b]);
              }
            }
          }
          goto LAB_005eacaa;
        }
        iVar5 = FUN_005aaf90();
        uStack_80 = *(uint *)(iVar5 + 0x87c);
        uStack_7c = 0;
        if ((float)uStack_80 - (float)piVar3[0x104] <= _DAT_0066d858) goto LAB_005eacaa;
        iVar5 = FUN_005c61d0(param_1 + 0x38,piVar3[0x3b]);
        FUN_005a33b0(&fStack_3c,piVar3 + 0xc);
        FUN_005a33b0(&fStack_2c,(undefined4 *)(iVar5 + 0x50));
        FUN_005b5090(&fStack_50,(float)param_1[0xf] - fStack_2c,(float)param_1[0x10] - fStack_28,
                     (float)param_1[0x11] - fStack_24);
        FUN_005b5090(&fStack_b8,fStack_50 - fStack_3c,fStack_4c - fStack_38,fStack_48 - fStack_34);
        if (fStack_ac == _DAT_0066d878) {
          fVar1 = fStack_b8 * fStack_b8 + fStack_b0 * fStack_b0 + fStack_b4 * fStack_b4;
        }
        else {
          fVar1 = fStack_ac * fStack_ac;
        }
        if (fVar1 < _DAT_0066d854) goto LAB_005eacaa;
        iVar5 = FUN_005aaf90();
        uStack_78 = *(uint *)(iVar5 + 0x87c);
        uStack_74 = 0;
        piVar2 = param_1 + 0x3b;
        piVar3[0x104] = (int)(float)uStack_78;
        iVar5 = FUN_005c61d0(piVar2,piVar3[0x3b]);
        if (iVar5 != 0) {
          FUN_005c6200(piVar2,piVar3[0x3b]);
        }
        uVar6 = piVar3[0x3b] + 1 + piVar3[0x103] & 3;
        piVar3[0x103] = uVar6;
        switch(uVar6) {
        case 0:
          fStack_b8 = 20.0;
          fStack_b4 = 0.0;
          fStack_b0 = 0.0;
          break;
        case 1:
          fStack_b8 = -20.0;
          fStack_b4 = 10.0;
          fStack_b0 = 5.0;
          break;
        case 2:
          fStack_b8 = 5.0;
          fStack_b4 = 0.0;
          fStack_b0 = 20.0;
          break;
        case 3:
          fStack_b8 = 0.0;
          fStack_b4 = -10.0;
          fStack_b0 = -20.0;
          break;
        default:
          goto switchD_005eab4e_default;
        }
        fStack_ac = -1.0;
switchD_005eab4e_default:
        puVar7 = (undefined4 *)FUN_00618b70(0xf4);
        uStack_4 = 0;
        puStack_40 = puVar7;
        if (puVar7 == (undefined4 *)0x0) {
          puVar7 = (undefined4 *)0x0;
        }
        else {
          FUN_005e9cc0(puVar7,(int)piVar3);
          uStack_4 = CONCAT31(uStack_4._1_3_,1);
          FUN_005ebc90(puVar7 + 8);
          *puVar7 = &PTR_FUN_0066d968;
          puVar7[8] = &PTR_LAB_0066d948;
        }
        puVar7[0x14] = fStack_b8;
        puVar7[0x15] = fStack_b4;
        puVar7[0x16] = fStack_b0;
        uStack_4 = 0xffffffff;
        puVar7[0x17] = fStack_ac;
        FUN_005c61a0(piVar2,puVar7);
        piVar2 = piStack_a4;
      }
LAB_005eacaa:
      piVar2 = (int *)(**(code **)(*piVar2 + 8))();
      piStack_a4 = piVar2;
    } while (piVar2 != (int *)0x0);
    if (param_1[0x49] == 0x7f7fffff) {
      param_1[0x49] = 0;
    }
    param_1[0x4b] = iStack_a8;
    if ((((float)param_1[0xf] == (float)param_1[0x32]) &&
        ((float)param_1[0x10] == (float)param_1[0x33])) &&
       ((float)param_1[0x11] == (float)param_1[0x34])) {
      param_1[0x4d] = param_1[0x4d] + 1;
    }
    else {
      param_1[0x4d] = 0;
    }
    param_1[0x32] = param_1[0xf];
    param_1[0x33] = param_1[0x10];
    param_1[0x34] = param_1[0x11];
    param_1[0x35] = param_1[0x12];
  }
  ExceptionList = pvStack_c;
  return;
}

