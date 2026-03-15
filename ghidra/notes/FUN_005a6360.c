
int * __thiscall FUN_005a6360(void *this,uint param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_005f5500((void *)((int)this + 0x14),param_1);
  if (iVar1 == 0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = (int *)(iVar1 + -0xd4);
  }
  if (piVar2 == (int *)0x0) {
    iVar1 = FUN_00597530();
    if (*(int *)(iVar1 + 0x668) == 0) {
      FUN_005aaf90();
      iVar1 = FUN_005a9330();
      if (iVar1 != 0) {
        return (int *)(iVar1 + 0x1c);
      }
    }
    else {
      FUN_00596f30();
      iVar1 = FUN_00596bd0();
      iVar1 = FUN_005a66a0(iVar1);
      if (iVar1 != 0) {
        return (int *)(iVar1 + 0x1c);
      }
    }
  }
  else {
    iVar1 = (**(code **)(*piVar2 + 0x20))();
    if (iVar1 == 0) {
      return piVar2 + 0x106;
    }
  }
  return (int *)0x0;
}

