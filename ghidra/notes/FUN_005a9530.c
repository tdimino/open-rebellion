
undefined4 __thiscall FUN_005a9530(void *this,float *param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  int *piVar4;
  int iVar5;
  undefined4 uVar6;
  float fStack_4;
  
  piVar4 = (int *)(**(code **)(*(int *)((int)this + 0x9dc) + 0xc))();
  uVar6 = 0;
  fStack_4 = 3.4028235e+38;
  for (; piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 0xc))()) {
    iVar5 = (**(code **)(*piVar4 + 8))();
    fVar1 = *(float *)(iVar5 + 0x4c) - param_1[2];
    fVar2 = *(float *)(iVar5 + 0x48) - param_1[1];
    fVar3 = *(float *)(iVar5 + 0x44) - *param_1;
    fVar1 = fVar3 * fVar3 + fVar1 * fVar1 + fVar2 * fVar2;
    if (fVar1 < fStack_4) {
      uVar6 = (**(code **)(*piVar4 + 8))();
      fStack_4 = fVar1;
    }
  }
  return uVar6;
}

