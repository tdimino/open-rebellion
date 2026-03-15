
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

char __fastcall FUN_005b3f10(int *param_1)

{
  uint uVar1;
  int iVar2;
  void *pvVar3;
  int iVar4;
  int *piVar5;
  int iVar6;
  int *piVar7;
  uint uVar8;
  float *pfVar9;
  void *unaff_EBP;
  float10 fVar10;
  undefined4 uVar11;
  float fVar12;
  float fVar13;
  void *local_2ac;
  float local_29c;
  uint local_298;
  uint uStack_294;
  float *pfStack_290;
  uint local_28c;
  float *local_288;
  uint uStack_284;
  float *pfStack_280;
  uint uStack_27c;
  float *pfStack_278;
  int local_274;
  void *pvStack_270;
  uint local_26c;
  undefined4 uStack_268;
  uint uStack_264;
  undefined4 uStack_260;
  uint uStack_25c;
  undefined4 uStack_258;
  uint uStack_254;
  undefined4 uStack_250;
  uint uStack_244;
  undefined4 uStack_240;
  uint uStack_23c;
  undefined4 uStack_238;
  uint local_234;
  undefined4 uStack_230;
  uint uStack_22c;
  undefined4 uStack_228;
  uint uStack_224;
  undefined4 uStack_220;
  uint uStack_214;
  undefined4 uStack_210;
  float local_20c;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006533ec;
  local_c = ExceptionList;
  local_29c = 0.0;
  local_298 = 4;
  local_274 = 0;
  ExceptionList = &local_c;
  do {
    uVar8 = 0;
    pfVar9 = (float *)(param_1 + 0x62);
    do {
      fVar13 = pfVar9[2] + pfVar9[1] + *pfVar9;
      pfVar9[0x19] = fVar13;
      if (local_29c < fVar13) {
        local_274 = 1;
        local_29c = fVar13;
        local_298 = uVar8;
      }
      uVar1 = local_298;
      uVar8 = uVar8 + 1;
      pfVar9 = pfVar9 + 0x1d;
    } while (uVar8 < 4);
    if (local_298 == 4) {
      ExceptionList = local_c;
      return '\x03';
    }
    if (_DAT_0066c4f0 < local_29c) {
      local_29c = 0.0;
      if ((float)param_1[local_298 * 0x1d + 100] != _DAT_0066c4f0) {
        iVar4 = param_1[(local_298 + 4) * 0x1d];
        FUN_005aaf90();
        iVar2 = FUN_005ad870();
        if ((iVar2 != 0) && (param_1[0x5c] != 6)) {
          local_20c = (float)param_1[uVar1 * 0x1d + 100];
          local_28c = 0;
          local_288 = (float *)FUN_00618b70(4);
          fVar13 = local_20c;
          if ((local_288 != (float *)0x0) && (&stack0x00000000 != (undefined1 *)0x20c)) {
            *local_288 = local_20c;
            local_28c = 4;
          }
          local_4 = 0;
          iVar2 = FUN_005aaf90();
          local_26c = *(uint *)(iVar2 + 0x87c);
          uStack_268 = 0;
          iVar2 = param_1[0x3b];
          fVar12 = (float)local_26c - _DAT_0066c4d0 * _DAT_0066c510;
          uVar11 = *(undefined4 *)(iVar4 + 0xec);
          iVar6 = param_1[0x5a];
          pvVar3 = (void *)FUN_005aaf90();
          FUN_005a6ac0(pvVar3,iVar6,iVar2,uVar11,fVar12,fVar13);
          pvVar3 = (void *)FUN_00618b70(0x220);
          local_4._0_1_ = 1;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            iVar2 = param_1[0x3b];
            uVar11 = *(undefined4 *)(iVar4 + 0xec);
            iVar4 = FUN_005aaf90();
            local_234 = *(uint *)(iVar4 + 0x87c);
            uStack_230 = 0;
            pvVar3 = FUN_005ce460(pvVar3,(float)local_234 + _DAT_0066c4d0,0xf,uVar11,iVar2,
                                  &local_28c);
          }
          local_4 = (uint)local_4._1_3_ << 8;
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x2c))(pvVar3);
          pvVar3 = (void *)FUN_00618b70(0x220);
          local_c._0_1_ = 2;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            iVar4 = param_1[0x3b];
            uVar11 = *(undefined4 *)((int)unaff_EBP + 0xec);
            iVar2 = FUN_005aaf90();
            uStack_25c = *(uint *)(iVar2 + 0x87c);
            uStack_258 = 0;
            pvVar3 = FUN_005ce460(pvVar3,(float)uStack_25c - _DAT_0066c4d0 * _DAT_0066c510,0x12,
                                  uVar11,iVar4,&uStack_294);
          }
          local_c = (void *)((uint)local_c._1_3_ << 8);
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x2c))(pvVar3,1);
          param_1[uVar1 * 0x1d + 0x79] =
               (int)((float)param_1[uVar1 * 0x1d + 0x79] - (float)param_1[uVar1 * 0x1d + 0x60]);
          param_1[uVar1 * 0x1d + 0x78] = 0;
          FUN_005b64e0(param_1,local_298);
          local_4 = 0xffffffff;
          if (local_288 != (float *)0x0) {
            FUN_00618b60((undefined *)local_288);
          }
          local_288 = (float *)0x0;
        }
        param_1[uVar1 * 0x1d + 100] = 0;
      }
      if ((float)param_1[uVar1 * 0x1d + 0x62] != _DAT_0066c4f0) {
        piVar5 = (int *)param_1[uVar1 * 0x1d + 0x75];
        FUN_005aaf90();
        iVar4 = FUN_005ad870();
        if ((iVar4 != 0) && (param_1[0x5c] != 6)) {
          local_20c = (float)param_1[uVar1 * 0x1d + 0x62];
          uStack_294 = 0;
          pfStack_290 = (float *)FUN_00618b70(4);
          if ((pfStack_290 != (float *)0x0) && (&stack0x00000000 != (undefined1 *)0x20c)) {
            *pfStack_290 = local_20c;
            uStack_294 = 4;
          }
          local_4 = 3;
          iVar4 = FUN_005aaf90();
          uStack_214 = *(uint *)(iVar4 + 0x87c);
          uStack_210 = 0;
          iVar4 = param_1[0x3b];
          iVar2 = param_1[0x5a];
          iVar6 = piVar5[0x3b];
          fVar13 = (float)uStack_214 + _DAT_0066c4d0;
          fVar12 = local_20c;
          pvVar3 = (void *)FUN_005aaf90();
          FUN_005a6ac0(pvVar3,iVar2,iVar4,iVar6,fVar13,fVar12);
          pvVar3 = (void *)FUN_00618b70(0x220);
          local_4._0_1_ = 4;
          if (pvVar3 == (void *)0x0) {
            local_2ac = (void *)0x0;
          }
          else {
            iVar4 = param_1[0x3b];
            iVar2 = piVar5[0x3b];
            iVar6 = FUN_005aaf90();
            uStack_244 = *(uint *)(iVar6 + 0x87c);
            uStack_240 = 0;
            local_2ac = FUN_005ce460(pvVar3,(float)uStack_244,0xd,iVar2,iVar4,&uStack_294);
          }
          local_4 = CONCAT31(local_4._1_3_,3);
          piVar7 = (int *)FUN_005aaf90();
          (**(code **)(*piVar7 + 0x2c))(local_2ac);
          pvVar3 = (void *)FUN_00618b70(0x220);
          local_c._0_1_ = 5;
          if (pvVar3 == (void *)0x0) {
            unaff_EBP = (void *)0x0;
          }
          else {
            iVar4 = param_1[0x3b];
            iVar2 = piVar5[0x3b];
            iVar6 = FUN_005aaf90();
            uStack_22c = *(uint *)(iVar6 + 0x87c);
            uStack_228 = 0;
            unaff_EBP = FUN_005ce460(pvVar3,(float)uStack_22c + _DAT_0066c4d0,0x11,iVar2,iVar4,
                                     (uint *)&local_29c);
          }
          local_c = (void *)CONCAT31(local_c._1_3_,3);
          piVar7 = (int *)FUN_005aaf90();
          (**(code **)(*piVar7 + 0x2c))(unaff_EBP,1);
          iVar4 = (**(code **)(*param_1 + 0x20))();
          if ((((iVar4 == 1) && (iVar4 = (**(code **)(*piVar5 + 0x20))(), iVar4 == 0)) &&
              (fVar10 = FUN_005b1650((int)piVar5), fVar10 == (float10)_DAT_0066c4f0)) &&
             (SQRT(((float)piVar5[0xc] - (float)param_1[0xc]) *
                   ((float)piVar5[0xc] - (float)param_1[0xc]) +
                   ((float)piVar5[0xe] - (float)param_1[0xe]) *
                   ((float)piVar5[0xe] - (float)param_1[0xe]) +
                   ((float)piVar5[0xd] - (float)param_1[0xd]) *
                   ((float)piVar5[0xd] - (float)param_1[0xd])) <= (float)param_1[0x4e])) {
            fVar10 = (float10)FUN_005b23a0(param_1);
            local_20c = (float)fVar10;
            uStack_27c = 0;
            pfStack_278 = (float *)FUN_00618b70(4);
            if ((pfStack_278 != (float *)0x0) && (&stack0x00000000 != (undefined1 *)0x20c)) {
              *pfStack_278 = local_20c;
              uStack_27c = 4;
            }
            local_4._0_1_ = 6;
            iVar4 = FUN_005aaf90();
            uStack_264 = *(uint *)(iVar4 + 0x87c);
            uStack_260 = 0;
            iVar4 = piVar5[0x3b];
            iVar2 = param_1[0x3b];
            iVar6 = param_1[0x5a];
            fVar13 = (float)uStack_264 - _DAT_0066c4d0 * _DAT_0066c510;
            fVar12 = local_20c;
            pvVar3 = (void *)FUN_005aaf90();
            FUN_005a6ac0(pvVar3,iVar6,iVar2,iVar4,fVar13,fVar12);
            pvVar3 = (void *)FUN_00618b70(0x220);
            local_4._0_1_ = 7;
            if (pvVar3 == (void *)0x0) {
              local_2ac = (void *)0x0;
            }
            else {
              iVar4 = param_1[0x3b];
              iVar2 = piVar5[0x3b];
              iVar6 = FUN_005aaf90();
              uStack_25c = *(uint *)(iVar6 + 0x87c);
              uStack_258 = 0;
              local_2ac = FUN_005ce460(pvVar3,(float)uStack_25c,0x10,iVar2,iVar4,&uStack_27c);
            }
            local_4 = CONCAT31(local_4._1_3_,6);
            piVar7 = (int *)FUN_005aaf90();
            (**(code **)(*piVar7 + 0x2c))(local_2ac);
            pvVar3 = (void *)FUN_00618b70(0x220);
            local_c._0_1_ = 8;
            if (pvVar3 == (void *)0x0) {
              pvVar3 = (void *)0x0;
            }
            else {
              iVar4 = param_1[0x3b];
              iVar2 = piVar5[0x3b];
              iVar6 = FUN_005aaf90();
              uStack_254 = *(uint *)(iVar6 + 0x87c);
              uStack_250 = 0;
              pvVar3 = FUN_005ce460(pvVar3,(float)uStack_254 - _DAT_0066c4d0 * _DAT_0066c510,0x14,
                                    iVar2,iVar4,&uStack_284);
            }
            local_c = (void *)CONCAT31(local_c._1_3_,6);
            piVar5 = (int *)FUN_005aaf90();
            (**(code **)(*piVar5 + 0x2c))(pvVar3,1);
            local_4 = CONCAT31(local_4._1_3_,3);
            if (pfStack_278 != (float *)0x0) {
              FUN_00618b60((undefined *)pfStack_278);
            }
            pfStack_278 = (float *)0x0;
          }
          param_1[uVar1 * 0x1d + 0x78] = 0;
          param_1[uVar1 * 0x1d + 0x79] =
               (int)((float)param_1[uVar1 * 0x1d + 0x79] - (float)param_1[uVar1 * 0x1d + 0x5f]);
          FUN_005b64e0(param_1,local_298);
          local_4 = 0xffffffff;
          if (pfStack_290 != (float *)0x0) {
            FUN_00618b60((undefined *)pfStack_290);
          }
          pfStack_290 = (float *)0x0;
        }
        param_1[uVar1 * 0x1d + 0x62] = 0;
      }
      if ((float)param_1[uVar1 * 0x1d + 99] != _DAT_0066c4f0) {
        iVar4 = param_1[uVar1 * 0x1d + 0x76];
        FUN_005aaf90();
        iVar2 = FUN_005ad870();
        if ((iVar2 != 0) && (param_1[0x5c] != 6)) {
          local_20c = (float)param_1[uVar1 * 0x1d + 99];
          uStack_284 = 0;
          pfStack_280 = (float *)FUN_00618b70(4);
          fVar13 = local_20c;
          if ((pfStack_280 != (float *)0x0) && (&stack0x00000000 != (undefined1 *)0x20c)) {
            *pfStack_280 = local_20c;
            uStack_284 = 4;
          }
          local_4 = 9;
          iVar2 = FUN_005aaf90();
          uStack_23c = *(uint *)(iVar2 + 0x87c);
          uStack_238 = 0;
          iVar2 = param_1[0x3b];
          fVar12 = (float)uStack_23c + _DAT_0066c4d0;
          uVar11 = *(undefined4 *)(iVar4 + 0xec);
          iVar6 = param_1[0x5a];
          pvVar3 = (void *)FUN_005aaf90();
          FUN_005a6ac0(pvVar3,iVar6,iVar2,uVar11,fVar12,fVar13);
          pvVar3 = (void *)FUN_00618b70(0x220);
          local_4._0_1_ = 10;
          pvStack_270 = pvVar3;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            iVar2 = param_1[0x3b];
            uVar11 = *(undefined4 *)(iVar4 + 0xec);
            iVar4 = FUN_005aaf90();
            uStack_22c = *(uint *)(iVar4 + 0x87c);
            uStack_228 = 0;
            pvVar3 = FUN_005ce460(pvVar3,(float)uStack_22c,0xe,uVar11,iVar2,&uStack_284);
          }
          local_4 = CONCAT31(local_4._1_3_,9);
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x2c))(pvVar3);
          pvVar3 = (void *)FUN_00618b70(0x220);
          local_c._0_1_ = 0xb;
          pfStack_278 = (float *)pvVar3;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            iVar4 = param_1[0x3b];
            uVar11 = *(undefined4 *)((int)unaff_EBP + 0xec);
            iVar2 = FUN_005aaf90();
            uStack_224 = *(uint *)(iVar2 + 0x87c);
            uStack_220 = 0;
            pvVar3 = FUN_005ce460(pvVar3,(float)uStack_224 + _DAT_0066c4d0,0x13,uVar11,iVar4,
                                  &local_28c);
          }
          local_c = (void *)CONCAT31(local_c._1_3_,9);
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x2c))(pvVar3,1);
          param_1[uVar1 * 0x1d + 0x78] = 0;
          param_1[uVar1 * 0x1d + 0x79] =
               (int)((float)param_1[uVar1 * 0x1d + 0x79] - (float)param_1[uVar1 * 0x1d + 0x61]);
          FUN_005b64e0(param_1,local_298);
          local_4 = 0xffffffff;
          if (pfStack_280 != (float *)0x0) {
            FUN_00618b60((undefined *)pfStack_280);
          }
          pfStack_280 = (float *)0x0;
        }
        param_1[uVar1 * 0x1d + 99] = 0;
      }
    }
  } while (local_29c != _DAT_0066c4f0);
  ExceptionList = local_c;
  return (local_274 != 0) + '\x03';
}

