
uint __thiscall FUN_005f5440(void *this,void *param_1)

{
  uint uVar1;
  
  if (param_1 == (void *)0x0) {
    return 0;
  }
  uVar1 = *(uint *)((int)param_1 + 0x18);
  if (uVar1 == 0) {
    FUN_005f5600(param_1,*(uint *)((int)this + 8));
    uVar1 = *(uint *)((int)this + 8);
    *(uint *)((int)this + 8) = uVar1 + 1;
    FUN_005f4f10(this,(int)param_1);
    return uVar1;
  }
  if (*(uint *)((int)this + 8) <= uVar1) {
    *(uint *)((int)this + 8) = uVar1 + 1;
  }
  FUN_005f4f10(this,(int)param_1);
  return uVar1;
}

