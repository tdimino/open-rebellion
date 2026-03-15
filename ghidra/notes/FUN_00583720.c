
int __thiscall FUN_00583720(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  
  pvVar1 = param_1;
  iVar2 = FUN_00553fa0(this,param_1);
  if (iVar2 != 0) {
    FUN_005f4db0(pvVar1,(int)this + 0x18);
    param_1 = *(void **)((int)this + 0x1c);
    FUN_005f4db0(pvVar1,&param_1);
    if (iVar2 != 0) {
      iVar2 = FUN_00540320((void *)((int)this + 0x20),pvVar1);
      if (iVar2 != 0) {
        iVar2 = FUN_00540320((void *)((int)this + 0x28),pvVar1);
        if (iVar2 != 0) {
          iVar2 = (**(code **)(*(int *)((int)this + 0x30) + 8))(pvVar1);
        }
      }
    }
  }
  return iVar2;
}

