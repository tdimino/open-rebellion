
void __thiscall FUN_005bddb0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_8 = 0;
  FUN_005f4d90(param_1,&local_8);
  FUN_005f5600(this,local_8);
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x20) = param_1;
  FUN_005f4d90(pvVar1,&param_1);
  *(void **)((int)this + 0x28) = param_1;
  FUN_005f4d90(pvVar1,(int)this + 0x2c);
  FUN_005f4d90(pvVar1,(int)this + 0x30);
  FUN_005f4dd0(pvVar1,(int)this + 0x34);
  FUN_005f3580((void *)((int)this + 0x38),pvVar1);
  FUN_005f4d90(pvVar1,&local_4);
  *(undefined4 *)((int)this + 0x44) = local_4;
  FUN_005f4d90(pvVar1,(int)this + 0x48);
  return;
}

