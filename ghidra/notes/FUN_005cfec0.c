
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005cfec0(void *this,uint param_1)

{
  float fVar1;
  int iVar2;
  int *piVar3;
  void *pvVar4;
  uint uVar5;
  undefined4 *puVar6;
  uint uVar7;
  int *piVar8;
  float *pfVar9;
  int *piVar10;
  
  piVar10 = (int *)0x0;
  iVar2 = *(int *)((int)this + 0x2c);
  *(uint *)((int)this + 0x2c) = param_1;
  *(undefined4 *)((int)this + 0x34) = 0;
  switch(param_1) {
  default:
    goto switchD_005cfee1_caseD_2;
  case 5:
    break;
  case 6:
    uVar7 = FUN_005c5eb0((int *)(*(int *)((int)this + 8) + 0x1c));
    uVar5 = 0;
    if (1 < uVar7) {
      if ((uVar7 & 0xfffffffe) < 3) {
        uVar5 = 1;
      }
      else {
        uVar5 = uVar7 >> 1;
      }
    }
    piVar3 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 8) + 0x1c) + 0x10))();
    while (piVar3 != (int *)0x0) {
      piVar10 = (int *)(**(code **)(*piVar3 + 8))();
      if (uVar5 == 0) {
        iVar2 = (**(code **)(*piVar3 + 0x10))();
        *(undefined4 *)(iVar2 + 0x3cc) = *(undefined4 *)(iVar2 + 0x3d0);
        *(undefined4 *)(iVar2 + 0x3c0) = *(undefined4 *)(iVar2 + 0x3c4);
        iVar2 = (**(code **)(*piVar3 + 0x10))();
        *(undefined4 *)(iVar2 + 0x170) = 2;
        piVar8 = (int *)(**(code **)(*piVar3 + 0x10))();
        pvVar4 = (void *)FUN_005aaf90();
        FUN_005a9380(pvVar4,piVar8);
        iVar2 = (**(code **)(*piVar3 + 0x10))();
        *(undefined4 *)(iVar2 + 0x108) = 0;
        iVar2 = (**(code **)(*piVar3 + 0x10))();
        FUN_0059ed70();
        FUN_0059f4d0(iVar2);
        uVar5 = 0;
        piVar3 = piVar10;
      }
      else {
        uVar5 = uVar5 - 1;
        piVar8 = (int *)(**(code **)(*piVar3 + 0x10))();
        (**(code **)(*piVar8 + 0xc0))(0x47c35000);
        piVar3 = (int *)(**(code **)(*piVar3 + 0x10))();
        (**(code **)(*piVar3 + 0x1c))();
        piVar3 = piVar10;
      }
    }
    FUN_005aaf90();
    piVar3 = (int *)FUN_005a9330();
    if (piVar3 != piVar10) {
      FUN_005ba890((int)piVar3);
      iVar2 = FUN_005aaf90();
      if (*(int *)(iVar2 + 0x9ec) == 1) {
        FUN_005c0fa0();
      }
      FUN_005c1200(piVar3[6]);
    }
    iVar2 = FUN_005aaf90();
    if (*(int **)(iVar2 + 0x9ec) == piVar10) {
      iVar2 = (**(code **)(**(int **)((int)this + 8) + 4))();
      if (iVar2 == 4) {
        FUN_005c7150(*(int *)((int)this + 8));
      }
      puVar6 = (undefined4 *)FUN_005aaf90();
      (**(code **)*puVar6)();
      iVar2 = FUN_005ba8b0();
      piVar3 = *(int **)(iVar2 + 0x18);
    }
    else {
      puVar6 = (undefined4 *)FUN_005aaf90();
      (**(code **)*puVar6)();
      iVar2 = FUN_005ba8b0();
      piVar3 = *(int **)(iVar2 + 0x18);
    }
    if (piVar3 == piVar10) {
      FUN_005c0fb0();
    }
    else {
      iVar2 = FUN_005ba8b0();
      *(int **)(iVar2 + 0x18) = piVar10;
      FUN_005c0fb0();
      iVar2 = FUN_005ba8b0();
      *(undefined4 *)(iVar2 + 0x18) = 1;
    }
    if ((*(int **)(*(int *)((int)this + 8) + 0x34) == piVar10) &&
       (iVar2 = FUN_005aaf90(), *(int **)(iVar2 + 0x9ec) == piVar10)) {
      iVar2 = *(int *)((int)this + 8);
      pvVar4 = (void *)FUN_0059ed70();
      FUN_0059f680(pvVar4,iVar2);
    }
    (**(code **)(**(int **)((int)this + 8) + 0x48))(piVar10);
    return;
  case 7:
    iVar2 = FUN_005aaf90();
    if (*(int *)(iVar2 + 0x9ec) == 0) {
      iVar2 = (**(code **)(**(int **)((int)this + 8) + 4))();
      if (iVar2 == 4) {
        iVar2 = FUN_005c7150(*(int *)((int)this + 8));
        if (iVar2 == 0) {
          uVar5 = 0x13b;
        }
        else if (iVar2 == 1) {
          uVar5 = 0x12f;
        }
        else {
          uVar5 = (-(uint)(iVar2 != 2) & 4) + 0x133;
        }
        iVar2 = 1;
        pvVar4 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar4,uVar5,iVar2);
      }
      puVar6 = (undefined4 *)FUN_005aaf90();
      (**(code **)*puVar6)();
      iVar2 = FUN_005ba8b0();
      iVar2 = *(int *)(iVar2 + 0x18);
    }
    else {
      puVar6 = (undefined4 *)FUN_005aaf90();
      (**(code **)*puVar6)();
      iVar2 = FUN_005ba8b0();
      iVar2 = *(int *)(iVar2 + 0x18);
    }
    if (iVar2 == 0) {
      FUN_005c0ff0();
    }
    else {
      iVar2 = FUN_005ba8b0();
      *(undefined4 *)(iVar2 + 0x18) = 0;
      FUN_005c0ff0();
      iVar2 = FUN_005ba8b0();
      *(undefined4 *)(iVar2 + 0x18) = 1;
    }
    iVar2 = FUN_005c5eb0((int *)(*(int *)((int)this + 8) + 0x1c));
    if (0 < iVar2) {
      do {
        piVar10 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 8) + 0x1c) + 0x10))();
        piVar3 = (int *)(**(code **)(*piVar10 + 0x10))();
        (**(code **)(*piVar3 + 0xc0))(0x47c35000);
        piVar10 = (int *)(**(code **)(*piVar10 + 0x10))();
        (**(code **)(*piVar10 + 0x1c))();
        iVar2 = iVar2 + -1;
      } while (iVar2 != 0);
      return;
    }
    return;
  case 8:
    if ((iVar2 != 2) && (iVar2 != 1)) {
      for (piVar10 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 8) + 0x1c) + 0x10))();
          piVar10 != (int *)0x0; piVar10 = (int *)(**(code **)(*piVar10 + 8))()) {
        iVar2 = (**(code **)(*piVar10 + 0x10))();
        *(undefined4 *)(iVar2 + 0x3cc) = *(undefined4 *)(iVar2 + 0x3d0);
        *(undefined4 *)(iVar2 + 0x3c0) = *(undefined4 *)(iVar2 + 0x3c4);
        iVar2 = (**(code **)(*piVar10 + 0x10))();
        *(undefined4 *)(iVar2 + 0x170) = 2;
        piVar3 = (int *)(**(code **)(*piVar10 + 0x10))();
        pvVar4 = (void *)FUN_005aaf90();
        FUN_005a9380(pvVar4,piVar3);
        iVar2 = (**(code **)(*piVar10 + 0x10))();
        *(undefined4 *)(iVar2 + 0x108) = 0;
        iVar2 = (**(code **)(*piVar10 + 0x10))();
        FUN_0059ed70();
        FUN_0059f4d0(iVar2);
      }
    }
    (**(code **)(**(int **)((int)this + 8) + 0x48))(3);
    return;
  }
  FUN_005d08d0((int)this);
  *(undefined4 *)((int)this + 0x30) = DAT_0066d080;
  iVar2 = FUN_005aaf90();
  if (*(int *)(iVar2 + 0x9ec) == 0) {
    *(undefined4 *)((int)this + 0x80) = 0;
    param_1 = 0;
    pfVar9 = (float *)((int)this + 0x38);
    do {
      fVar1 = (float)(int)param_1;
      param_1 = param_1 + 1;
      *pfVar9 = DAT_0066d084 - fVar1 * DAT_0066d084 * _DAT_0066d09c;
      pfVar9 = pfVar9 + 1;
    } while (param_1 < 9);
    iVar2 = (**(code **)(**(int **)((int)this + 8) + 4))();
    if (iVar2 == 4) {
      iVar2 = FUN_005c7150(*(int *)((int)this + 8));
      if (iVar2 == 0) {
        uVar5 = 0x13a;
      }
      else if (iVar2 == 1) {
        uVar5 = 0x12e;
      }
      else {
        uVar5 = (-(uint)(iVar2 != 2) & 4) + 0x132;
      }
      goto LAB_005d0031;
    }
  }
  else {
    uVar5 = 0x11d;
LAB_005d0031:
    iVar2 = 1;
    pvVar4 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar4,uVar5,iVar2);
  }
  if (*(int *)(*(int *)((int)this + 8) + 0x34) == 0) {
    FUN_005d04e0((int)this);
    return;
  }
switchD_005cfee1_caseD_2:
  return;
}

