
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005ad530(void *this,float param_1)

{
  float fVar1;
  void *pvVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  bool bVar6;
  float10 fVar7;
  uint uVar8;
  int iVar9;
  
  fVar1 = param_1;
  *(undefined4 *)(*(int *)((int)this + (int)param_1 * 4 + 8) + 0x84) = 2;
  if (*(float *)((int)this + 0x9ec) == param_1) {
    iVar9 = 1;
    if (*(float *)((int)this + 0x9ec) == 0.0) {
      uVar8 = 0x8f;
    }
    else {
      uVar8 = 0x10d;
    }
    pvVar2 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar2,uVar8,iVar9);
  }
  iVar9 = *(int *)((int)this + (int)param_1 * 4 + 8);
  if (*(int *)(iVar9 + 0x10) == 0) {
    for (piVar3 = (int *)(**(code **)(*(int *)(iVar9 + 0x70) + 0xc))(); piVar3 != (int *)0x0;
        piVar3 = (int *)(**(code **)(*piVar3 + 4))()) {
      piVar4 = (int *)(**(code **)(*piVar3 + 0xc))();
      iVar9 = FUN_005c6070(piVar4 + 7);
      iVar5 = (**(code **)(*piVar4 + 4))();
      if (iVar5 == 4) {
        if (param_1 == *(float *)((int)this + 0x9ec)) {
          uVar8 = piVar4[6];
          pvVar2 = (void *)FUN_0059ed70();
          FUN_005a0320(pvVar2,uVar8);
        }
        if (iVar9 != 0) {
          if (piVar4[0x19] == 6) {
            (**(code **)(*(int *)this + 0x28))(0x3c,piVar4[6],8,1);
          }
          fVar7 = FUN_005b1bc0(iVar9);
          if (fVar7 != (float10)_DAT_0066c284) goto LAB_005ad609;
          (**(code **)(*piVar4 + 100))(2);
        }
      }
      else {
LAB_005ad609:
        (**(code **)(*piVar4 + 100))(3);
      }
    }
    for (piVar3 = (int *)(**(code **)(*(int *)(*(int *)((int)this + (int)param_1 * 4 + 8) + 0x58) +
                                     0x10))(); piVar3 != (int *)0x0;
        piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
      iVar9 = (**(code **)(*piVar3 + 0x10))();
      if (*(int *)(*(int *)((int)this + (int)param_1 * 4 + 8) + 0x10) == 0) {
        uVar8 = *(uint *)(iVar9 + 0x174);
        if (uVar8 != 0) {
          pvVar2 = (void *)FUN_00596bd0();
          piVar4 = (int *)FUN_005a6480(pvVar2,uVar8);
          if (piVar4 != (int *)0x0) {
            iVar5 = iVar9;
            pvVar2 = (void *)FUN_0059ed70();
            iVar5 = FUN_005a0c50(pvVar2,iVar5);
            if ((iVar5 == 0) && (piVar4[0x14] == *(int *)((int)this + 0x9ec))) {
              iVar5 = piVar4[6];
              pvVar2 = (void *)FUN_0059ed70();
              FUN_005a0410(pvVar2,iVar5);
            }
            FUN_005a8d30(this,iVar9,piVar4);
          }
        }
        piVar4 = FUN_005aded0(this,iVar9);
        (**(code **)(*piVar4 + 100))(3);
      }
    }
  }
  bVar6 = param_1 == 1.4013e-45;
  param_1 = *(float *)(*(int *)((int)this + (int)param_1 * 4 + 8) + 0x7c);
  if ((bVar6) && (iVar9 = FUN_005a9330(), iVar9 != 0)) {
    iVar9 = FUN_005a9330();
    iVar9 = FUN_005ba420(iVar9);
    if (iVar9 != 0) {
      if (param_1 <= DAT_0066c234) {
        *(float *)(*(int *)((int)this + 0xc) + 0x7c) = DAT_0066c234;
        return;
      }
      goto LAB_005ad73c;
    }
  }
  if (param_1 <= DAT_0066c240) {
    param_1 = DAT_0066c240;
  }
LAB_005ad73c:
  *(float *)(*(int *)((int)this + (int)fVar1 * 4 + 8) + 0x7c) = param_1;
  return;
}

