
void __thiscall FUN_005c32d0(void *this,void *param_1)

{
  FUN_005f4db0(param_1,(int)this + 0x624);
  FUN_005f4db0(param_1,(int)this + 0x648);
  (**(code **)(**(int **)((int)this + 0x63c) + 8))(param_1);
  if (*(int *)((int)this + 0x640) == 0) {
    FUN_005f4db0(param_1,&stack0x00000000);
  }
  else {
    FUN_005f4db0(param_1,&stack0x00000000);
    (**(code **)(**(int **)((int)this + 0x640) + 8))(param_1);
  }
  if (*(int *)((int)this + 0x644) != 0) {
    FUN_005f4db0(param_1,&stack0x00000000);
    (**(code **)(**(int **)((int)this + 0x644) + 8))(param_1);
    return;
  }
  FUN_005f4db0(param_1,&stack0x00000000);
  return;
}

