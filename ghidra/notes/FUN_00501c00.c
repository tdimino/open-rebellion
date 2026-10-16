
void __thiscall FUN_00501c00(void *this,void *param_1)

{
  int iVar1;
  
  iVar1 = (**(code **)(*(int *)this + 0x274))();
  if (iVar1 == 0) {
    iVar1 = FUN_005006a0((int)this);
    if (iVar1 == 0) {
      FUN_004f7800(this,0,param_1);
      return;
    }
  }
  FUN_004f7800(this,1,param_1);
  return;
}

