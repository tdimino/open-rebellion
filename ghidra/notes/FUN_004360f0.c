
void __thiscall FUN_004360f0(void *this,undefined4 *param_1)

{
  int iVar1;
  
  if (param_1 == (undefined4 *)0x0) {
    return;
  }
  if (param_1[8] == 1) {
    iVar1 = (**(code **)(**(int **)((int)this + 0x14) + 0x20))(param_1);
    if (iVar1 != 0) {
      (**(code **)(**(int **)((int)this + 0x10) + 0x1c))(param_1);
      return;
    }
  }
  else {
    if (param_1[8] != 2) {
      (**(code **)*param_1)(1);
      return;
    }
    iVar1 = (**(code **)(**(int **)((int)this + 0x10) + 0x20))(param_1);
    if (iVar1 != 0) {
      (**(code **)(**(int **)((int)this + 0x14) + 0x1c))(param_1);
      return;
    }
  }
  FUN_00536fe0((void *)((int)this + 0x24),param_1);
  return;
}

