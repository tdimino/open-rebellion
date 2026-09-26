
void __thiscall FUN_005c9940(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = *(undefined4 *)((int)this + 0x18);
  FUN_005f4db0(param_1,&local_4);
  FUN_005c5e50((void *)((int)this + 0x1c),pvVar1);
  FUN_005c5e50((void *)((int)this + 0x44),pvVar1);
  FUN_005f4db0(pvVar1,(int)this + 0x6c);
  FUN_005f4db0(pvVar1,(int)this + 0x70);
  param_1 = *(void **)((int)this + 0x38);
  FUN_005f4db0(pvVar1,&param_1);
  param_1 = *(void **)((int)this + 0x3c);
  FUN_005f4db0(pvVar1,&param_1);
  (**(code **)(**(int **)((int)this + 0x40) + 0x14))(pvVar1);
  FUN_005f4db0(pvVar1,&stack0x00000000);
  FUN_005f4db0(pvVar1,&stack0x00000000);
  FUN_005f4db0(pvVar1,&stack0x00000000);
  if (*(int *)((int)this + 0x54) != 0) {
    (**(code **)(*(int *)(*(int *)((int)this + 0x54) + 0xc) + 0x14))(pvVar1);
  }
  FUN_005f4db0(pvVar1,&stack0x00000000);
  FUN_005f4db0(pvVar1,&stack0x00000000);
  if (*(int **)((int)this + 0x60) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x60) + 0x24))(pvVar1);
  }
  FUN_005c5be0((void *)((int)this + 0x74),pvVar1);
  FUN_005c5be0((void *)((int)this + 0x80),pvVar1);
  FUN_005d1f40((void *)((int)this + 0x8c),pvVar1);
  FUN_005d1f40((void *)((int)this + 0x98),pvVar1);
  FUN_005f4db0(pvVar1,(int)this + 0xb8);
  FUN_005f4db0(pvVar1,(int)this + 0xb0);
  FUN_005f4db0(pvVar1,(int)this + 0xb4);
  return;
}

