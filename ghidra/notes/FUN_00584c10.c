
undefined4 __thiscall FUN_00584c10(void *this,void *param_1)

{
  void *pvVar1;
  void *local_4;
  
  pvVar1 = param_1;
  local_4 = this;
  FUN_005f5610(this,param_1);
  FUN_005f4d90(pvVar1,(int)this + 0x20);
  FUN_005f4d90(pvVar1,&param_1);
  FUN_005f4d90(pvVar1,&local_4);
  FUN_005f4d90(pvVar1,(int)this + 0x2c);
  FUN_005f4dd0(pvVar1,(int)this + 0x30);
  FUN_005f4dd0(pvVar1,(int)this + 0x32);
  *(void **)((int)this + 0x24) = param_1;
  *(void **)((int)this + 0x28) = local_4;
  return 1;
}

