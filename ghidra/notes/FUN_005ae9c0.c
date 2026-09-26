
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005ae9c0(void *this,float param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  float unaff_ESI;
  bool bVar7;
  float fStack_20;
  uint uStack_1c;
  float fVar8;
  undefined4 uStack_4;
  
  uStack_1c = 0;
  *(float *)this = param_1 + *(float *)this;
  fStack_20 = 8.349092e-39;
  (**(code **)(**(int **)((int)this + 0x24c) + 0x14))();
  fStack_20 = 0.0;
  (**(code **)(**(int **)((int)this + 0x250) + 0x14))();
  iVar2 = (**(code **)(**(int **)((int)this + 0x250) + 0x28))(uStack_4);
  iVar3 = (**(code **)(**(int **)((int)this + 0x24c) + 0x28))(uStack_4);
  (**(code **)(**(int **)((int)this + 0x24c) + 0x14))(0);
  (**(code **)(**(int **)((int)this + 0x250) + 0x14))(0);
  iVar4 = *(int *)((int)this + 0x24c);
  iVar1 = *(int *)((int)this + 0x250);
  fVar8 = *(float *)(iVar1 + 0x3c);
  if (*(float *)((int)this + 0x264) <= *(float *)(iVar4 + 0x3c)) {
    *(float *)((int)this + 0x26c) = unaff_ESI + *(float *)((int)this + 0x26c);
  }
  else {
    *(float *)((int)this + 0x264) = *(float *)(iVar4 + 0x3c);
    *(undefined4 *)((int)this + 0x26c) = 0;
  }
  if (*(float *)((int)this + 0x268) <= fVar8) {
    *(float *)((int)this + 0x270) = unaff_ESI + *(float *)((int)this + 0x270);
  }
  else {
    *(undefined4 *)((int)this + 0x270) = 0;
    *(float *)((int)this + 0x268) = fVar8;
  }
  if ((iVar3 == 0) && (iVar2 == 0)) {
    if (*(float *)((int)this + 0x270) <= *(float *)((int)this + 0x26c)) {
      fVar8 = *(float *)((int)this + 0x270);
    }
    else {
      fVar8 = *(float *)((int)this + 0x26c);
    }
    if (_DAT_0066c3e0 <= fVar8) {
      fVar8 = *(float *)(iVar4 + 0x3c);
      fStack_20 = *(float *)(iVar1 + 0x3c);
      iVar4 = FUN_00596bd0();
      iVar4 = FUN_005a66a0(iVar4);
      if ((iVar4 != 0) && (iVar4 = FUN_005ba420(iVar4), iVar4 != 0)) {
        fVar8 = -1.0;
      }
      if (fStack_20 <= fVar8) {
        iVar4 = (**(code **)(**(int **)((int)this + 0x250) + 0xc))();
        if (iVar4 == 0) {
          piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x250) + 0x58) + 0x10))();
          while (piVar5 != (int *)0x0) {
            piVar6 = (int *)(**(code **)(*piVar5 + 8))();
            piVar5 = (int *)(**(code **)(*piVar5 + 0x10))();
            FUN_005af7a0(this,piVar5);
            piVar5 = piVar6;
          }
          piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x250) + 100) + 0x10))();
          while (piVar5 != (int *)0x0) {
            piVar6 = (int *)(**(code **)(*piVar5 + 8))();
            piVar5 = (int *)(**(code **)(*piVar5 + 0x10))();
            FUN_005af7a0(this,piVar5);
            piVar5 = piVar6;
          }
        }
        else {
          *(undefined4 *)(*(int *)((int)this + 0x250) + 0xb0) = 1;
        }
      }
      if (fVar8 <= fStack_20) {
        iVar4 = (**(code **)(**(int **)((int)this + 0x24c) + 0xc))();
        if (iVar4 != 0) {
          *(undefined4 *)(*(int *)((int)this + 0x24c) + 0xb0) = 1;
          return 0;
        }
        piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x24c) + 0x58) + 0x10))();
        while (piVar5 != (int *)0x0) {
          piVar6 = (int *)(**(code **)(*piVar5 + 8))();
          piVar5 = (int *)(**(code **)(*piVar5 + 0x10))();
          FUN_005af7a0(this,piVar5);
          piVar5 = piVar6;
        }
        piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x24c) + 100) + 0x10))();
        while (piVar5 != (int *)0x0) {
          piVar6 = (int *)(**(code **)(*piVar5 + 8))();
          piVar5 = (int *)(**(code **)(*piVar5 + 0x10))();
          FUN_005af7a0(this,piVar5);
          piVar5 = piVar6;
        }
      }
    }
    return 0;
  }
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x14) = 0;
  *(undefined4 *)((int)this + 0x10) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x18) = 0;
  fStack_20 = 0.0;
  uStack_1c = 0;
  if (*(int *)(iVar4 + 0x84) == 5) {
    *(undefined4 *)((int)this + 0x10) = 1;
  }
  if (*(int *)(iVar1 + 0x84) == 5) {
    *(undefined4 *)((int)this + 0x18) = 1;
  }
  iVar4 = FUN_00596bd0();
  iVar4 = FUN_005a66a0(iVar4);
  if (iVar4 != 0) {
    iVar4 = FUN_00596bd0();
    iVar4 = FUN_005a66a0(iVar4);
    if (*(int *)(iVar4 + 0x78) != 0) {
      uStack_1c = 1;
    }
  }
  for (piVar5 = (int *)(**(code **)(*(int *)((int)this + 0x4c) + 0x10))(); piVar5 != (int *)0x0;
      piVar5 = (int *)(**(code **)(*piVar5 + 8))()) {
    iVar4 = (**(code **)(*piVar5 + 0x10))();
    if (*(int *)(iVar4 + 0x170) == 9) {
      iVar4 = (**(code **)(*piVar5 + 0x10))();
      (&fStack_20)[*(int *)(iVar4 + 0x168)] = 1.4013e-45;
    }
  }
  *(uint *)((int)this + 0x10) = *(uint *)((int)this + 0x10) & (uint)fStack_20;
  *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) & uStack_1c;
  iVar4 = FUN_00596bd0();
  iVar4 = FUN_005a66a0(iVar4);
  if (iVar4 != 0) {
    iVar4 = FUN_00596bd0();
    iVar4 = FUN_005a66a0(iVar4);
    iVar4 = FUN_005ba420(iVar4);
    if (iVar4 != 0) goto LAB_005aedcf;
  }
  bVar7 = false;
  for (piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x250) + 0x58) + 0x10))();
      piVar5 != (int *)0x0; piVar5 = (int *)(**(code **)(*piVar5 + 8))()) {
    if (bVar7) goto LAB_005aedcf;
    iVar4 = (**(code **)(*piVar5 + 0x10))();
    bVar7 = *(int *)(iVar4 + 0x170) == 2;
  }
  if (!bVar7) {
    for (piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x250) + 100) + 0x10))();
        piVar5 != (int *)0x0; piVar5 = (int *)(**(code **)(*piVar5 + 8))()) {
      if (bVar7) goto LAB_005aedcf;
      iVar4 = (**(code **)(*piVar5 + 0x10))();
      bVar7 = *(int *)(iVar4 + 0x170) == 2;
    }
    if ((!bVar7) && (*(int *)((int)this + 0x10) == 0)) {
      *(undefined4 *)((int)this + 0x14) = 1;
    }
  }
LAB_005aedcf:
  bVar7 = false;
  for (piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x24c) + 0x58) + 0x10))();
      piVar5 != (int *)0x0; piVar5 = (int *)(**(code **)(*piVar5 + 8))()) {
    if (bVar7) goto LAB_005aee49;
    iVar4 = (**(code **)(*piVar5 + 0x10))();
    if (*(int *)(iVar4 + 0x170) == 2) {
      bVar7 = true;
    }
  }
  if (!bVar7) {
    piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x24c) + 100) + 0x10))();
    while ((piVar5 != (int *)0x0 && (!bVar7))) {
      iVar4 = (**(code **)(*piVar5 + 0x10))();
      if (*(int *)(iVar4 + 0x170) == 2) {
        bVar7 = true;
      }
      piVar5 = (int *)(**(code **)(*piVar5 + 8))();
    }
  }
LAB_005aee49:
  if ((!bVar7) && (*(int *)((int)this + 0x18) == 0)) {
    *(undefined4 *)((int)this + 0x1c) = 1;
  }
  if ((*(int *)((int)this + 0x1c) != 0) && (*(int *)((int)this + 0x14) != 0)) {
    *(undefined4 *)((int)this + 0x1c) = 0;
    *(undefined4 *)((int)this + 0x14) = 0;
  }
  FUN_005aee90((int)this);
  return 1;
}

