
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b5b60(int *param_1,float param_2,undefined4 param_3,uint param_4)

{
  float fVar1;
  bool bVar2;
  int *piVar3;
  int iVar4;
  void *pvVar5;
  float *pfVar6;
  float *pfVar7;
  float10 fVar8;
  uint uVar9;
  int iVar10;
  float fStack_228;
  uint uStack_21c;
  float *pfStack_218;
  undefined4 *puStack_214;
  undefined4 uStack_210;
  float afStack_20c [128];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00653457;
  pvStack_c = ExceptionList;
  if (_DAT_0066c4f0 < (float)param_1[0xf3]) {
    ExceptionList = &pvStack_c;
    fVar8 = (float10)(**(code **)(*param_1 + 0xbc))();
    fStack_228 = (float)fVar8;
    if (DAT_006bc470 == (undefined4 *)0x0) {
      puStack_214 = (undefined4 *)FUN_00618b70(0x48);
      uStack_4 = 0;
      if (puStack_214 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(puStack_214);
      }
      uStack_4 = 0xffffffff;
    }
    piVar3 = (int *)FUN_005a6340(DAT_006bc470,param_4);
    if (piVar3 != (int *)0x0) {
      fVar8 = (float10)(**(code **)(*piVar3 + 0xbc))();
      fStack_228 = (float)((float10)fStack_228 / fVar8);
      if (fStack_228 < _DAT_0066c518) {
        fStack_228 = 0.1;
      }
    }
    iVar4 = (**(code **)(*param_1 + 0x20))();
    if (iVar4 == 1) {
      param_2 = param_2 / fStack_228;
    }
    fVar1 = (float)param_1[0xf0];
    if (param_2 < _DAT_0066c514) {
      param_2 = _DAT_0066c514;
    }
    param_2 = (float)param_1[0xf0] - param_2;
    bVar2 = _DAT_0066c4f0 <= param_2;
    param_1[0xf0] = (int)param_2;
    if (bVar2) {
      iVar4 = 1;
      uVar9 = 0xe;
      pvVar5 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar5,uVar9,iVar4);
      if ((param_1[0x49] == 0) && (iVar4 = (**(code **)(*param_1 + 0x20))(), iVar4 == 0)) {
        fStack_228 = (_DAT_0066c51c / fVar1) * (fVar1 - (float)param_1[0xf0]);
        if (fStack_228 < _DAT_0066c514) {
          fStack_228 = 1.0;
        }
        iVar10 = 100;
        iVar4 = 0;
        FUN_005aaf90();
        uVar9 = FUN_005a8a70(iVar4,iVar10);
        if ((float)uVar9 <= fStack_228) {
          piVar3 = (int *)FUN_005aaf90();
          (**(code **)(*piVar3 + 0x28))(0x1e,param_3,0);
        }
      }
      uVar9 = param_1[0x3b];
      iVar4 = (**(code **)(*param_1 + 0xb4))();
      FUN_005c13d0(uVar9,2,iVar4);
    }
    else {
      fStack_228 = -param_2;
      iVar4 = 1;
      uVar9 = 0x13;
      param_1[0xf0] = 0;
      pvVar5 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar5,uVar9,iVar4);
      uVar9 = param_1[0x3b];
      iVar4 = (**(code **)(*param_1 + 0xb0))();
      FUN_005c13d0(uVar9,2,iVar4);
      if ((param_1[0x49] == 0) && (iVar4 = (**(code **)(*param_1 + 0x20))(), iVar4 != 1)) {
        afStack_20c[2] = 0.0;
        afStack_20c[4] = 0.0;
        afStack_20c[3] = 0.0;
        afStack_20c[5] = 0.0;
        afStack_20c[6] = 0.0;
        afStack_20c[7] = 0.0;
        if (_DAT_0066c4f0 < fStack_228) {
          do {
            iVar10 = 10;
            iVar4 = 1;
            FUN_005aaf90();
            iVar4 = FUN_005a8a70(iVar4,iVar10);
            if (iVar4 == 1) {
              iVar10 = 0x32;
              iVar4 = 0x1e;
              FUN_005aaf90();
              puStack_214 = (undefined4 *)FUN_005a8a70(iVar4,iVar10);
              uStack_210 = 0;
              afStack_20c[2] = (float)puStack_214 + afStack_20c[2];
            }
            else if (iVar4 == 2) {
              iVar10 = 0x32;
              iVar4 = 0x1e;
              FUN_005aaf90();
              uVar9 = FUN_005a8a70(iVar4,iVar10);
              afStack_20c[3] = (float)uVar9 + afStack_20c[3];
            }
            else {
              afStack_20c[(iVar4 - 3U >> 1) + 4] = 1.4013e-45;
            }
            fStack_228 = fStack_228 - _DAT_0066c514;
          } while (_DAT_0066c4f0 < fStack_228);
        }
        uStack_21c = 0;
        pfStack_218 = (float *)FUN_00618b70(0x20);
        if ((pfStack_218 != (float *)0x0) && (&stack0x00000000 != (undefined1 *)0x20c)) {
          pfVar6 = afStack_20c;
          pfVar7 = pfStack_218;
          for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
            *pfVar7 = *pfVar6;
            pfVar6 = pfVar6 + 1;
            pfVar7 = pfVar7 + 1;
          }
          uStack_21c = 0x20;
        }
        uStack_4 = 1;
        pvVar5 = (void *)FUN_00618b70(0x220);
        uStack_4._0_1_ = 2;
        puStack_214 = pvVar5;
        if (pvVar5 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          iVar4 = FUN_005aaf90();
          pvVar5 = FUN_005ce500(pvVar5,(float)*(uint *)(iVar4 + 0x87c),0x16,param_3,param_4,
                                &uStack_21c);
        }
        uStack_4 = CONCAT31(uStack_4._1_3_,1);
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x2c))(pvVar5);
        uStack_4 = 0xffffffff;
        if (pfStack_218 != (float *)0x0) {
          FUN_00618b60((undefined *)pfStack_218);
        }
      }
    }
  }
  else {
    param_1[0xf3] = 0;
  }
  ExceptionList = pvStack_c;
  return;
}

