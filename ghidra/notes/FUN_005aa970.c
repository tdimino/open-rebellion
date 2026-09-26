
int __thiscall
FUN_005aa970(void *this,int param_1,float param_2,float param_3,float param_4,undefined4 param_5,
            int param_6)

{
  float fVar1;
  float fVar2;
  float fVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  int iVar8;
  undefined4 local_c;
  
  local_c = 3.4028235e+38;
  piVar4 = (int *)(**(code **)(*(int *)((int)this + 0x988) + 0x10))();
  iVar8 = 0;
  for (; piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
    iVar5 = (**(code **)(*piVar4 + 0x10))();
    iVar6 = FUN_005ad870(iVar5);
    if (((iVar6 != 0) &&
        (fVar1 = param_4 - *(float *)(iVar5 + 0x38), fVar2 = param_3 - *(float *)(iVar5 + 0x34),
        fVar3 = param_2 - *(float *)(iVar5 + 0x30),
        fVar1 = fVar3 * fVar3 + fVar1 * fVar1 + fVar2 * fVar2, fVar1 < local_c)) &&
       (iVar6 = (**(code **)(*piVar4 + 0x10))(), *(int *)(iVar6 + 0x168) == param_1)) {
      piVar7 = (int *)(**(code **)(*piVar4 + 0x10))();
      iVar6 = (**(code **)(*piVar7 + 0x20))();
      if (iVar6 == param_6) {
        iVar8 = iVar5;
        local_c = fVar1;
      }
    }
  }
  if (iVar8 == 0) {
    local_c = 3.4028235e+38;
    for (piVar4 = (int *)0x0; piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
      iVar5 = (**(code **)(*piVar4 + 0x10))();
      iVar6 = FUN_005ad870(iVar5);
      if (((iVar6 != 0) &&
          (fVar1 = param_4 - *(float *)(iVar5 + 0x38), fVar2 = param_3 - *(float *)(iVar5 + 0x34),
          fVar3 = param_2 - *(float *)(iVar5 + 0x30),
          fVar1 = fVar3 * fVar3 + fVar1 * fVar1 + fVar2 * fVar2, fVar1 < local_c)) &&
         (iVar6 = (**(code **)(*piVar4 + 0x10))(), *(int *)(iVar6 + 0x168) == param_1)) {
        piVar7 = (int *)(**(code **)(*piVar4 + 0x10))();
        iVar6 = (**(code **)(*piVar7 + 0x20))();
        if (iVar6 != param_6) {
          iVar8 = iVar5;
          local_c = fVar1;
        }
      }
    }
  }
  return iVar8;
}

