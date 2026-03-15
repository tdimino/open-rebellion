
undefined4 __thiscall FUN_005686f0(void *this,void *param_1)

{
  void *local_4;
  
  local_4 = this;
  FUN_005687a0((int)this);
  FUN_005f4ce0(param_1,(int *)((int)this + 4));
  FUN_005f4ce0(param_1,(int *)((int)this + 8));
  FUN_005f4ce0(param_1,(int)this + 0xc);
  local_4 = (void *)0x0;
  FUN_005f4ce0(param_1,&local_4);
  if (local_4 != (void *)0x0) {
    FUN_005687d0((int)this);
    if (*(int *)((int)this + 0x10) != 0) {
      FUN_006173b0(param_1,*(int *)((int)this + 0x10),
                   *(int *)((int)this + 8) * *(int *)((int)this + 4));
    }
  }
  return 1;
}

