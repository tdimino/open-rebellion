
void __thiscall FUN_00488ca0(void *this,int *param_1)

{
  int iVar1;
  
  if (param_1 != (int *)0x0) {
    iVar1 = (**(code **)(*param_1 + 0x10))();
    if (iVar1 == 1) {
      FUN_005f54d0((void *)((int)this + 0x1c),param_1[6]);
      (**(code **)*param_1)(1);
    }
    else {
      if (iVar1 == 2) {
        FUN_005f54d0((void *)((int)this + 0x1c),param_1[6]);
        FUN_005f5440((void *)((int)this + 0x34),param_1);
        return;
      }
      if (iVar1 == 6) {
        FUN_005f54d0((void *)((int)this + 0x28),param_1[6]);
        *(uint *)((int)this + 8) = *(uint *)((int)this + 8) & 0xdfffffff;
        (**(code **)*param_1)(1);
        return;
      }
    }
  }
  return;
}

