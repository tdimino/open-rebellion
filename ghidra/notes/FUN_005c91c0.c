
undefined4 __thiscall FUN_005c91c0(void *this,undefined4 param_1)

{
  undefined4 uVar1;
  
  if (*(int **)((int)this + 0x2c) != (int *)0x0) {
    uVar1 = (**(code **)(**(int **)((int)this + 0x2c) + 0x10))(param_1);
    return uVar1;
  }
  return 0;
}

