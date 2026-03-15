
void __thiscall FUN_005cee20(void *this,void *param_1)

{
  void *pvVar1;
  
  pvVar1 = param_1;
  FUN_005f4db0(param_1,(int)this + 4);
  param_1 = *(void **)(*(int *)((int)this + 8) + 0x18);
  FUN_005f4db0(pvVar1,&param_1);
  FUN_005f4db0(pvVar1,(int)this + 0xc);
  FUN_005f4db0(pvVar1,(int)this + 0x10);
  param_1 = *(void **)((int)this + 0x14);
  FUN_005f4db0(pvVar1,&param_1);
  FUN_005c5be0((void *)((int)this + 0x18),pvVar1);
  return;
}

