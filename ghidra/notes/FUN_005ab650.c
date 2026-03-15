
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005ab650(void *param_1)

{
  float fVar1;
  uint uVar2;
  bool bVar3;
  bool bVar4;
  int iVar5;
  int *piVar6;
  int *piVar7;
  int *piVar8;
  undefined3 extraout_var;
  float *pfVar9;
  undefined3 extraout_var_00;
  int iVar10;
  int iVar11;
  int *piVar12;
  undefined4 uVar13;
  undefined4 *puVar14;
  void *pvVar15;
  DWORD DVar16;
  int unaff_EBX;
  float fVar17;
  uint uVar18;
  ushort uVar19;
  uint *puVar20;
  int aiStackY_140 [12];
  undefined4 uStackY_110;
  int iStack_e0;
  undefined ***pppuVar21;
  int *piStack_9c;
  float fStack_98;
  undefined **ppuStack_94;
  float fStack_90;
  int iStack_8c;
  float fStack_88;
  int *piStack_84;
  uint uStack_80;
  int *piStack_7c;
  int *apiStack_78 [2];
  undefined **ppuStack_70;
  int iStack_6c;
  int iStack_68;
  int iStack_64;
  uint local_60;
  uint local_5c;
  undefined4 local_58;
  void *apvStack_54 [2];
  uint uStack_4c;
  int iStack_44;
  int iStack_40;
  int iStack_3c;
  int iStack_38;
  undefined **local_34 [3];
  undefined **local_28 [7];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652e7b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined4 *)((int)param_1 + 0x8f4) = *(undefined4 *)((int)param_1 + 0x898);
  *(undefined4 *)((int)param_1 + 0x8f0) = *(undefined4 *)((int)param_1 + 0x898);
  *(undefined4 *)((int)param_1 + 0x8cc) = 0x42c80000;
  FUN_005aaf90();
  iVar5 = FUN_005a9330();
  local_58 = 0;
  *(undefined4 *)((int)param_1 + 0x8d4) = 0;
  if (iVar5 != 0) {
    local_58 = *(undefined4 *)(iVar5 + 0x18);
    *(uint *)((int)param_1 + 0x8d4) = (-(uint)(*(int *)(iVar5 + 0x74) != 0) & 10) + 0x1392;
  }
  *(undefined4 *)((int)param_1 + 0xa50) = 0x7da;
  *(undefined4 *)((int)param_1 + 0xa54) = 0x7e4;
  *(undefined4 *)((int)param_1 + 0xa58) = 0x7ee;
  *(undefined4 *)((int)param_1 + 0xa5c) = 0x7f8;
  *(undefined4 *)((int)param_1 + 0xa60) = 0x802;
  *(undefined4 *)((int)param_1 + 0xa64) = 0x80c;
  *(undefined4 *)((int)param_1 + 0xa68) = 0x816;
  *(undefined4 *)((int)param_1 + 0xa6c) = 0x820;
  *(undefined4 *)((int)param_1 + 0xa70) = 0x82a;
  *(undefined4 *)((int)param_1 + 0xa74) = 0x834;
  *(undefined4 *)((int)param_1 + 0xa78) = 0x83e;
  *(undefined4 *)((int)param_1 + 0xa7c) = 0x848;
  *(undefined4 *)((int)param_1 + 0xa80) = 0x852;
  *(undefined4 *)((int)param_1 + 0xa84) = 0x85c;
  *(undefined4 *)((int)param_1 + 0xa88) = 0x866;
  *(undefined4 *)((int)param_1 + 0xa8c) = 0x9ce;
  *(undefined4 *)((int)param_1 + 0xa90) = 0x9d8;
  *(undefined4 *)((int)param_1 + 0xa94) = 0x9e2;
  *(undefined4 *)((int)param_1 + 0xa98) = 0x9ec;
  *(undefined4 *)((int)param_1 + 0xa9c) = 0x9f6;
  *(undefined4 *)((int)param_1 + 0xaa0) = 0xa00;
  *(undefined4 *)((int)param_1 + 0xaa4) = 0xa0a;
  *(undefined4 *)((int)param_1 + 0xaa8) = 0xa14;
  *(undefined4 *)((int)param_1 + 0xaac) = 0xa1e;
  *(undefined4 *)((int)param_1 + 0xab0) = 0xa28;
  *(undefined4 *)((int)param_1 + 0xab4) = 0xa32;
  *(undefined4 *)((int)param_1 + 0xab8) = 0xa3c;
  *(undefined4 *)((int)param_1 + 0xabc) = 0xa46;
  *(undefined4 *)((int)param_1 + 0xac0) = 0xa50;
  *(undefined4 *)((int)param_1 + 0xae4) = 0x1392;
  if (*(int *)((int)param_1 + 0x9ec) == 0) {
    *(undefined4 *)((int)param_1 + 0xac4) = 4000;
    *(undefined4 *)((int)param_1 + 0xac8) = 4000;
    *(undefined4 *)((int)param_1 + 0xacc) = 0xfb4;
    *(undefined4 *)((int)param_1 + 0xad0) = 0xfbe;
    *(undefined4 *)((int)param_1 + 0xad4) = 0x1008;
    *(undefined4 *)((int)param_1 + 0xad8) = 0x1012;
    *(undefined4 *)((int)param_1 + 0xadc) = 0x101c;
    *(undefined4 *)((int)param_1 + 0xae0) = 0x1026;
  }
  else {
    *(undefined4 *)((int)param_1 + 0xac4) = 0xfa4;
    *(undefined4 *)((int)param_1 + 0xac8) = 0xfae;
    *(undefined4 *)((int)param_1 + 0xacc) = 0xfb8;
    *(undefined4 *)((int)param_1 + 0xad0) = 0xfc2;
    *(undefined4 *)((int)param_1 + 0xad4) = 0x1004;
    *(undefined4 *)((int)param_1 + 0xad8) = 0x100e;
    *(undefined4 *)((int)param_1 + 0xadc) = 0x1018;
    *(undefined4 *)((int)param_1 + 0xae0) = 0x1022;
  }
  FUN_005f52c0(local_28);
  local_28[0] = &PTR_FUN_0066bdd0;
  local_4 = 0;
  FUN_005f52c0(local_34);
  local_34[0] = &PTR_FUN_0066bdd0;
  local_4 = CONCAT31(local_4._1_3_,1);
  local_5c = 0;
  local_60 = 0;
  (**(code **)(*(int *)((int)param_1 + 0x9a0) + 0x10))();
  piStack_84 = (int *)0x0;
  piStack_9c = (int *)0x0;
  uStack_80 = 0;
  for (piVar6 = (int *)(**(code **)(*(int *)((int)param_1 + 0x9a0) + 0x10))(); piStack_7c = piVar6,
      piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
    piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
    piVar8 = (int *)FUN_005b5ad0(piVar7,apvStack_54);
    piVar7[4] = *piVar8;
    piVar7[5] = piVar8[1];
    piVar7[6] = piVar8[2];
    piVar7[7] = piVar8[3];
    iVar5 = (**(code **)(*piVar7 + 0x20))();
    if (iVar5 == 1) {
      bVar4 = FUN_005b69f0((int)piVar7);
      if (CONCAT31(extraout_var,bVar4) == 0) {
        FUN_005c5f20((void *)(*(int *)((int)param_1 + piVar7[0x5a] * 4 + 8) + 100),(int)piVar7);
      }
    }
    else {
      FUN_005c5f20((void *)(*(int *)((int)param_1 + piVar7[0x5a] * 4 + 8) + 0x58),(int)piVar7);
      if (piVar7[0x5a] == *(int *)((int)param_1 + 0x8e8)) {
        piVar7[0x42] = 1;
      }
      ppuStack_70 = (undefined **)piVar7[4];
      iStack_6c = piVar7[5];
      iStack_68 = piVar7[6];
      iStack_64 = piVar7[7];
      iStack_44 = piVar7[0xc];
      iStack_40 = piVar7[0xd];
      iStack_3c = piVar7[0xe];
      iVar5 = piVar7[0x5a];
      iVar10 = piVar7[0x3b];
      iStack_38 = piVar7[0xf];
      pppuVar21 = &ppuStack_70;
      piVar6 = &iStack_44;
      iVar11 = (**(code **)(*piVar7 + 0x24))();
      pfVar9 = (float *)FUN_005c1100(iVar10,iVar5,*(short *)((int)param_1 + iVar11 * 4 + 0xa50),
                                     piVar6,pppuVar21);
      piVar6 = piStack_7c;
      if (pfVar9 != (float *)0x0) {
        ppuStack_94 = (undefined **)(pfVar9[4] - pfVar9[1]);
        fStack_98 = pfVar9[3] - *pfVar9;
        fStack_90 = pfVar9[5] - pfVar9[2];
        fVar17 = (float)ppuStack_94 * _DAT_0066c29c;
        if ((float)ppuStack_94 * _DAT_0066c29c < _DAT_0066c29c) {
          fVar17 = _DAT_0066c29c;
        }
        piVar7[2] = (int)fVar17;
        fStack_88 = SQRT(fStack_98 * fStack_98 + fStack_90 * fStack_90);
        piVar7[0x33] = (int)ABS(pfVar9[5] - pfVar9[2]);
        apiStack_78[0] = (int *)ABS(pfVar9[3] - *pfVar9);
        piVar7[0x34] = (int)apiStack_78[0];
        fVar17 = _DAT_0066c23c;
        if (_DAT_0066c23c <= fStack_88) {
          fVar17 = fStack_88;
        }
        piVar7[3] = (int)fVar17;
      }
    }
    bVar4 = FUN_005b69f0((int)piVar7);
    if (CONCAT31(extraout_var_00,bVar4) == 0) {
      FUN_005c5f20((void *)((int)param_1 + 0x988),(int)piVar7);
      if (piVar7[0x5a] == 0) {
        local_60 = local_60 + 1;
        if (local_5c < local_60) {
LAB_005abb3f:
          *(float *)((int)param_1 + 0x8cc) = *(float *)((int)param_1 + 0x8cc) - _DAT_0066c2a0;
        }
      }
      else {
        local_5c = local_5c + 1;
        if (local_60 < local_5c) goto LAB_005abb3f;
      }
    }
  }
  for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)((int)param_1 + 0xc) + 0x58) + 0x10))();
      piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
    piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
    if ((uStack_80 & 1) == 0) {
      piStack_9c = (int *)((float)piStack_9c * _DAT_0066c27c);
    }
    else {
      bVar4 = (float)piStack_9c < _DAT_0066c284;
      piStack_84 = (int *)((float)piStack_84 - _DAT_0066c2a4);
      piStack_9c = piStack_84;
      if (bVar4) {
        piStack_9c = (int *)-(float)piStack_84;
      }
    }
    uStack_80 = uStack_80 + 1;
    FUN_005a9030(param_1,piVar7,(int)piStack_9c,0,
                 (int)(*(float *)((int)param_1 + 0x8cc) * _DAT_0066c29c));
  }
  bVar4 = false;
  piStack_84 = (int *)0x0;
  piStack_9c = (int *)0x0;
  for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)((int)param_1 + 8) + 0x58) + 0x10))();
      piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
    piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
    if (bVar4) {
      bVar3 = (float)piStack_9c < _DAT_0066c284;
      piStack_84 = (int *)((float)piStack_84 - _DAT_0066c2a4);
      piStack_9c = piStack_84;
      if (bVar3) {
        piStack_9c = (int *)-(float)piStack_84;
      }
    }
    else {
      piStack_9c = (int *)((float)piStack_9c * _DAT_0066c27c);
    }
    bVar4 = (bool)(bVar4 ^ 1);
    FUN_005a9030(param_1,piVar7,(int)piStack_9c,0,
                 (int)(*(float *)((int)param_1 + 0x8cc) * _DAT_0066c2a8));
  }
  bVar4 = false;
  piStack_84 = (int *)0x0;
  piStack_9c = (int *)0x0;
  for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)((int)param_1 + 8) + 100) + 0x10))();
      piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
    piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
    if (bVar4) {
      bVar3 = (float)piStack_9c < _DAT_0066c284;
      piStack_84 = (int *)((float)piStack_84 - _DAT_0066c2a4);
      piStack_9c = piStack_84;
      if (bVar3) {
        piStack_9c = (int *)-(float)piStack_84;
      }
    }
    else {
      piStack_9c = (int *)((float)piStack_9c * _DAT_0066c27c);
    }
    bVar4 = (bool)(bVar4 ^ 1);
    FUN_005a9030(param_1,piVar7,(int)piStack_9c,0,
                 (int)(_DAT_0066c2ac - *(float *)((int)param_1 + 0x8cc) * _DAT_0066c29c));
  }
  bVar4 = false;
  piStack_84 = (int *)0x0;
  piStack_9c = (int *)0x0;
  for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)((int)param_1 + 0xc) + 100) + 0x10))();
      piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
    piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
    if (bVar4) {
      bVar3 = (float)piStack_9c < _DAT_0066c284;
      piStack_84 = (int *)((float)piStack_84 - _DAT_0066c2a4);
      piStack_9c = piStack_84;
      if (bVar3) {
        piStack_9c = (int *)-(float)piStack_84;
      }
    }
    else {
      piStack_9c = (int *)((float)piStack_9c * _DAT_0066c27c);
    }
    bVar4 = (bool)(bVar4 ^ 1);
    FUN_005a9030(param_1,piVar7,(int)piStack_9c,0,
                 (int)(*(float *)((int)param_1 + 0x8cc) * _DAT_0066c29c - _DAT_0066c2ac));
  }
  fStack_98 = 0.0;
  ppuStack_94 = (undefined **)0x0;
  fStack_90 = 0.0;
  iStack_8c = 0;
  for (piVar6 = (int *)(**(code **)(*(int *)((int)param_1 + 0x9a0) + 0x10))(); piVar6 != (int *)0x0;
      piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
    iVar5 = (**(code **)(*piVar6 + 0x10))();
    if (*(int *)(iVar5 + 0x168) == 0) {
      piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
      iVar5 = (**(code **)(*piVar7 + 0x24))();
      if (iVar5 - 0x1dU < 4) {
        piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
        iVar5 = (**(code **)(*piVar7 + 0x24))();
        *(int *)(&stack0xfffffef4 + iVar5 * 4) = *(int *)(&stack0xfffffef4 + iVar5 * 4) + 1;
      }
    }
  }
  fVar17 = 0.0;
  iVar5 = 0;
  do {
    fVar1 = (&fStack_98)[iVar5];
    apvStack_54[iVar5] = (void *)0x0;
    if ((uint)fVar17 < (uint)fVar1) {
      fVar17 = fVar1;
    }
    if (fVar1 != 0.0) {
      piVar6 = FUN_005adfa0(param_1,(int *)((int)param_1 + 0x9a0),iVar5 + 0x1d,iVar5);
      apvStack_54[iVar5] = piVar6;
    }
    iVar5 = iVar5 + 1;
  } while (iVar5 < 4);
  uStack_80 = 0;
  do {
    uVar18 = uStack_80;
    if ((&fStack_98)[uStack_80] == 0.0) {
      fVar17 = 0.0;
      iVar5 = 0;
      iVar10 = 0;
      pfVar9 = &fStack_98;
      do {
        if ((uint)fVar17 < (uint)*pfVar9) {
          fVar17 = *pfVar9;
          iVar5 = iVar10;
        }
        iVar10 = iVar10 + 1;
        pfVar9 = pfVar9 + 1;
      } while (iVar10 < 4);
      if (7 < (uint)(&fStack_98)[iVar5]) {
        fVar17 = (float)((uint)(&fStack_98)[iVar5] >> 1);
        (&fStack_98)[uStack_80] = fVar17;
        (&fStack_98)[iVar5] = (float)((int)(&fStack_98)[iVar5] - (int)fVar17);
        FUN_005f52c0(&ppuStack_70);
        ppuStack_70 = &PTR_FUN_0066bdd0;
        piVar6 = (int *)apvStack_54[iVar5];
        local_4 = CONCAT31(local_4._1_3_,2);
        piVar7 = (int *)(**(code **)(piVar6[7] + 0x10))();
        (**(code **)(*piVar7 + 8))();
        piVar8 = (int *)(**(code **)(*piVar7 + 0x10))();
        piStack_7c = (int *)(**(code **)(*piVar8 + 0x24))();
        fVar17 = (&fStack_98)[uVar18];
        if ((&fStack_98)[uVar18] != 0.0) {
          do {
            fStack_88 = fVar17;
            apiStack_78[0] = (int *)(**(code **)(*piVar7 + 8))();
            iVar5 = (**(code **)(*piVar7 + 0x10))();
            FUN_005c5f20(&ppuStack_70,iVar5);
            iVar5 = *piVar6;
            (**(code **)(*piVar7 + 4))();
            (**(code **)(iVar5 + 0x54))();
            iVar5 = *piVar6;
            (**(code **)(*piVar7 + 4))();
            (**(code **)(iVar5 + 0x30))();
            piVar7 = apiStack_78[0];
            fVar17 = (float)((int)fStack_88 + -1);
          } while ((float)((int)fStack_88 + -1) != 0.0);
          fStack_88 = 0.0;
          uVar18 = uStack_80;
        }
        piVar6 = FUN_005adfa0(param_1,(int *)&ppuStack_70,(int)piStack_7c,uVar18);
        apvStack_54[uVar18] = piVar6;
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_005c5ff0(&ppuStack_70);
      }
    }
    uStack_80 = uVar18 + 1;
  } while ((int)(uVar18 + 1) < 4);
  FUN_005c81d0(*(int **)((int)param_1 + 8));
  (**(code **)(**(int **)((int)param_1 + 8) + 0x1c))();
  (**(code **)(**(int **)((int)param_1 + 8) + 0x1c))();
  (**(code **)(**(int **)((int)param_1 + 8) + 0x1c))();
  for (piVar6 = (int *)(**(code **)(*(int *)((int)param_1 + 0x9a0) + 0x10))(); piVar6 != (int *)0x0;
      piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
    iVar5 = (**(code **)(*piVar6 + 0x10))();
    if (*(int *)(iVar5 + 0x168) == 1) {
      piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
      iVar5 = (**(code **)(*piVar7 + 0x24))();
      if (iVar5 - 0x21U < 4) {
        piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
        iVar5 = (**(code **)(*piVar7 + 0x24))();
        aiStackY_140[iVar5] = aiStackY_140[iVar5] + 1;
      }
    }
  }
  uVar18 = 0;
  iVar5 = 0;
  do {
    uVar2 = *(uint *)(&stack0xffffff44 + iVar5 * 4);
    apiStack_78[iVar5] = (int *)0x0;
    if (uVar18 < uVar2) {
      uVar18 = uVar2;
    }
    if (uVar2 != 0) {
      iStack_e0 = 0x5ac03c;
      piVar6 = FUN_005adfa0(param_1,(int *)((int)param_1 + 0x9a0),iVar5 + 0x21,iVar5);
      apiStack_78[iVar5] = piVar6;
    }
    iVar5 = iVar5 + 1;
  } while (iVar5 < 4);
  iVar5 = 0;
  do {
    if (*(int *)(&stack0xffffff44 + iVar5 * 4) == 0) {
      uVar18 = 0;
      iVar10 = 0;
      iVar11 = 0;
      puVar20 = (uint *)&stack0xffffff44;
      do {
        if (uVar18 < *puVar20) {
          uVar18 = *puVar20;
          iVar10 = iVar11;
        }
        iVar11 = iVar11 + 1;
        puVar20 = puVar20 + 1;
      } while (iVar11 < 4);
      iVar11 = iVar10 * 4;
      if (7 < *(uint *)(&stack0xffffff44 + iVar11)) {
        uVar18 = *(uint *)(&stack0xffffff44 + iVar11) >> 1;
        *(uint *)(&stack0xffffff44 + iVar5 * 4) = uVar18;
        *(uint *)(&stack0xffffff44 + iVar11) = *(int *)(&stack0xffffff44 + iVar11) - uVar18;
        FUN_005f52c0(&ppuStack_94);
        ppuStack_94 = &PTR_FUN_0066bdd0;
        piVar6 = apiStack_78[iVar10];
        local_28[0] = (undefined **)CONCAT31(local_28[0]._1_3_,3);
        piVar7 = (int *)(**(code **)(piVar6[7] + 0x10))();
        (**(code **)(*piVar7 + 8))();
        piVar8 = (int *)(**(code **)(*piVar7 + 0x10))();
        unaff_EBX = (**(code **)(*piVar8 + 0x24))();
        for (iVar10 = *(int *)(&stack0xffffff44 + iVar5 * 4); iVar10 != 0; iVar10 = iVar10 + -1) {
          piStack_9c = (int *)(**(code **)(*piVar7 + 8))();
          iVar11 = (**(code **)(*piVar7 + 0x10))();
          FUN_005c5f20(&ppuStack_94,iVar11);
          iVar11 = *piVar6;
          (**(code **)(*piVar7 + 4))();
          (**(code **)(iVar11 + 0x54))();
          iVar11 = *piVar6;
          (**(code **)(*piVar7 + 4))();
          (**(code **)(iVar11 + 0x30))();
          piVar7 = piStack_9c;
        }
        iStack_e0 = 0x5ac146;
        piVar6 = FUN_005adfa0(param_1,(int *)&ppuStack_94,unaff_EBX,iVar5);
        apiStack_78[iVar5] = piVar6;
        local_28[0] = (undefined **)CONCAT31(local_28[0]._1_3_,1);
        FUN_005c5ff0(&ppuStack_94);
      }
    }
    iVar5 = iVar5 + 1;
  } while (iVar5 < 4);
  FUN_005c81d0(*(int **)((int)param_1 + 0xc));
  iStack_e0 = 0x5ac184;
  iStack_e0 = (**(code **)(**(int **)((int)param_1 + 0xc) + 0x1c))();
  iStack_e0 = iStack_e0 + 3;
  (**(code **)(**(int **)((int)param_1 + 0xc) + 0x1c))();
  (**(code **)(**(int **)((int)param_1 + 0xc) + 0x1c))();
  piVar6 = (int *)(**(code **)(*(int *)((int)param_1 + 0x9a0) + 0x10))();
  do {
    do {
      while( true ) {
        if (piVar6 == (int *)0x0) {
          *(undefined4 *)((int)param_1 + 0x964) = 0xc97423f0;
          *(undefined4 *)((int)param_1 + 0x968) = 0xc97423f0;
          *(undefined4 *)((int)param_1 + 0x960) = 0x497423f0;
          *(undefined4 *)((int)param_1 + 0x96c) = 0x497423f0;
          for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)((int)param_1 + 8) + 0x58) + 0x10))();
              piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
            iVar5 = (**(code **)(*piVar6 + 0x10))();
            fVar17 = *(float *)(iVar5 + 0x30);
            fVar1 = *(float *)(iVar5 + 0x34);
            if (fVar17 < *(float *)((int)param_1 + 0x960)) {
              *(float *)((int)param_1 + 0x960) = fVar17;
            }
            if (*(float *)((int)param_1 + 0x968) < fVar17) {
              *(float *)((int)param_1 + 0x968) = fVar17;
            }
            if (fVar1 < *(float *)((int)param_1 + 0x96c)) {
              *(float *)((int)param_1 + 0x96c) = fVar1;
            }
            if (*(float *)((int)param_1 + 0x964) < fVar1) {
              *(float *)((int)param_1 + 0x964) = fVar1;
            }
          }
          *(undefined4 *)((int)param_1 + 0x974) = 0xc97423f0;
          *(undefined4 *)((int)param_1 + 0x978) = 0xc97423f0;
          *(undefined4 *)((int)param_1 + 0x970) = 0x497423f0;
          *(undefined4 *)((int)param_1 + 0x97c) = 0x497423f0;
          for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)((int)param_1 + 0xc) + 0x58) + 0x10))()
              ; piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
            iVar5 = (**(code **)(*piVar6 + 0x10))();
            fVar17 = *(float *)(iVar5 + 0x30);
            fVar1 = *(float *)(iVar5 + 0x34);
            if (fVar17 < *(float *)((int)param_1 + 0x970)) {
              *(float *)((int)param_1 + 0x970) = fVar17;
            }
            if (*(float *)((int)param_1 + 0x978) < fVar17) {
              *(float *)((int)param_1 + 0x978) = fVar17;
            }
            if (fVar1 < *(float *)((int)param_1 + 0x97c)) {
              *(float *)((int)param_1 + 0x97c) = fVar1;
            }
            if (*(float *)((int)param_1 + 0x974) < fVar1) {
              *(float *)((int)param_1 + 0x974) = fVar1;
            }
          }
          pvVar15 = (void *)FUN_0059ed70();
          FUN_005a0820(pvVar15);
          FUN_005c10a0(unaff_EBX,*(float *)((int)param_1 + 0x8cc));
          fVar17 = *(float *)((int)param_1 + 0x8cc);
          pvVar15 = (void *)FUN_0059ed70();
          FUN_0059f800(pvVar15,fVar17);
          iVar5 = FUN_0059ed70();
          FUN_0059fb30(iVar5);
          DVar16 = timeGetTime();
          *(undefined4 *)((int)param_1 + 0x8a8) = 1000;
          *(undefined4 *)((int)param_1 + 0x878) = 0;
          *(undefined4 *)((int)param_1 + 0x884) = 0;
          *(undefined4 *)((int)param_1 + 0x87c) = *(undefined4 *)((int)param_1 + 0x8ac);
          *(float *)((int)param_1 + 0x8f0) = (float)DVar16;
          uStack_4c = uStack_4c & 0xffffff00;
          FUN_005c5ff0(&piStack_7c);
          uStack_4c = 0xffffffff;
          FUN_005c5ff0(&ppuStack_70);
          ExceptionList = apvStack_54[0];
          return;
        }
        piVar8 = (int *)(**(code **)(*piVar6 + 0x10))();
        piVar6 = (int *)(**(code **)(*piVar6 + 8))();
        piVar7 = piVar6;
        piVar12 = (int *)FUN_005b5ad0(piVar8,&iStack_64);
        piVar8[4] = *piVar12;
        piVar8[5] = piVar12[1];
        piVar8[6] = piVar12[2];
        piVar8[7] = piVar12[3];
        iVar5 = (**(code **)(*piVar8 + 0x20))();
        if (iVar5 != 0) break;
        iStack_8c = piVar8[4];
        fStack_88 = (float)piVar8[5];
        piStack_84 = (int *)piVar8[6];
        uStack_80 = piVar8[7];
        iStack_e0 = piVar8[0xc];
        FUN_005c12a0(piVar8[0x3b],&iStack_e0,&iStack_8c);
      }
      iVar5 = (**(code **)(*piVar8 + 0x20))();
    } while (iVar5 != 1);
    iVar5 = FUN_005ad870();
    if ((iVar5 == 0) || (piVar8[0x5a] == *(int *)((int)param_1 + 0x8e8))) {
      piVar8[0x42] = 1;
    }
    if (piVar8[0x5a] == *(int *)((int)param_1 + 0x9ec)) {
      uVar13 = (**(code **)(*piVar8 + 0x24))();
      switch(uVar13) {
      case 0x1d:
        uVar19 = 4000;
        break;
      case 0x1e:
        uVar19 = 0xfaa;
        break;
      case 0x1f:
        uVar19 = 0xfb4;
        break;
      case 0x20:
        uVar19 = 0xfbe;
        break;
      case 0x21:
        uVar19 = 0x1004;
        break;
      case 0x22:
        uVar19 = 0x100e;
        break;
      case 0x23:
        uVar19 = 0x1018;
        break;
      case 0x24:
        uVar19 = 0x1022;
        break;
      default:
        goto switchD_005ac2cb_default;
      }
    }
    else {
      uVar13 = (**(code **)(*piVar8 + 0x24))();
      switch(uVar13) {
      case 0x1d:
        uVar19 = 0xfa4;
        break;
      case 0x1e:
        uVar19 = 0xfae;
        break;
      case 0x1f:
        uVar19 = 0xfb8;
        break;
      case 0x20:
        uVar19 = 0xfc2;
        break;
      case 0x21:
        uVar19 = 0x1008;
        break;
      case 0x22:
        uVar19 = 0x1012;
        break;
      case 0x23:
        uVar19 = 0x101c;
        break;
      case 0x24:
        uVar19 = 0x1026;
        break;
      default:
switchD_005ac2cb_default:
        uVar19 = 0x541;
      }
    }
    uVar18 = piVar8[0x5e];
    if (DAT_006bc470 == (undefined4 *)0x0) {
      puVar14 = (undefined4 *)FUN_00618b70(0x48);
      uStack_4c._0_1_ = 4;
      if (puVar14 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(puVar14);
      }
      uStack_4c = CONCAT31(uStack_4c._1_3_,1);
    }
    iVar5 = FUN_005a6470(DAT_006bc470,uVar18);
    if ((iVar5 != 0) && (piVar8[0x5a] == *(int *)((int)param_1 + 0x9ec))) {
      uVar13 = FUN_005c7150(iVar5);
      uVar19 = uVar19 + (short)uVar13;
    }
    piStack_9c = (int *)piVar8[0xc];
    fStack_98 = (float)piVar8[0xd];
    ppuStack_94 = (undefined **)piVar8[0xe];
    fStack_90 = (float)piVar8[0xf];
    uStackY_110 = 0x5ac421;
    FUN_005c1130(piVar8[0x3b],piVar8[0x5a],uVar19,&piStack_9c,&stack0xffffff48);
    piVar6 = piVar7;
  } while( true );
}

