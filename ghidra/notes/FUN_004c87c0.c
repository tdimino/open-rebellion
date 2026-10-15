
undefined4 __thiscall FUN_004c87c0(void *this,undefined4 *param_1)

{
  int *piVar1;
  undefined4 uVar2;
  
  piVar1 = (int *)FUN_005f5500((void *)(*(int *)((int)this + 0x2c) + 0xec),
                               *(uint *)((int)this + 0x48));
  if (piVar1 != (int *)0x0) {
    uVar2 = (**(code **)(*piVar1 + 0x14))(param_1);
    return uVar2;
  }
  *param_1 = 1;
  return 0;
}

