
undefined4 __thiscall FUN_005afc80(void *this,int param_1)

{
  int iVar1;
  
  if (param_1 != 0) {
    iVar1 = FUN_005ca4b0((void *)((int)this + 100),*(uint *)(param_1 + 0xec));
    if ((iVar1 != 0) && (*(int *)(param_1 + 0x170) == 2)) {
      return 1;
    }
  }
  return 0;
}

