
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005b8f70(int *param_1)

{
  float fVar1;
  int iVar2;
  float fVar3;
  undefined4 *puVar4;
  int iVar5;
  int *piVar6;
  int iVar7;
  float fVar8;
  int unaff_ESI;
  int unaff_EDI;
  ushort uVar9;
  float10 fVar10;
  float10 fVar11;
  float10 fVar12;
  float unaff_retaddr;
  float local_b0;
  float local_ac;
  undefined4 *puStack_a8;
  float fStack_a4;
  float fStack_a0;
  float local_9c;
  float fStack_98;
  float fStack_94;
  float fStack_90;
  float fStack_8c;
  float local_88;
  float fStack_80;
  undefined **appuStack_7c [3];
  undefined4 uStack_70;
  undefined1 auStack_6c [4];
  float fStack_68;
  undefined1 auStack_60 [16];
  float fStack_50;
  float local_4c;
  float fStack_48;
  float fStack_3c;
  float fStack_38;
  float fStack_34;
  float fStack_2c;
  float fStack_28;
  float fStack_24;
  undefined4 uStack_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006534f6;
  pvStack_c = ExceptionList;
  iVar7 = 0;
  local_9c = 0.0;
  local_88 = 0.0;
  ExceptionList = &pvStack_c;
  param_1[0x101] = -0x800001;
  param_1[0x102] = 0x7f7fffff;
  (**(code **)(*param_1 + 0x7c))(&local_4c);
  if (((fStack_50 == DAT_006bc940) && (local_4c == DAT_006bc944)) && (fStack_48 == DAT_006bc948)) {
    if (DAT_006bc470 == (undefined4 *)0x0) {
      puVar4 = (undefined4 *)FUN_00618b70(0x48);
      puStack_8 = (undefined1 *)0x0;
      if (puVar4 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(puVar4);
      }
      puStack_8 = (undefined1 *)0xffffffff;
    }
    iVar5 = FUN_005a6480(DAT_006bc470,param_1[0x5d]);
    if (*(int *)(iVar5 + 100) == 3) {
      ExceptionList = pvStack_10;
      return;
    }
  }
  (**(code **)(*param_1 + 0x80))(auStack_60);
  fVar10 = (float10)FUN_005cd2d0((int)param_1);
  local_ac = (float)fVar10;
  (**(code **)(*param_1 + 4))(local_ac,uStack_4);
  FUN_005a33b0(&fStack_2c,param_1 + 4);
  FUN_005a33b0(&fStack_3c,param_1 + 8);
  local_b0 = fStack_38 * fStack_28 + fStack_34 * fStack_24 + fStack_3c * fStack_2c;
  if (local_b0 < _DAT_0066c6c4) {
    local_b0 = 0.25;
  }
  FUN_005f5830(appuStack_7c);
  appuStack_7c[0] = &PTR_FUN_0066c668;
  uStack_14 = 1;
  fVar11 = (float10)(**(code **)(*param_1 + 0x90))();
  fVar10 = (float10)(float)pvStack_c;
  fVar12 = (float10)_DAT_0066c6c8;
  iVar5 = FUN_005aaf90();
  piVar6 = FUN_005cdbf0((void *)(iVar5 + 0x8f8),param_1,
                        (float)(fVar11 * (float10)local_b0 * fVar10 * fVar12));
  if (piVar6 != (int *)0x0) {
    iVar5 = (**(code **)(*piVar6 + 0xc))();
    while (iVar5 != 0) {
      iVar7 = *(int *)(iVar5 + 0x10);
      puVar4 = (undefined4 *)(iVar7 + 0x30);
      FUN_005a33b0(&puStack_a8,puVar4);
      FUN_005a33b0(&fStack_90,param_1 + 0xc);
      fStack_a4 = fStack_a4 - fStack_8c;
      puStack_a8 = (undefined4 *)((float)puStack_a8 - fStack_90);
      local_9c = -1.0;
      fStack_a0 = fStack_a0 - local_88;
      if (fStack_a4 <= _DAT_0066c6d0) {
        if (_DAT_0066c6d0 <= fStack_a4) {
          FUN_005d25b0(appuStack_7c,iVar7);
        }
        else {
          fStack_98 = 1.4013e-45;
          FUN_005a33b0(auStack_6c,puVar4);
          fVar8 = (float)param_1[2] + *(float *)(iVar7 + 8) + fStack_68;
          if (fVar8 < (float)param_1[0x101]) {
            fVar8 = (float)param_1[0x101];
          }
          param_1[0x101] = (int)fVar8;
          if ((float)param_1[0xd] < fVar8) {
            unaff_EDI = 1;
          }
        }
      }
      else {
        unaff_ESI = 1;
        FUN_005a33b0(&local_4c,puVar4);
        fVar8 = (fStack_48 - *(float *)(iVar7 + 8)) - (float)param_1[2];
        if ((float)param_1[0x102] < fVar8) {
          fVar8 = (float)param_1[0x102];
        }
        param_1[0x102] = (int)fVar8;
        if (fVar8 < (float)param_1[0xd]) {
          local_ac = 1.4013e-45;
        }
      }
      iVar5 = FUN_0041c210(iVar5);
      iVar7 = unaff_ESI;
    }
  }
  iVar5 = FUN_0041c210((int)appuStack_7c);
  fVar8 = fStack_98;
  for (; iVar5 != 0; iVar5 = FUN_0041c210(iVar5)) {
    iVar2 = *(int *)(iVar5 + 0x10);
    if ((uint)param_1[0x3b] < *(uint *)(iVar2 + 0xec)) {
      if (fVar8 == 0.0) {
        iVar7 = 1;
        FUN_005a33b0(auStack_6c,(undefined4 *)(iVar2 + 0x30));
        fVar1 = (fStack_68 - *(float *)(iVar2 + 8)) - (float)param_1[2];
        if (fVar1 <= (float)param_1[0x102]) goto LAB_005b93b4;
        fVar1 = (float)param_1[0x102];
        param_1[0x102] = (int)fVar1;
        if ((float)param_1[0xd] <= fVar1) goto LAB_005b9487;
LAB_005b947f:
        local_ac = 1.4013e-45;
      }
      else {
        if (iVar7 == 0) {
          fVar8 = 1.4013e-45;
          FUN_005a33b0(&local_4c,(undefined4 *)(iVar2 + 0x30));
          fVar1 = (float)param_1[2] + *(float *)(iVar2 + 8) + fStack_48;
          if (fVar1 < (float)param_1[0x101]) goto LAB_005b9424;
          goto LAB_005b942c;
        }
LAB_005b93d1:
        fStack_80 = (float)param_1[0x101];
        fStack_94 = (float)param_1[0xd];
        if ((float)param_1[0xd] - fStack_80 <= (float)param_1[0x102] - fStack_94) {
          fVar1 = (float)param_1[2] + *(float *)(iVar2 + 8) + *(float *)(iVar2 + 0x34);
          uVar9 = (ushort)((float)param_1[0x101] < fVar1) << 8 |
                  (ushort)((float)param_1[0x101] == fVar1) << 0xe;
          goto LAB_005b941d;
        }
        fVar1 = (*(float *)(iVar2 + 0x34) - *(float *)(iVar2 + 8)) - (float)param_1[2];
        if ((float)param_1[0x102] < fVar1) {
          fVar1 = (float)param_1[0x102];
        }
        param_1[0x102] = (int)fVar1;
        if (fVar1 < (float)param_1[0xd]) goto LAB_005b947f;
      }
    }
    else if (iVar7 == 0) {
      fStack_98 = (float)param_1[0x101];
      fVar8 = 1.4013e-45;
      fVar1 = (float)param_1[2] + *(float *)(iVar2 + 8) + *(float *)(iVar2 + 0x34);
      uVar9 = (ushort)(fStack_98 < fVar1) << 8 | (ushort)(fStack_98 == fVar1) << 0xe;
LAB_005b941d:
      if (uVar9 == 0) {
LAB_005b9424:
        fVar1 = (float)param_1[0x101];
      }
LAB_005b942c:
      param_1[0x101] = (int)fVar1;
      if ((float)param_1[0xd] < fVar1) {
        unaff_EDI = 1;
      }
    }
    else {
      if (fVar8 != 0.0) goto LAB_005b93d1;
      fVar1 = (*(float *)(iVar2 + 0x34) - *(float *)(iVar2 + 8)) - (float)param_1[2];
      if ((float)param_1[0x102] < fVar1) {
        fVar1 = (float)param_1[0x102];
      }
LAB_005b93b4:
      iVar7 = 1;
      param_1[0x102] = (int)fVar1;
      if (fVar1 < (float)param_1[0xd]) goto LAB_005b947f;
    }
LAB_005b9487:
  }
  fVar10 = (float10)(**(code **)(*param_1 + 0x90))();
  fVar8 = (float)(fVar10 * (float10)_DAT_0066c6b4);
  if (local_ac == 0.0) {
LAB_005b960d:
    if (unaff_EDI == 0) goto LAB_005b969f;
    fVar1 = (float)param_1[0xd] - (float)param_1[0x101];
    if (_DAT_0066c6d0 < fVar1) goto LAB_005b962f;
    if (fVar1 < _DAT_0066c6d0) {
      fVar1 = -fVar1;
      fVar8 = fVar8 * (float)pvStack_c * _DAT_0066c6c8;
      if (fVar8 <= fVar1) goto LAB_005b968c;
    }
LAB_005b9692:
    fVar3 = (float)param_1[0xd] + fVar1;
  }
  else {
    if (unaff_EDI != 0) {
      if ((float)param_1[0x102] - (float)param_1[0x101] < (float)param_1[2] + (float)param_1[2])
      goto LAB_005b969f;
      fVar1 = ((float)param_1[0x102] - (float)param_1[0x101]) * _DAT_0066c6d8 - (float)param_1[0xd];
      if (fVar1 <= _DAT_0066c6d0) {
        if (_DAT_0066c6d0 <= fVar1) goto LAB_005b969f;
        fVar8 = fVar8 * (float)pvStack_c * _DAT_0066c6c8;
        fVar3 = -fVar1;
        if (fVar8 <= -fVar1) {
          fVar3 = (float)param_1[0xd] - fVar8;
          goto LAB_005b9697;
        }
        goto LAB_005b9603;
      }
LAB_005b962f:
      fVar8 = fVar8 * (float)pvStack_c * _DAT_0066c6c8;
      if (fVar8 <= fVar1) {
LAB_005b968c:
        fVar1 = fVar8;
      }
      goto LAB_005b9692;
    }
    if (local_ac == 0.0) goto LAB_005b960d;
    fVar1 = (float)param_1[0x102] - (float)param_1[0xd];
    fVar3 = fVar1;
    if (fVar1 <= _DAT_0066c6d0) {
      if (fVar1 < _DAT_0066c6d0) {
        fVar8 = fVar8 * (float)pvStack_c * _DAT_0066c6c8;
        fVar3 = -fVar1;
        if (fVar8 <= -fVar1) {
          fVar3 = fVar8;
        }
      }
LAB_005b9603:
      fVar3 = (float)param_1[0xd] - fVar3;
    }
    else {
      fVar8 = fVar8 * (float)pvStack_c * _DAT_0066c6c8;
      if (fVar1 < fVar8) goto LAB_005b9603;
      fVar3 = (float)param_1[0xd] - fVar8;
    }
  }
LAB_005b9697:
  param_1[0xf] = -0x40800000;
  param_1[0xd] = (int)fVar3;
LAB_005b969f:
  puStack_a8 = (undefined4 *)param_1[4];
  fStack_a4 = (float)param_1[5];
  fStack_a0 = (float)param_1[6];
  fVar10 = (float10)(**(code **)(*param_1 + 0x90))();
  fVar12 = fVar10 * (float10)local_9c * (float10)fStack_a4;
  fStack_80 = (float)(fVar10 * (float10)fStack_98);
  fVar11 = fVar10 * (float10)fStack_98 * (float10)fStack_a4;
  appuStack_7c[0] = (undefined **)(float)(fVar10 * (float10)fStack_94);
  fVar10 = fVar10 * (float10)fStack_94 * (float10)fStack_a4;
  local_9c = (float)fVar12;
  fStack_98 = (float)fVar11;
  fStack_94 = (float)fVar10;
  param_1[0x1b] = (int)(float)fVar12;
  param_1[0x1c] = (int)(float)fVar11;
  param_1[0x1d] = (int)(float)fVar10;
  param_1[0x1e] = -0x40800000;
  FUN_005cd640(param_1,unaff_retaddr);
  if (puStack_a8 != (undefined4 *)0x0) {
    (**(code **)*puStack_a8)(1);
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_005d2640(&uStack_70);
  ExceptionList = pvStack_10;
  return;
}

