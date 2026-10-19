
int * __thiscall FUN_004cf3b0(void *this,undefined4 *param_1)

{
  void *this_00;
  int *piVar1;
  
  piVar1 = (int *)0x0;
  *param_1 = 1;
  this_00 = (void *)FUN_005f5500((void *)(*(int *)((int)this + 0x2c) + 0x11c),
                                 *(uint *)((int)this + 0x40));
  if (this_00 != (void *)0x0) {
    piVar1 = FUN_00474390(this_00,param_1);
  }
  return piVar1;
}

