
void * __thiscall FUN_005c6090(void *this,int *param_1)

{
  int *piVar1;
  int iVar2;
  
  if (param_1 != this) {
    for (piVar1 = (int *)(**(code **)(*param_1 + 0x10))(); piVar1 != (int *)0x0;
        piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      FUN_005c5f20(this,iVar2);
    }
  }
  return this;
}

