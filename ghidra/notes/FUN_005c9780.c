
void __thiscall FUN_005c9780(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0;
  FUN_005f4d90(param_1,&local_4);
  FUN_005f5600(this,local_4);
  FUN_005c5de0((void *)((int)this + 0x1c),pvVar1);
  FUN_005c5de0((void *)((int)this + 0x44),pvVar1);
  FUN_005f4d90(pvVar1,(int)this + 0x6c);
  FUN_005f4d90(pvVar1,(int)this + 0x70);
  if (*(undefined4 **)((int)this + 0x40) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)((int)this + 0x40))(1);
  }
  *(undefined4 *)((int)this + 0x40) = 0;
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x38) = param_1;
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x3c) = param_1;
  FUN_005c9c50(this,(int)param_1);
  if (*(int **)((int)this + 0x40) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x40) + 0x18))(pvVar1,this);
  }
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x50) = param_1;
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x58) = param_1;
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x5c) = param_1;
  FUN_005cb1a0(this,*(int *)((int)this + 0x58));
  if (*(int *)((int)this + 0x54) != 0) {
    (**(code **)(*(int *)(*(int *)((int)this + 0x54) + 0xc) + 0x10))(pvVar1);
  }
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 100) = param_1;
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x68) = param_1;
  (**(code **)(*(int *)this + 0x48))(*(undefined4 *)((int)this + 100));
  if (*(int **)((int)this + 0x60) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x60) + 0x20))(pvVar1);
    *(void **)(*(int *)((int)this + 0x60) + 8) = this;
  }
  FUN_005c5b70((void *)((int)this + 0x74),pvVar1);
  FUN_005c5b70((void *)((int)this + 0x80),pvVar1);
  FUN_005d1ed0((void *)((int)this + 0x8c),pvVar1);
  FUN_005d1ed0((void *)((int)this + 0x98),pvVar1);
  FUN_005f4d90(pvVar1,(int)this + 0xb8);
  FUN_005f4d90(pvVar1,(int)this + 0xb0);
  FUN_005f4d90(pvVar1,(int)this + 0xb4);
  return;
}

