
undefined4 __thiscall FUN_005ae170(void *this,int param_1)

{
  int *piVar1;
  
  piVar1 = (int *)FUN_0059ed70();
  if ((((*piVar1 == 1) && (param_1 != 0)) &&
      (*(int *)(param_1 + 0x50) == *(int *)((int)this + 0x9ec))) &&
     (*(int *)(*(int *)((int)this + *(int *)((int)this + 0x9ec) * 4 + 8) + 0x84) == 0)) {
    return 1;
  }
  return 0;
}

