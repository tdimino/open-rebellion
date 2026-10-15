
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005d14d0(int *param_1,float param_2)

{
  int iVar1;
  void *this;
  int iVar2;
  int *piVar3;
  int *piVar4;
  void *pvVar5;
  int *piVar6;
  float10 fVar7;
  uint uVar8;
  float fVar9;
  
  FUN_00596f30();
  iVar1 = FUN_00596bd0();
  this = (void *)FUN_005a66a0(iVar1);
  iVar1 = (**(code **)(*param_1 + 4))();
  if (iVar1 == 0) {
    if ((this != (void *)0x0) && (iVar1 = FUN_005ba450((int)this), iVar1 == 1)) {
      iVar1 = FUN_00596f30();
      iVar1 = FUN_005c5eb0((int *)(*(int *)(iVar1 + 0x250) + 100));
      iVar2 = FUN_005c5eb0(param_1 + 0x19);
      if (iVar1 < iVar2) {
        for (piVar3 = (int *)(**(code **)(param_1[0x1c] + 0xc))(); piVar3 != (int *)0x0;
            piVar3 = (int *)(**(code **)(*piVar3 + 4))()) {
          piVar4 = (int *)(**(code **)(*piVar3 + 0xc))();
          iVar1 = (**(code **)(*piVar4 + 4))();
          if (iVar1 == 4) {
            iVar1 = (**(code **)(*piVar3 + 0xc))();
            pvVar5 = (void *)FUN_00596f30();
            FUN_005aefc0(pvVar5,iVar1);
            break;
          }
        }
      }
    }
  }
  else {
    if ((this != (void *)0x0) && (iVar1 = FUN_005ba420((int)this), iVar1 != 0)) {
      if (*(float *)((int)this + 0x6c) != _DAT_0066d0f4) {
        FUN_005ba7f0(this,param_2);
      }
      if (*(float *)((int)this + 0x6c) == _DAT_0066d0f4) {
        (**(code **)(*param_1 + 8))();
        for (iVar1 = FUN_005f5060((int)(param_1 + 0x24)); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)
            ) {
          iVar2 = (**(code **)(**(int **)(iVar1 + 0x1c) + 0x20))();
          if (iVar2 == 0) {
            FUN_005ba710((int)this);
            piVar3 = *(int **)(iVar1 + 0x1c);
            pvVar5 = (void *)FUN_00596f30();
            FUN_005af7a0(pvVar5,piVar3);
            break;
          }
        }
      }
    }
    iVar1 = FUN_00596f30();
    piVar3 = (int *)(**(code **)(*(int *)(iVar1 + 0x58) + 0x10))();
    if (piVar3 != (int *)0x0) {
      do {
        iVar1 = (**(code **)(*piVar3 + 0x10))();
        uVar8 = *(uint *)(iVar1 + 0x164);
        pvVar5 = (void *)FUN_00596bd0();
        piVar4 = FUN_005a6360(pvVar5,uVar8);
        if (piVar4 != (int *)0x0) {
          iVar1 = (**(code **)(*piVar3 + 0x10))();
          FUN_005b8450(piVar4,iVar1);
        }
        iVar1 = (**(code **)(*piVar3 + 0x10))();
        *(undefined4 *)(iVar1 + 0x170) = 2;
        iVar1 = (**(code **)(*piVar3 + 0x10))();
        FUN_005c5f20(param_1 + 0x19,iVar1);
        piVar3 = (int *)(**(code **)(*piVar3 + 8))();
      } while (piVar3 != (int *)0x0);
    }
    iVar1 = FUN_00596f30();
    (**(code **)(*(int *)(iVar1 + 0x58) + 4))();
  }
  (**(code **)(*param_1 + 8))();
  FUN_005c9200(param_1);
  for (piVar3 = (int *)(**(code **)(param_1[0x1c] + 0xc))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 4))()) {
    iVar1 = (**(code **)(*piVar3 + 0xc))();
    piVar4 = (int *)FUN_005d2190((int *)(iVar1 + 0x8c));
    if (piVar4 != (int *)0x0) {
      piVar6 = piVar4;
      pvVar5 = (void *)(**(code **)(*piVar3 + 0xc))();
      fVar7 = FUN_005cb650(pvVar5,piVar6);
      FUN_005b54d0(piVar4,(float)(fVar7 * (float10)param_2));
      if ((float)piVar4[0xf3] == _DAT_0066d0f8) {
        pvVar5 = (void *)FUN_00596f30();
        FUN_005af7a0(pvVar5,piVar4);
      }
    }
  }
  iVar1 = (**(code **)(*param_1 + 0x2c))();
  if (iVar1 == 1) {
    piVar3 = (int *)(**(code **)(param_1[0x19] + 0x10))();
    while (piVar4 = piVar3, piVar4 != (int *)0x0) {
      piVar3 = (int *)(**(code **)(*piVar4 + 8))();
      piVar6 = (int *)(**(code **)(*piVar4 + 0x10))();
      iVar1 = FUN_005b1b70(piVar6);
      if (iVar1 == 0) {
        iVar1 = (**(code **)(*piVar4 + 0x10))();
        pvVar5 = (void *)FUN_00596f30();
        iVar1 = FUN_005afe40(pvVar5,iVar1);
        if (iVar1 == 0) {
          piVar4 = (int *)(**(code **)(*piVar4 + 0x10))();
          pvVar5 = (void *)FUN_00596f30();
          FUN_005af7a0(pvVar5,piVar4);
        }
      }
      else {
        piVar4 = (int *)(**(code **)(*piVar4 + 0x10))();
        pvVar5 = (void *)FUN_00596f30();
        FUN_005af910(pvVar5,piVar4);
      }
    }
    iVar1 = (**(code **)(*param_1 + 4))();
    if (((iVar1 == 1) && (this != (void *)0x0)) && (iVar1 = FUN_005ba420((int)this), iVar1 != 0)) {
      *(undefined4 *)((int)this + 0x78) = 1;
    }
    piVar3 = (int *)(**(code **)(param_1[0x16] + 0x10))();
    while (piVar3 != (int *)0x0) {
      piVar4 = (int *)(**(code **)(*piVar3 + 8))();
      piVar6 = (int *)(**(code **)(*piVar3 + 0x10))();
      iVar1 = FUN_005b1b70(piVar6);
      if (iVar1 == 1) {
        piVar3 = (int *)(**(code **)(*piVar3 + 0x10))();
        pvVar5 = (void *)FUN_00596f30();
        FUN_005af910(pvVar5,piVar3);
        piVar3 = piVar4;
      }
      else {
        piVar3 = (int *)(**(code **)(*piVar3 + 0x10))();
        pvVar5 = (void *)FUN_00596f30();
        FUN_005af7a0(pvVar5,piVar3);
        piVar3 = piVar4;
      }
    }
    param_1[0x21] = 5;
  }
  iVar1 = 0;
  for (piVar3 = (int *)(**(code **)(param_1[0x16] + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar2 = (**(code **)(*piVar3 + 0x10))();
    if (*(int *)(iVar2 + 0x170) == 2) {
      iVar1 = iVar1 + 1;
    }
    fVar9 = param_2;
    pvVar5 = (void *)(**(code **)(*piVar3 + 0x10))();
    FUN_005b6320(pvVar5,fVar9);
  }
  for (piVar3 = (int *)(**(code **)(param_1[0x19] + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar2 = (**(code **)(*piVar3 + 0x10))();
    if (*(int *)(iVar2 + 0x170) == 2) {
      iVar1 = iVar1 + 1;
    }
    fVar9 = param_2;
    pvVar5 = (void *)(**(code **)(*piVar3 + 0x10))();
    FUN_005b6320(pvVar5,fVar9);
  }
  if (iVar1 == 0) {
    iVar1 = (**(code **)(*param_1 + 4))();
    if ((iVar1 != 1) || (this == (void *)0x0)) {
      return 1;
    }
    iVar1 = FUN_005ba420((int)this);
    if (iVar1 == 0) {
      return 1;
    }
  }
  return 0;
}

