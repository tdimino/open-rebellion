
undefined4 __thiscall FUN_0049b0f0(void *this,int param_1)

{
  int *piVar1;
  
  for (piVar1 = *(int **)(param_1 + 8); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[2]) {
    (**(code **)(*piVar1 + 0x1c))(this);
  }
  if (*(int *)((int)this + 0x50) != 0) {
    if (*(int *)((int)this + 0x50) != *(int *)((int)this + 0x58)) {
      FUN_0049b4d0((int)this);
      return 1;
    }
    FUN_0049b330((int)this);
    return 1;
  }
  FUN_0049b150((int)this);
  return 1;
}

