
void __thiscall FUN_005aae40(void *this,int *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  void *pvVar4;
  uint uVar5;
  
  FUN_005f54a0((void *)((int)this + 0xa44),param_1[6]);
  FUN_005f54a0((void *)(*(int *)((int)this + param_1[0x14] * 4 + 8) + 0x70),param_1[6]);
  iVar1 = (**(code **)(*param_1 + 4))();
  if (iVar1 == 3) {
    piVar2 = (int *)(**(code **)(param_1[7] + 0x10))();
    while (piVar2 != (int *)0x0) {
      piVar3 = (int *)(**(code **)(*piVar2 + 8))();
      iVar1 = (**(code **)(*piVar2 + 0x10))();
      (**(code **)(*param_1 + 0x30))(*(undefined4 *)(iVar1 + 0xec));
      piVar2 = piVar3;
    }
    if ((param_1[0xd] == 0) && (param_1[0x14] == *(int *)((int)this + 0x9ec))) {
      iVar1 = param_1[6];
      pvVar4 = (void *)FUN_0059ed70();
      FUN_005a0410(pvVar4,iVar1);
    }
  }
  else {
    piVar2 = (int *)(**(code **)(param_1[7] + 0x10))();
    while (piVar2 != (int *)0x0) {
      piVar3 = (int *)(**(code **)(*piVar2 + 8))();
      iVar1 = (**(code **)(*piVar2 + 0x10))();
      (**(code **)(*param_1 + 0x30))(*(undefined4 *)(iVar1 + 0xec));
      piVar2 = piVar3;
    }
    if ((param_1[0xd] == 0) && (param_1[0x14] == *(int *)((int)this + 0x9ec))) {
      uVar5 = param_1[6];
      pvVar4 = (void *)FUN_0059ed70();
      FUN_005a0320(pvVar4,uVar5);
    }
  }
  uVar5 = param_1[6];
  pvVar4 = (void *)FUN_00596bd0();
  iVar1 = FUN_005a64b0(pvVar4,uVar5);
  if ((iVar1 != 0) && (param_1 != (int *)0x0)) {
    (**(code **)*param_1)(1);
  }
  return;
}

