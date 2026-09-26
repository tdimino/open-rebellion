
int __thiscall FUN_004f9450(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  
  pvVar1 = param_1;
  iVar2 = FUN_0053a130(this,param_1);
  if (iVar2 != 0) {
    param_1 = (void *)(uint)(*(int *)((int)this + 0x34) != 0);
    FUN_005f4db0(pvVar1,&param_1);
    if (param_1 != (void *)0x0) {
      FUN_005f3590(*(void **)((int)this + 0x34),pvVar1);
    }
    FUN_005f4db0(pvVar1,(int)this + 0x50);
    FUN_004ecea0((void *)((int)this + 0x38),pvVar1);
    FUN_004ecea0((void *)((int)this + 0x3c),pvVar1);
    FUN_005f4db0(pvVar1,(int)this + 0x40);
    FUN_005f4db0(pvVar1,(int)this + 0x44);
    FUN_005f4db0(pvVar1,(int)this + 0x48);
    FUN_005f4db0(pvVar1,(int)this + 0x4c);
    if ((iVar2 != 0) && ((*(uint *)((int)this + 0x24) & 0x30) == 0)) {
      iVar2 = (**(code **)(**(int **)((int)this + 0x54) + 8))(pvVar1);
    }
  }
  return iVar2;
}

