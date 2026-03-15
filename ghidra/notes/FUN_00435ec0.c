
undefined4 __thiscall FUN_00435ec0(void *this,void *param_1)

{
  void *pvVar1;
  
  pvVar1 = param_1;
  FUN_005f4db0(param_1,(int)this + 0x1c);
  FUN_005f4db0(pvVar1,(int)this + 0x20);
  FUN_005f4db0(pvVar1,this);
  FUN_005f4db0(pvVar1,(int)this + 4);
  FUN_005f4db0(pvVar1,(int)this + 0x18);
  FUN_005f4db0(pvVar1,(int)this + 8);
  param_1 = (void *)(**(code **)(**(int **)((int)this + 0x10) + 0xc))();
  FUN_005f4db0(pvVar1,&param_1);
  (**(code **)(**(int **)((int)this + 0x10) + 8))(pvVar1);
  (**(code **)(**(int **)((int)this + 0x14) + 0xc))();
  FUN_005f4db0(pvVar1,&stack0x00000000);
  (**(code **)(**(int **)((int)this + 0x14) + 8))(pvVar1);
  FUN_005f5810(&DAT_006b14d8,pvVar1);
  return 1;
}

