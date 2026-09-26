
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005a8bc0(void *this,int param_1)

{
  int *piVar1;
  int iVar2;
  
  if (param_1 == 0) {
    piVar1 = (int *)(**(code **)(*(int *)(*(int *)((int)this + _DAT_00000168 * 4 + 8) + 0x58) + 0x10
                                ))();
    if (piVar1 == (int *)0x0) {
      return;
    }
  }
  else {
    piVar1 = (int *)FUN_005ca4b0((void *)(*(int *)((int)this + *(int *)(param_1 + 0x168) * 4 + 8) +
                                         0x58),*(uint *)(param_1 + 0xec));
    if (piVar1 == (int *)0x0) {
      return;
    }
    iVar2 = (**(code **)(*piVar1 + 0xc))();
    if (iVar2 == 0) {
      piVar1 = (int *)(**(code **)(*(int *)(*(int *)((int)this + *(int *)(param_1 + 0x168) * 4 + 8)
                                           + 0x58) + 0x14))();
      (**(code **)(*piVar1 + 0x10))();
      return;
    }
    piVar1 = (int *)(**(code **)(*piVar1 + 0xc))();
  }
  (**(code **)(*piVar1 + 0x10))();
  return;
}

