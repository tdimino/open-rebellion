
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool __thiscall FUN_005a6d60(void *this,float param_1)

{
  float fVar1;
  bool bVar2;
  char cVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  undefined3 extraout_var;
  float *pfVar8;
  undefined3 extraout_var_00;
  void *this_00;
  undefined3 extraout_var_01;
  int iVar9;
  uint uVar10;
  float fStack_cc;
  float local_c4;
  float local_c0;
  float local_bc;
  int iStack_b8;
  int *piStack_b4;
  float fStack_b0;
  float fStack_ac;
  float fStack_a8;
  int iStack_a4;
  float fStack_9c;
  float fStack_98;
  float fStack_8c;
  float fStack_88;
  float fStack_7c;
  float fStack_78;
  float fStack_6c;
  float fStack_68;
  float fStack_60;
  float fStack_5c;
  float fStack_58;
  undefined1 auStack_50 [16];
  undefined4 auStack_40 [4];
  undefined4 auStack_30 [4];
  undefined4 auStack_20 [4];
  undefined4 auStack_10 [4];
  
  if (_DAT_0066c280 < param_1) {
    param_1 = 200.0;
  }
  FUN_005c0f80(1);
  if (*(int *)((int)this + 0xe9c) == 1) {
    iVar4 = FUN_0059ed70();
    iVar9 = *(int *)(iVar4 + 0x184) + 1;
    *(int *)(iVar4 + 0x184) = iVar9;
    if (iVar9 < 0) {
      *(int *)(iVar4 + 0x184) = 0;
    }
    *(undefined4 *)((int)this + 0xe9c) = 0;
    *(undefined4 *)((int)this + 0xea0) = 0;
    local_c4 = 0.0;
    local_c0 = 0.0;
    local_bc = 0.0;
    for (piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this +
                                                       *(int *)((int)this + 0x8e8) * 4 + 8) + 0x70)
                                     + 0xc))(); piVar5 != (int *)0x0;
        piVar5 = (int *)(**(code **)(*piVar5 + 4))()) {
      iVar4 = (**(code **)(*piVar5 + 0xc))();
      for (piVar6 = (int *)(**(code **)(*(int *)(iVar4 + 0x1c) + 0x10))(); piVar6 != (int *)0x0;
          piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
        piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
        bVar2 = FUN_005b69f0((int)piVar7);
        if (CONCAT31(extraout_var,bVar2) == 0) {
          iVar4 = piVar7[0x3b];
          iVar9 = (**(code **)(*piVar7 + 0x24))();
          if ((iVar9 + iVar4 & 3U) == 0) {
            fStack_cc = -_DAT_0066c258;
            pfVar8 = (float *)FUN_005a7490(this,auStack_10,*(int *)((int)this + 0x8e8));
            local_c4 = fStack_cc * *pfVar8;
            local_c0 = pfVar8[1] * fStack_cc;
            local_bc = pfVar8[2] * fStack_cc;
            fStack_8c = local_c0;
            fStack_88 = local_bc;
          }
          else {
            iVar4 = piVar7[0x3b];
            cVar3 = (**(code **)(*piVar7 + 0x24))();
            if ((cVar3 + (char)iVar4 & 3U) == 1) {
              fStack_cc = -_DAT_0066c25c;
              pfVar8 = (float *)FUN_005a7490(this,auStack_20,*(int *)((int)this + 0x8e8));
              local_c4 = fStack_cc * *pfVar8;
              local_c0 = pfVar8[1] * fStack_cc;
              local_bc = pfVar8[2] * fStack_cc;
              fStack_6c = local_c0;
              fStack_68 = local_bc;
            }
            else {
              iVar4 = piVar7[0x3b];
              cVar3 = (**(code **)(*piVar7 + 0x24))();
              if ((cVar3 + (char)iVar4 & 3U) == 2) {
                fStack_cc = -_DAT_0066c260;
                pfVar8 = (float *)FUN_005a7490(this,auStack_30,*(int *)((int)this + 0x8e8));
                local_c4 = fStack_cc * *pfVar8;
                local_c0 = pfVar8[1] * fStack_cc;
                local_bc = pfVar8[2] * fStack_cc;
                fStack_9c = local_c0;
                fStack_98 = local_bc;
              }
              else {
                iVar4 = piVar7[0x3b];
                iVar9 = (**(code **)(*piVar7 + 0x24))();
                if ((iVar9 + iVar4 & 3U) == 0) {
                  fStack_cc = -_DAT_0066c264;
                  pfVar8 = (float *)FUN_005a7490(this,auStack_40,*(int *)((int)this + 0x8e8));
                  local_c4 = fStack_cc * *pfVar8;
                  local_c0 = pfVar8[1] * fStack_cc;
                  local_bc = pfVar8[2] * fStack_cc;
                  fStack_7c = local_c0;
                  fStack_78 = local_bc;
                }
              }
            }
          }
          FUN_005a33b0(&fStack_60,piVar7 + 0xc);
          fStack_b0 = fStack_60 + local_c4;
          piVar7[0x42] = 0;
          iStack_a4 = -0x40800000;
          fStack_ac = fStack_5c + local_c0;
          fStack_a8 = fStack_58 + local_bc;
          FUN_005c1220(piVar7[0x3b]);
          FUN_005a33b0(auStack_50,piVar7 + 4);
          FUN_005c12a0(piVar7[0x3b],&fStack_b0,auStack_50);
          FUN_0059ed70();
          FUN_0059f4d0((int)piVar7);
        }
      }
    }
  }
  piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + *(int *)((int)this + 0x8e8) * 4 + 8) +
                                       0x70) + 0xc))();
  do {
    piStack_b4 = piVar5;
    if (piVar5 == (int *)0x0) {
      fVar1 = param_1 + *(float *)((int)this + 0xea0);
      *(float *)((int)this + 0xea0) = fVar1;
      if (_DAT_0066c248 <= fVar1) {
        for (piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this +
                                                           *(int *)((int)this + 0x8e8) * 4 + 8) +
                                                  0x70) + 0xc))(); piVar5 != (int *)0x0;
            piVar5 = (int *)(**(code **)(*piVar5 + 4))()) {
          iVar4 = (**(code **)(*piVar5 + 0xc))();
          for (piVar6 = (int *)(**(code **)(*(int *)(iVar4 + 0x1c) + 0x10))(); piVar6 != (int *)0x0;
              piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
            iVar4 = (**(code **)(*piVar6 + 0x10))();
            bVar2 = FUN_005b69f0(iVar4);
            if (CONCAT31(extraout_var_01,bVar2) == 0) {
              fStack_b0 = *(float *)(iVar4 + 0x10);
              fStack_ac = *(float *)(iVar4 + 0x14);
              fStack_a8 = *(float *)(iVar4 + 0x18);
              iStack_a4 = *(int *)(iVar4 + 0x1c);
              local_c4 = *(float *)(iVar4 + 0x30);
              local_c0 = *(float *)(iVar4 + 0x34);
              local_bc = *(float *)(iVar4 + 0x38);
              iStack_b8 = *(undefined4 *)(iVar4 + 0x3c);
              FUN_005c12a0(*(uint *)(iVar4 + 0xec),&local_c4,&fStack_b0);
              FUN_0059ed70();
              FUN_0059f4d0(iVar4);
            }
          }
        }
        *(undefined4 *)((int)this + 0x8b4) = 4;
        FUN_005c0f80(0);
        iVar4 = FUN_0059ed70();
        iVar9 = *(int *)(iVar4 + 0x184) + -1;
        *(int *)(iVar4 + 0x184) = iVar9;
        if (iVar9 < 0) {
          *(int *)(iVar4 + 0x184) = 0;
        }
      }
      return *(int *)((int)this + 0x8b4) == 4;
    }
    iVar4 = (**(code **)(*piVar5 + 0xc))();
    for (piVar6 = (int *)(**(code **)(*(int *)(iVar4 + 0x1c) + 0x10))(); piVar6 != (int *)0x0;
        piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
      piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
      bVar2 = FUN_005b69f0((int)piVar7);
      if (CONCAT31(extraout_var_00,bVar2) == 0) {
        if (*(float *)((int)this + 0xea0) == _DAT_0066c254) {
          iVar4 = (**(code **)(*piVar7 + 0x20))();
          iVar9 = 1;
          if (iVar4 == 1) {
            uVar10 = 0x1c;
          }
          else {
            uVar10 = 0x1e;
          }
          this_00 = (void *)FUN_005ba8b0();
          FUN_005ba980(this_00,uVar10,iVar9);
        }
        fVar1 = param_1 + *(float *)((int)this + 0xea0);
        if (_DAT_0066c248 < fVar1) {
          fVar1 = _DAT_0066c248;
        }
        fVar1 = fVar1 * _DAT_0066c26c;
        iVar4 = piVar7[0x3b];
        iVar9 = (**(code **)(*piVar7 + 0x24))();
        if ((iVar9 + iVar4 & 3U) == 0) {
          fStack_cc = fVar1 * fVar1 * _DAT_006bc474 - _DAT_0066c258;
        }
        else {
          iVar4 = piVar7[0x3b];
          cVar3 = (**(code **)(*piVar7 + 0x24))();
          if ((cVar3 + (char)iVar4 & 3U) == 1) {
            fStack_cc = fVar1 * fVar1 * _DAT_006bc490 - _DAT_0066c25c;
          }
          else {
            iVar4 = piVar7[0x3b];
            cVar3 = (**(code **)(*piVar7 + 0x24))();
            if ((cVar3 + (char)iVar4 & 3U) == 2) {
              fStack_cc = fVar1 * fVar1 * _DAT_006bc48c - _DAT_0066c260;
            }
            else {
              iVar4 = piVar7[0x3b];
              cVar3 = (**(code **)(*piVar7 + 0x24))();
              if ((cVar3 + (char)iVar4 & 3U) == 3) {
                fStack_cc = fVar1 * fVar1 * _DAT_006bc480 - _DAT_0066c264;
              }
            }
          }
        }
        iVar4 = FUN_005a7490(this,auStack_40,*(int *)((int)this + 0x8e8));
        local_c0 = (float)piVar7[0xd];
        local_bc = (float)piVar7[0xe];
        local_c4 = (float)piVar7[0xc];
        fStack_cc = *(float *)(iVar4 + 8) * fStack_cc;
        iStack_b8 = piVar7[0xf];
        iVar4 = FUN_005a7490(this,auStack_30,*(int *)((int)this + 0x8e8));
        local_bc = local_bc + fStack_cc;
        if (*(float *)(iVar4 + 8) <= _DAT_0066c284) {
          if (local_bc <= (float)piVar7[0xe]) goto LAB_005a72a4;
        }
        else if ((float)piVar7[0xe] <= local_bc) {
LAB_005a72a4:
          local_bc = (float)piVar7[0xe];
        }
        fStack_b0 = (float)piVar7[4];
        fStack_ac = (float)piVar7[5];
        fStack_a8 = (float)piVar7[6];
        iStack_a4 = piVar7[7];
        iStack_b8 = 0xbf800000;
        FUN_005c12a0(piVar7[0x3b],&local_c4,&fStack_b0);
        FUN_0059ed70();
        FUN_0059f4d0((int)piVar7);
        piVar5 = piStack_b4;
      }
    }
    piVar5 = (int *)(**(code **)(*piVar5 + 4))();
  } while( true );
}

