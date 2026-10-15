
undefined4 __thiscall FUN_0048bbf0(void *this,int param_1)

{
  int *piVar1;
  undefined4 uVar2;
  
  uVar2 = 1;
  if (param_1 == 0) {
    uVar2 = 0;
  }
  else {
    for (piVar1 = *(int **)(param_1 + 8); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[2]) {
      (**(code **)(*piVar1 + 0x1c))(this);
    }
  }
  if (*(int *)((int)this + 0x40) != 0) {
    *(undefined4 *)((int)this + 0x28) = 0xc;
    *(undefined2 *)((int)this + 0x30) = 0x456;
    if (*(int *)((int)this + 0x20) == 1) {
      *(undefined2 *)((int)this + 0x2e) = 0x3f5;
      return uVar2;
    }
    *(undefined2 *)((int)this + 0x2e) = 0x3f6;
  }
  return uVar2;
}

