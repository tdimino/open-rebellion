
undefined4 __thiscall FUN_00584c90(void *this,void *param_1)

{
  void *pvVar1;
  
  pvVar1 = param_1;
  FUN_005f5630(this,param_1);
  FUN_005f4db0(pvVar1,(int)this + 0x20);
  param_1 = *(void **)((int)this + 0x24);
  FUN_005f4db0(pvVar1,&param_1);
  param_1 = *(void **)((int)this + 0x28);
  FUN_005f4db0(pvVar1,&param_1);
  FUN_005f4db0(pvVar1,(int)this + 0x2c);
  FUN_005f4df0(pvVar1,(int)this + 0x30);
  FUN_005f4df0(pvVar1,(int)this + 0x32);
  return 1;
}

