
void __thiscall FUN_005bd890(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0;
  FUN_005f4d90(param_1,&local_4);
  FUN_005f5600(this,local_4);
  FUN_005f4d90(pvVar1,&local_4);
  *(undefined4 *)((int)this + 0x20) = local_4;
  FUN_005f4d90(pvVar1,&local_4);
  *(undefined4 *)((int)this + 0x28) = local_4;
  FUN_005f4d90(pvVar1,(int)this + 0x2c);
  FUN_005f4d90(pvVar1,(int)this + 0x30);
  FUN_005f4dd0(pvVar1,(int)this + 0x34);
  FUN_005f3580((void *)((int)this + 0x38),pvVar1);
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x44) = param_1;
  FUN_005f4d90(pvVar1,(int)this + 0x48);
  FUN_005f4d90(pvVar1,(int)this + 0x54);
  FUN_005f4d90(pvVar1,(int)this + 0x58);
  FUN_005f4d90(pvVar1,(int)this + 0x4c);
  FUN_005f4d90(pvVar1,(int)this + 0x50);
  FUN_005f4d90(pvVar1,(int)this + 0x5c);
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x60) = param_1;
  return;
}

