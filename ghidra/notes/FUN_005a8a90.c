
undefined4 __thiscall FUN_005a8a90(void *this,int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  if (param_1 < 2) {
    iVar1 = FUN_005ae1d0();
    if (iVar1 != 1) {
      uVar2 = (**(code **)(**(int **)((int)this + param_1 * 4 + 8) + 0xc))();
      return uVar2;
    }
  }
  return 0;
}

