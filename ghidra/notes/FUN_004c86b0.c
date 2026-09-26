
undefined4 __fastcall FUN_004c86b0(int param_1)

{
  void *this;
  uint uVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  this = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),*(uint *)(param_1 + 0x40));
  if (this != (void *)0x0) {
    if (*(uint *)((int)this + 0xe4) != 0) {
      FUN_0042ef90((void *)(*(int *)(param_1 + 0x2c) + 0xec),*(uint *)((int)this + 0xe4));
      *(undefined4 *)((int)this + 0xe4) = 0;
    }
    if (*(uint *)((int)this + 0xe0) != 0) {
      uVar1 = FUN_0042ef50((void *)(*(int *)(param_1 + 0x2c) + 0xec),*(uint *)((int)this + 0xe0));
      *(uint *)(param_1 + 0x48) = uVar1;
      piVar2 = (int *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xec),uVar1);
      if (piVar2 != (int *)0x0) {
        iVar3 = (**(code **)(*piVar2 + 0xc))();
        if (iVar3 == 1) {
          FUN_00476d10(this,piVar2);
        }
        uVar4 = 1;
      }
    }
  }
  return uVar4;
}

