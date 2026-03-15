
void __thiscall FUN_005ceda0(void *this,void *param_1)

{
  void *pvVar1;
  void *this_00;
  undefined4 uVar2;
  void *pvVar3;
  void *local_4;
  
  pvVar1 = param_1;
  local_4 = this;
  FUN_005f4d90(param_1,(int)this + 4);
  param_1 = (void *)0x0;
  FUN_005f4d90(pvVar1,&param_1);
  pvVar3 = param_1;
  this_00 = (void *)FUN_00596bd0();
  uVar2 = FUN_005a6480(this_00,(uint)pvVar3);
  *(undefined4 *)((int)this + 8) = uVar2;
  FUN_005f4d90(pvVar1,(int)this + 0xc);
  FUN_005f4d90(pvVar1,(int)this + 0x10);
  FUN_005f4d90(pvVar1,&local_4);
  *(void **)((int)this + 0x14) = local_4;
  FUN_005c5b70((void *)((int)this + 0x18),pvVar1);
  return;
}

