
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005ab0e0(int param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  int iVar5;
  float fVar6;
  bool bVar7;
  int iVar8;
  int *piVar9;
  int *piVar10;
  int iVar11;
  float *pfVar12;
  undefined4 uVar13;
  undefined4 *puVar14;
  void *pvVar15;
  undefined3 extraout_var;
  int *this;
  ushort uVar16;
  int *piVar17;
  uint uVar18;
  undefined4 local_8c;
  int iStack_74;
  int iStack_70;
  int iStack_6c;
  int iStack_68;
  int iStack_64;
  int iStack_60;
  int iStack_5c;
  int iStack_58;
  int iStack_54;
  int iStack_50;
  int iStack_4c;
  int iStack_48;
  int iStack_44;
  int iStack_40;
  int iStack_3c;
  int iStack_38;
  undefined **local_34 [3];
  undefined **local_28 [3];
  undefined4 auStack_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652e3e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005aaf90();
  iVar8 = FUN_005a9330();
  local_8c = 0;
  *(undefined4 *)(param_1 + 0x8d4) = 0;
  if (iVar8 != 0) {
    local_8c = *(undefined4 *)(iVar8 + 0x18);
    if (*(int *)(iVar8 + 0x74) == 0) {
      iVar8 = FUN_005ba420(iVar8);
      if (iVar8 != 0) {
        *(undefined4 *)(param_1 + 0x8d4) = 0x1392;
      }
    }
    else {
      *(undefined4 *)(param_1 + 0x8d4) = 0x139c;
    }
  }
  FUN_005f52c0(local_28);
  local_28[0] = &PTR_FUN_0066bdd0;
  local_4 = 0;
  FUN_005f52c0(local_34);
  local_34[0] = &PTR_FUN_0066bdd0;
  local_4 = CONCAT31(local_4._1_3_,1);
  this = (int *)0x0;
  piVar9 = (int *)(**(code **)(*(int *)(param_1 + 0x9a0) + 0x10))();
  do {
    if (piVar9 == (int *)0x0) {
      FUN_005c10a0(local_8c,*(float *)(param_1 + 0x8cc));
      iVar8 = FUN_0059ed70();
      FUN_0059fb30(iVar8);
      uVar18 = 0xffffffff;
      pvVar15 = (void *)FUN_0059ed70();
      FUN_0059fc50(pvVar15,uVar18);
      pvVar15 = (void *)FUN_0059ed70();
      FUN_0059fca0(pvVar15);
      pvVar15 = (void *)FUN_0059ed70();
      FUN_0059e760(pvVar15);
      pvVar15 = (void *)FUN_0059ed70();
      FUN_005a0820(pvVar15);
      for (piVar9 = (int *)(**(code **)(*(int *)(param_1 + 0x9a0) + 0x10))(); piVar9 != (int *)0x0;
          piVar9 = (int *)(**(code **)(*piVar9 + 8))()) {
        bVar7 = FUN_005b23c0((int)this);
        if (((CONCAT31(extraout_var,bVar7) != 0) &&
            (iVar8 = (**(code **)(*this + 0x20))(), iVar8 == 0)) && (this[0x54] == 0)) {
          FUN_005c1360(this[0x3b],1);
        }
      }
      FUN_005ae1f0(param_1);
      local_4 = local_4 & 0xffffff00;
      FUN_005c5ff0(local_34);
      local_4 = 0xffffffff;
      FUN_005c5ff0(local_28);
      ExceptionList = pvStack_c;
      return;
    }
    this = (int *)(**(code **)(*piVar9 + 0x10))();
    piVar10 = (int *)FUN_005b5ad0(this,auStack_1c);
    this[4] = *piVar10;
    this[5] = piVar10[1];
    this[6] = piVar10[2];
    this[7] = piVar10[3];
    iVar8 = FUN_005ad870(this);
    if (iVar8 == 0) {
      this[0x42] = 1;
    }
    iVar11 = (**(code **)(*this + 0x20))();
    if (iVar11 == 0) {
      iStack_74 = this[4];
      iStack_6c = this[6];
      iStack_70 = this[5];
      iStack_68 = this[7];
      iStack_60 = this[0xd];
      iStack_64 = this[0xc];
      iVar11 = this[0x5a];
      iVar5 = this[0x3b];
      iStack_5c = this[0xe];
      iStack_58 = this[0xf];
      piVar10 = &iStack_74;
      piVar17 = &iStack_64;
      iVar8 = (**(code **)(*this + 0x24))(piVar17,piVar10,iVar8 != 0);
      pfVar12 = (float *)FUN_005c1100(iVar5,iVar11,*(short *)(param_1 + 0xa50 + iVar8 * 4),piVar17,
                                      piVar10);
      if (pfVar12 != (float *)0x0) {
        fVar1 = pfVar12[3];
        fVar6 = (pfVar12[4] - pfVar12[1]) * _DAT_0066c29c;
        fVar2 = pfVar12[5];
        fVar3 = *pfVar12;
        fVar4 = pfVar12[2];
        if (fVar6 < _DAT_0066c29c) {
          fVar6 = _DAT_0066c29c;
        }
        this[2] = (int)fVar6;
        fVar1 = SQRT((fVar1 - fVar3) * (fVar1 - fVar3) + (fVar2 - fVar4) * (fVar2 - fVar4));
        this[0x33] = (int)ABS(pfVar12[5] - pfVar12[2]);
        this[0x34] = (int)ABS(pfVar12[3] - *pfVar12);
        if (_DAT_0066c23c <= fVar1) {
          this[3] = (int)fVar1;
        }
        else {
          this[3] = (int)_DAT_0066c23c;
        }
      }
    }
    else {
      iVar8 = (**(code **)(*this + 0x20))();
      if (iVar8 == 1) {
        if (this[0x5a] == *(int *)(param_1 + 0x9ec)) {
          uVar13 = (**(code **)(*this + 0x24))();
          switch(uVar13) {
          case 0x1d:
            uVar16 = 4000;
            break;
          case 0x1e:
            uVar16 = 0xfaa;
            break;
          case 0x1f:
            uVar16 = 0xfb4;
            break;
          case 0x20:
            uVar16 = 0xfbe;
            break;
          case 0x21:
            uVar16 = 0x1004;
            break;
          case 0x22:
            uVar16 = 0x100e;
            break;
          case 0x23:
            uVar16 = 0x1018;
            break;
          case 0x24:
            uVar16 = 0x1022;
            break;
          default:
            goto switchD_005ab39e_default;
          }
        }
        else {
          uVar13 = (**(code **)(*this + 0x24))();
          switch(uVar13) {
          case 0x1d:
            uVar16 = 0xfa4;
            break;
          case 0x1e:
            uVar16 = 0xfae;
            break;
          case 0x1f:
            uVar16 = 0xfb8;
            break;
          case 0x20:
            uVar16 = 0xfc2;
            break;
          case 0x21:
            uVar16 = 0x1008;
            break;
          case 0x22:
            uVar16 = 0x1012;
            break;
          case 0x23:
            uVar16 = 0x101c;
            break;
          case 0x24:
            uVar16 = 0x1026;
            break;
          default:
switchD_005ab39e_default:
            uVar16 = 0x541;
          }
        }
        uVar18 = this[0x5e];
        if (DAT_006bc470 == (undefined4 *)0x0) {
          puVar14 = (undefined4 *)FUN_00618b70(0x48);
          local_4._0_1_ = 2;
          if (puVar14 == (undefined4 *)0x0) {
            DAT_006bc470 = (undefined4 *)0x0;
          }
          else {
            DAT_006bc470 = FUN_005a6290(puVar14);
          }
          local_4 = CONCAT31(local_4._1_3_,1);
        }
        iVar8 = FUN_005a6470(DAT_006bc470,uVar18);
        if ((iVar8 != 0) && (this[0x5a] == *(int *)(param_1 + 0x9ec))) {
          uVar13 = FUN_005c7150(iVar8);
          uVar16 = uVar16 + (short)uVar13;
        }
        iStack_44 = this[4];
        iStack_40 = this[5];
        iStack_3c = this[6];
        iStack_38 = this[7];
        iStack_54 = this[0xc];
        iStack_50 = this[0xd];
        iStack_4c = this[0xe];
        iStack_48 = this[0xf];
        FUN_005c1130(this[0x3b],this[0x5a],uVar16,&iStack_54,&iStack_44);
      }
    }
    piVar9 = (int *)(**(code **)(*piVar9 + 8))();
  } while( true );
}

