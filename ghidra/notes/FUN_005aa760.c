
undefined4 __thiscall FUN_005aa760(void *this,int *param_1)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  undefined4 *puVar5;
  int *piVar6;
  undefined3 extraout_var;
  int iVar7;
  
  piVar3 = param_1;
  FUN_005c60d0((void *)((int)this + 0x988),(uint)param_1);
  pvVar2 = (void *)FUN_00596bd0();
  piVar3 = (int *)FUN_005a6340(pvVar2,(uint)piVar3);
  FUN_005a93b0(this,piVar3);
  iVar4 = (**(code **)(*piVar3 + 0x20))();
  iVar7 = *(int *)((int)this + piVar3[0x5a] * 4 + 8);
  if (iVar4 == 0) {
    FUN_005c5fb0((void *)(iVar7 + 0x58),(int)piVar3);
    iVar7 = *(int *)((int)this + piVar3[0x5a] * 4 + 8);
    param_1 = piVar3;
    puVar5 = (undefined4 *)FUN_005ae2e0(&param_1);
    if (puVar5 != (undefined4 *)0x0) {
      FUN_005f4fa0((void *)(iVar7 + 0xa0),(int)puVar5);
      (**(code **)*puVar5)(1);
    }
    iVar7 = *(int *)((int)this + piVar3[0x5a] * 4 + 8);
    param_1 = piVar3;
    puVar5 = (undefined4 *)FUN_005ae2e0(&param_1);
    if (puVar5 != (undefined4 *)0x0) {
      FUN_005f4fa0((void *)(iVar7 + 0x98),(int)puVar5);
      (**(code **)*puVar5)(1);
    }
    iVar7 = *(int *)((int)this + piVar3[0x5a] * 4 + 8);
    param_1 = piVar3;
    puVar5 = (undefined4 *)FUN_005ae2e0(&param_1);
    if (puVar5 != (undefined4 *)0x0) {
      FUN_005f4fa0((void *)(iVar7 + 0xa8),(int)puVar5);
      (**(code **)*puVar5)(1);
    }
    for (piVar6 = (int *)(**(code **)(*(int *)piVar3[0x10d] + 0x10))(); piVar6 != (int *)0x0;
        piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
      iVar7 = *(int *)((int)this + piVar3[0x5a] * 4 + 8);
      iVar4 = (**(code **)(*piVar6 + 0x10))();
      FUN_005c5fb0((void *)(iVar7 + 100),iVar4);
      param_1 = (int *)(**(code **)(*piVar6 + 0x10))();
      iVar7 = *(int *)((int)this + piVar3[0x5a] * 4 + 8);
      puVar5 = (undefined4 *)FUN_005ae2e0(&param_1);
      if (puVar5 != (undefined4 *)0x0) {
        FUN_005f4fa0((void *)(iVar7 + 0xa0),(int)puVar5);
        (**(code **)*puVar5)(1);
      }
    }
    (**(code **)(*(int *)piVar3[0x10d] + 4))();
  }
  else {
    FUN_005c5fb0((void *)(iVar7 + 100),(int)piVar3);
    pvVar2 = (void *)(*(int *)((int)this + piVar3[0x5a] * 4 + 8) + 0xa0);
    param_1 = piVar3;
    for (puVar5 = (undefined4 *)FUN_005f5060((int)pvVar2); puVar5 != (undefined4 *)0x0;
        puVar5 = (undefined4 *)puVar5[4]) {
      bVar1 = FUN_005ced00(puVar5 + 7,(int *)&param_1);
      if (CONCAT31(extraout_var,bVar1) != 0) goto LAB_005aa924;
    }
    puVar5 = (undefined4 *)0x0;
LAB_005aa924:
    if (puVar5 != (undefined4 *)0x0) {
      FUN_005f4fa0(pvVar2,(int)puVar5);
      (**(code **)*puVar5)(1);
    }
  }
  piVar3[0x5c] = 0;
  iVar7 = (**(code **)(*piVar3 + 0x20))();
  if (iVar7 == 1) {
    piVar3[0x42] = 1;
    FUN_0059ed70();
    FUN_0059f4d0((int)piVar3);
  }
  return 1;
}

