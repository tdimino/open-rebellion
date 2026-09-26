
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005c8780(int *param_1)

{
  float fVar1;
  bool bVar2;
  void *pvVar3;
  int iVar4;
  void *pvVar5;
  int *piVar6;
  undefined4 *puVar7;
  int *piVar8;
  undefined4 uVar9;
  int iVar10;
  float fVar11;
  undefined3 extraout_var;
  float10 fVar12;
  float unaff_retaddr;
  uint uVar13;
  float fStack_c;
  
  (**(code **)(*param_1 + 0x14))(0);
  FUN_005aaf90();
  pvVar3 = (void *)FUN_005a9330();
  fVar11 = _DAT_0066cbb4;
  if ((param_1[0x21] == 2) &&
     (fVar1 = (float)param_1[0x1f], param_1[0x1f] = (int)(fVar1 - unaff_retaddr),
     fVar1 - unaff_retaddr <= fVar11)) {
    param_1[0x21] = 3;
    if (param_1[4] != 0) {
      return 0;
    }
    iVar4 = (**(code **)(*param_1 + 4))();
    if (((iVar4 == 1) && (pvVar3 != (void *)0x0)) && (iVar4 = FUN_005ba420((int)pvVar3), iVar4 != 0)
       ) {
      iVar4 = FUN_005aaf90();
      if (*(int *)(iVar4 + 0x9ec) == 1) {
        pvVar5 = (void *)FUN_0059ed70();
        FUN_005a0820(pvVar5);
        FUN_005c0fa0();
      }
      uVar9 = *(undefined4 *)((int)pvVar3 + 0x18);
      piVar6 = (int *)FUN_005aaf90();
      (**(code **)(*piVar6 + 0x28))(0x45,uVar9,0,1);
    }
  }
  if (param_1[4] == 0) {
    (**(code **)(*param_1 + 8))();
    iVar4 = FUN_005aaf90();
    if (_DAT_0066cbb4 <= (float)*(uint *)(iVar4 + 0x87c) - _DAT_0066cba8) {
      iVar4 = FUN_005aaf90();
      fStack_c = (float)*(uint *)(iVar4 + 0x87c) - _DAT_0066cba8;
    }
    else {
      fStack_c = 0.0;
    }
    piVar6 = param_1 + 0x22;
    puVar7 = (undefined4 *)FUN_005f5060((int)piVar6);
    while ((puVar7 != (undefined4 *)0x0 && ((float)puVar7[6] < fStack_c))) {
      FUN_005f4fa0(piVar6,(int)puVar7);
      if (puVar7 != (undefined4 *)0x0) {
        (**(code **)*puVar7)(1);
      }
      puVar7 = (undefined4 *)FUN_005f5060((int)piVar6);
    }
    iVar4 = (**(code **)(*param_1 + 4))();
    if ((iVar4 == 1) && (pvVar3 != (void *)0x0)) {
      FUN_005ba7f0(pvVar3,unaff_retaddr);
    }
    if (param_1[0x21] == 3) {
      bVar2 = false;
      for (piVar6 = (int *)(**(code **)(param_1[0x16] + 0x10))(); piVar6 != (int *)0x0;
          piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
        piVar8 = (int *)(**(code **)(*piVar6 + 0x10))();
        iVar4 = FUN_005b1b70(piVar8);
        if (iVar4 == 1) {
          uVar9 = (**(code **)(*piVar6 + 0x10))();
          FUN_005aaf90();
          iVar4 = FUN_005ad870(uVar9);
          if (((iVar4 == 1) ||
              (iVar4 = (**(code **)(*piVar6 + 0x10))(), *(int *)(iVar4 + 0x170) == 8)) ||
             (iVar4 = (**(code **)(*piVar6 + 0x10))(), *(int *)(iVar4 + 0x170) == 7)) {
            bVar2 = true;
            break;
          }
        }
      }
      for (piVar6 = (int *)(**(code **)(param_1[0x19] + 0x10))(); piVar6 != (int *)0x0;
          piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
        if (bVar2) goto LAB_005c8a8f;
        piVar8 = (int *)(**(code **)(*piVar6 + 0x10))();
        iVar4 = FUN_005b1b70(piVar8);
        if (iVar4 == 1) {
          uVar9 = (**(code **)(*piVar6 + 0x10))();
          FUN_005aaf90();
          iVar4 = FUN_005ad870(uVar9);
          if (((iVar4 == 1) ||
              (iVar4 = (**(code **)(*piVar6 + 0x10))(), *(int *)(iVar4 + 0x170) == 8)) ||
             (iVar4 = (**(code **)(*piVar6 + 0x10))(), *(int *)(iVar4 + 0x170) == 7)) {
            bVar2 = true;
            break;
          }
        }
      }
      if (!bVar2) {
        param_1[0x21] = 4;
        for (piVar6 = (int *)(**(code **)(param_1[0x16] + 0x10))(); piVar6 != (int *)0x0;
            piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
          piVar8 = (int *)(**(code **)(*piVar6 + 0x10))();
          iVar4 = FUN_005b1b70(piVar8);
          if (iVar4 == 0) {
            piVar8 = (int *)FUN_005aaf90();
            iVar4 = *piVar8;
            uVar9 = (**(code **)(*piVar6 + 4))(0,1);
            (**(code **)(iVar4 + 0x28))(0x2f,uVar9);
          }
        }
        for (piVar6 = (int *)(**(code **)(param_1[0x19] + 0x10))(); piVar6 != (int *)0x0;
            piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
          piVar8 = (int *)(**(code **)(*piVar6 + 0x10))();
          iVar4 = FUN_005b1b70(piVar8);
          if (iVar4 == 0) {
            piVar8 = (int *)FUN_005aaf90();
            iVar4 = *piVar8;
            uVar9 = (**(code **)(*piVar6 + 4))(0,1);
            (**(code **)(iVar4 + 0x28))(0x2f,uVar9);
          }
        }
      }
    }
LAB_005c8a8f:
    fVar11 = _DAT_0066cbb4;
    if (param_1[0x21] == 0) {
      if (param_1[2] == 1) {
        fVar1 = (float)param_1[0x10] - _DAT_0066cbd4;
        param_1[0x10] = (int)fVar1;
        if (fVar1 <= fVar11) {
          if (param_1[5] == 0) {
            param_1[5] = 1;
            (**(code **)(*param_1 + 0x18))();
          }
          param_1[0x10] = DAT_0066cba4;
          FUN_005c9200(param_1);
          (**(code **)(*param_1 + 0x14))(0);
          iVar4 = (**(code **)(*param_1 + 4))();
          iVar10 = FUN_005aaf90();
          (**(code **)(**(int **)(iVar10 + 8 + (uint)(iVar4 == 0) * 4) + 0x14))(0);
          iVar4 = (**(code **)(*param_1 + 4))();
          pvVar5 = (void *)FUN_005aaf90();
          fVar12 = FUN_005a9610(pvVar5,iVar4);
          if ((((float10)_DAT_0066cbb4 < fVar12) &&
              (((float10)(float)param_1[0xf] / fVar12 < (float10)(float)param_1[9] ||
               ((float10)(float)param_1[0xf] / fVar12 == (float10)_DAT_0066cbb4)))) &&
             (iVar4 = (**(code **)(*param_1 + 0xc))(), iVar4 != 0)) {
            iVar4 = FUN_005aaf90();
            iVar4 = *(int *)(iVar4 + 0x9ec);
            iVar10 = (**(code **)(*param_1 + 4))();
            if (((iVar4 != iVar10) || (param_1[2] != 1)) && (iVar4 = FUN_005d0ae0(), iVar4 == 0)) {
              fVar11 = (float)(**(code **)(*param_1 + 4))();
              pvVar5 = (void *)FUN_005aaf90();
              FUN_005ad530(pvVar5,fVar11);
            }
          }
          if ((pvVar3 != (void *)0x0) && (iVar4 = FUN_005ba420((int)pvVar3), iVar4 != 0)) {
            iVar4 = (**(code **)(*param_1 + 4))();
            if (iVar4 == 1) {
              if ((*(float *)((int)pvVar3 + 0x6c) == _DAT_0066cbd8) &&
                 (iVar4 = FUN_005f5060((int)(param_1 + 0x24)), iVar4 != 0)) {
                do {
                  iVar10 = (**(code **)(**(int **)(iVar4 + 0x1c) + 0x20))();
                  if (iVar10 == 0) {
                    uVar9 = *(undefined4 *)(iVar4 + 0x1c);
                    FUN_005aaf90();
                    iVar10 = FUN_005ad870(uVar9);
                    if (iVar10 != 0) {
                      FUN_005ba720(pvVar3,*(undefined4 *)(*(int *)(iVar4 + 0x1c) + 0xec));
                      return 1;
                    }
                  }
                  iVar4 = *(int *)(iVar4 + 0x10);
                } while (iVar4 != 0);
                return 1;
              }
            }
            else if ((param_1[6] == 0) && (iVar4 = FUN_005d0ae0(), iVar4 == 0)) {
              iVar4 = FUN_005aaf90();
              bVar2 = FUN_005ad930(iVar4);
              if (CONCAT31(extraout_var,bVar2) == 0) {
                iVar4 = (**(code **)(param_1[0x19] + 0x10))();
                if ((iVar4 == 0) && (iVar4 = (**(code **)(*param_1 + 0xc))(), iVar4 != 0)) {
                  fVar11 = (float)(**(code **)(*param_1 + 4))();
                  pvVar3 = (void *)FUN_005aaf90();
                  FUN_005ad530(pvVar3,fVar11);
                  return 1;
                }
              }
              else {
                piVar6 = (int *)(**(code **)(param_1[0x19] + 0x10))();
                if (piVar6 != (int *)0x0) {
                  iVar4 = (**(code **)(*piVar6 + 0x10))();
                  uVar13 = *(uint *)(iVar4 + 0x174);
                  pvVar3 = (void *)FUN_00596bd0();
                  piVar6 = (int *)FUN_005a6480(pvVar3,uVar13);
                  if ((piVar6 != (int *)0x0) && (piVar6[0x1a] != 6)) {
                    param_1[6] = 1;
                    (**(code **)(*piVar6 + 100))(6);
                    return 1;
                  }
                }
              }
            }
          }
        }
      }
      else {
        pvVar3 = (void *)FUN_0059ed70();
        FUN_005a0820(pvVar3);
      }
      return 1;
    }
  }
  return 0;
}

