
void __thiscall FUN_005af7a0(void *this,int *param_1)

{
  void *pvVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  uint uVar5;
  
  iVar3 = param_1[0x5c];
  if (iVar3 == 0xb) {
    return;
  }
  if (iVar3 == 2) {
    uVar5 = param_1[0x3b];
    pvVar1 = (void *)((int)this + 100);
  }
  else {
    if (iVar3 != 1) goto LAB_005af7de;
    uVar5 = param_1[0x3b];
    pvVar1 = (void *)((int)this + 0x58);
  }
  FUN_005c60d0(pvVar1,uVar5);
LAB_005af7de:
  param_1[0x5c] = 0xb;
  FUN_005b54d0(param_1,100000.0);
  FUN_005c5f20((void *)((int)this + 0x70),(int)param_1);
  uVar5 = param_1[0x5d];
  pvVar1 = (void *)FUN_00596bd0();
  piVar2 = (int *)FUN_005a6480(pvVar1,uVar5);
  if ((piVar2 != (int *)0x0) && (FUN_005caed0(piVar2,param_1[0x3b]), piVar2[0x1b] == 0)) {
    FUN_005f54a0((void *)(*(int *)((int)this + piVar2[0x14] * 4 + 0x24c) + 0x70),piVar2[6]);
  }
  iVar3 = (**(code **)(*param_1 + 0x20))();
  if (iVar3 != 0) {
    FUN_005c60d0((void *)(*(int *)((int)this + param_1[0x5a] * 4 + 0x24c) + 100),param_1[0x3b]);
    return;
  }
  FUN_005c60d0((void *)(*(int *)((int)this + param_1[0x5a] * 4 + 0x24c) + 0x58),param_1[0x3b]);
  piVar2 = (int *)(**(code **)(*(int *)param_1[0x10d] + 0x10))();
  while (piVar2 != (int *)0x0) {
    piVar4 = (int *)(**(code **)(*piVar2 + 8))();
    piVar2 = (int *)(**(code **)(*piVar2 + 0x10))();
    FUN_005af7a0(this,piVar2);
    piVar2 = piVar4;
  }
  (**(code **)(*(int *)param_1[0x10d] + 4))();
  piVar2 = (int *)(**(code **)(*(int *)param_1[0x10e] + 0x10))();
  while (piVar2 != (int *)0x0) {
    piVar4 = (int *)(**(code **)(*piVar2 + 8))();
    piVar2 = (int *)(**(code **)(*piVar2 + 0x10))();
    FUN_005af7a0(this,piVar2);
    piVar2 = piVar4;
  }
  (**(code **)(*(int *)param_1[0x10e] + 4))();
  return;
}

