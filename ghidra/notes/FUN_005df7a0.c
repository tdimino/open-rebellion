
void __thiscall FUN_005df7a0(void *this,undefined4 param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  FUN_005ef320((int)this);
  *(undefined4 *)((int)this + 0xc0) = param_1;
  (**(code **)(**(int **)((int)this + 0x9c) + 0xc))();
  (**(code **)(**(int **)((int)this + 0xa0) + 0xc))();
  (**(code **)(**(int **)((int)this + 0xa4) + 0x10))();
  (**(code **)(**(int **)((int)this + 0xa8) + 0x10))();
  iVar1 = (**(code **)(**(int **)((int)this + 0xc0) + 4))();
  if (iVar1 == 1) {
    if (*(int *)((int)this + 0xc0) != 0) {
      uVar2 = FUN_00596af0(*(int *)((int)this + 0xc0) + -0xd4);
      *(undefined4 *)((int)this + 0xc4) = uVar2;
      FUN_005df560((int)this);
      return;
    }
    uVar2 = FUN_00596af0(0);
    *(undefined4 *)((int)this + 0xc4) = uVar2;
    FUN_005df560((int)this);
    return;
  }
  iVar1 = (**(code **)(**(int **)((int)this + 0xc0) + 4))();
  if (iVar1 != 4) goto LAB_005df88e;
  (**(code **)(**(int **)((int)this + 0xa4) + 0xc))();
  iVar1 = FUN_00401060();
  if (iVar1 == 1) {
    iVar1 = FUN_00596bb0();
    if (iVar1 == 0) goto LAB_005df86e;
    (**(code **)(**(int **)((int)this + 0xa8) + 0xc))();
  }
  else {
LAB_005df86e:
    (**(code **)(**(int **)((int)this + 0xa8) + 0x10))();
  }
  uVar2 = FUN_00596b20(*(int *)((int)this + 0xc0));
  *(undefined4 *)((int)this + 0xc4) = uVar2;
LAB_005df88e:
  FUN_005df560((int)this);
  return;
}

