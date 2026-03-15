
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b2140(void *this,uint param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  int *piVar4;
  void *this_00;
  int iVar5;
  int iVar6;
  float10 fVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  uint uVar10;
  
  if (*(int *)((int)this + 0x124) != 0) {
    return;
  }
  if (*(int *)((int)this + 0x3ac) == 1) {
    return;
  }
  if (*(float *)((int)this + 0x3b8) == _DAT_0066c4f0) {
    return;
  }
  if (param_1 == 0) {
    if (*(int *)((int)this + 0x380) == 0) {
      return;
    }
    uVar9 = *(undefined4 *)((int)this + 0xec);
    piVar4 = (int *)FUN_005aaf90();
    param_1 = *(uint *)((int)this + 0x380);
    iVar5 = *piVar4;
    uVar8 = 0x29;
  }
  else {
    uVar10 = param_1;
    this_00 = (void *)FUN_00596bd0();
    iVar5 = FUN_005a6340(this_00,uVar10);
    if (iVar5 == 0) {
      return;
    }
    if (*(uint *)((int)this + 0x380) == param_1) {
      iVar6 = iVar5;
      FUN_005aaf90();
      iVar6 = FUN_005ad870(iVar6);
      if ((iVar6 != 0) && (fVar7 = FUN_005b1790(this), fVar7 != (float10)_DAT_0066c4f0)) {
                    /* WARNING: Load size is inaccurate */
        fVar1 = *(float *)((int)this + 0x38) - *(float *)(iVar5 + 0x38);
        fVar3 = *(float *)((int)this + 0x34) - *(float *)(iVar5 + 0x34);
        fVar2 = *(float *)((int)this + 0x30) - *(float *)(iVar5 + 0x30);
        fVar7 = (float10)(**(code **)(*this + 0x5c))();
        if ((float10)SQRT(fVar2 * fVar2 + fVar1 * fVar1 + fVar3 * fVar3) <= fVar7) {
          return;
        }
        uVar9 = *(undefined4 *)((int)this + 0xec);
        piVar4 = (int *)FUN_005aaf90();
        param_1 = *(uint *)((int)this + 0x380);
        iVar5 = *piVar4;
        uVar8 = 0x29;
        goto LAB_005b231e;
      }
    }
    else if (*(uint *)((int)this + 0x380) == 0) {
      fVar7 = FUN_005b1790(this);
      if (fVar7 == (float10)_DAT_0066c4f0) {
        return;
      }
                    /* WARNING: Load size is inaccurate */
      fVar1 = *(float *)((int)this + 0x38) - *(float *)(iVar5 + 0x38);
      fVar3 = *(float *)((int)this + 0x34) - *(float *)(iVar5 + 0x34);
      fVar2 = *(float *)((int)this + 0x30) - *(float *)(iVar5 + 0x30);
      fVar7 = (float10)(**(code **)(*this + 0x5c))();
      if (fVar7 < (float10)SQRT(fVar2 * fVar2 + fVar1 * fVar1 + fVar3 * fVar3)) {
        return;
      }
      uVar9 = *(undefined4 *)((int)this + 0xec);
      piVar4 = (int *)FUN_005aaf90();
      iVar5 = *piVar4;
      uVar8 = 0x28;
      goto LAB_005b231e;
    }
    uVar9 = *(undefined4 *)((int)this + 0xec);
    piVar4 = (int *)FUN_005aaf90();
    param_1 = *(uint *)((int)this + 0x380);
    iVar5 = *piVar4;
    uVar8 = 0x29;
  }
LAB_005b231e:
  (**(code **)(iVar5 + 0x28))(uVar8,uVar9,param_1,1);
  *(undefined4 *)((int)this + 0x3ac) = 1;
  return;
}

