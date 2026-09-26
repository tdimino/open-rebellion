
undefined4 __thiscall FUN_0053bcc0(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  
  pvVar1 = param_1;
  FUN_00585280(this,param_1);
  FUN_005f4db0(param_1,(int)this + 0x60);
  FUN_005f4db0(param_1,(int)this + 100);
  FUN_005f4db0(param_1,(int)this + 0x68);
  FUN_005f4db0(param_1,(int)this + 0x6c);
  FUN_005f4db0(param_1,(int)this + 0x70);
  iVar3 = (int)this + 0x74;
  param_1 = (void *)0x4;
  do {
    iVar2 = 3;
    do {
      FUN_005f4db0(pvVar1,iVar3);
      iVar3 = iVar3 + 4;
      iVar2 = iVar2 + -1;
    } while (iVar2 != 0);
    param_1 = (void *)((int)param_1 + -1);
  } while (param_1 != (void *)0x0);
  iVar3 = (int)this + 0xa4;
  iVar2 = 3;
  do {
    FUN_005f4db0(pvVar1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  iVar3 = (int)this + 0xb0;
  iVar2 = 3;
  do {
    FUN_005f4db0(pvVar1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  FUN_005f4db0(pvVar1,(int)this + 0xbc);
  return 1;
}

