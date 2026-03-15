
undefined4 FUN_005a8cc0(int param_1)

{
  void *this;
  int iVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  uint uVar5;
  
  if (param_1 != 0) {
    uVar5 = *(uint *)(param_1 + 0x178);
    this = (void *)FUN_00596bd0();
    iVar1 = FUN_005a6460(this,uVar5);
    if (iVar1 != 0) {
      piVar2 = (int *)FUN_005ca4b0((int *)(iVar1 + 0x44),*(uint *)(param_1 + 0xec));
      if (piVar2 != (int *)0x0) {
        iVar3 = (**(code **)(*piVar2 + 0xc))();
        if (iVar3 != 0) {
          piVar2 = (int *)(**(code **)(*piVar2 + 0xc))();
          uVar4 = (**(code **)(*piVar2 + 0x10))();
          return uVar4;
        }
      }
      piVar2 = (int *)(**(code **)(*(int *)(iVar1 + 0x44) + 0x14))();
      uVar4 = (**(code **)(*piVar2 + 0x10))();
      return uVar4;
    }
  }
  return 0;
}

