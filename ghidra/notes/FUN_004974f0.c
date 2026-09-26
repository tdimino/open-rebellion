
undefined4 __thiscall FUN_004974f0(void *this,int param_1)

{
  undefined4 uVar1;
  int *piVar2;
  
  uVar1 = 1;
  if (param_1 == 0) {
    uVar1 = 0;
  }
  else {
    piVar2 = *(int **)(param_1 + 8);
    if (piVar2 != (int *)0x0) {
      do {
        (**(code **)(*piVar2 + 0x1c))(this);
        piVar2 = (int *)piVar2[2];
      } while (piVar2 != (int *)0x0);
      return 1;
    }
  }
  return uVar1;
}

