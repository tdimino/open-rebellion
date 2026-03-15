
void __thiscall FUN_005a5580(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 uVar2;
  uint uVar3;
  
  pvVar1 = param_1;
  FUN_0059e640(this,param_1);
  FUN_005f4d90(pvVar1,(int)this + 0x54);
  FUN_005f4d90(pvVar1,(uint *)((int)this + 0x58));
  param_1 = (void *)0x0;
  FUN_005f4d90(pvVar1,&param_1);
  FUN_005f4d90(pvVar1,&param_1);
  uVar3 = *(uint *)((int)this + 0x58);
  *(void **)((int)this + 100) = param_1;
  pvVar1 = (void *)FUN_00596bd0();
  uVar2 = FUN_005a6470(pvVar1,uVar3);
  *(undefined4 *)((int)this + 0x5c) = uVar2;
  return;
}

