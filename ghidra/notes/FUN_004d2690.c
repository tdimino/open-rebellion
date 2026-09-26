
undefined4 __fastcall FUN_004d2690(int param_1)

{
  void *this;
  uint uVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  this = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),*(uint *)(param_1 + 0x40));
  if (this != (void *)0x0) {
    if (*(uint *)((int)this + 0xc4) != 0) {
      FUN_0042f690((void *)(*(int *)(param_1 + 0x2c) + 0xa8),*(uint *)((int)this + 0xc4));
      *(undefined4 *)((int)this + 0xc4) = 0;
    }
    if (*(int *)((int)this + 0xbc) != 0) {
      uVar1 = FUN_0042f640((void *)(*(int *)(param_1 + 0x2c) + 0xa8),*(int *)((int)this + 0xbc),
                           *(uint *)((int)this + 0xc0));
      piVar2 = (int *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xa8),uVar1);
      *(undefined4 *)((int)this + 0xbc) = 0;
      if (piVar2 != (int *)0x0) {
        iVar3 = (**(code **)(*piVar2 + 0x10))();
        if (iVar3 == 1) {
          FUN_00476ba0(this,piVar2);
          uVar4 = 1;
        }
      }
    }
  }
  return uVar4;
}

