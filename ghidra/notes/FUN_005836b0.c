
int __thiscall FUN_005836b0(void *this,void *param_1)

{
  int iVar1;
  
  iVar1 = FUN_00553f60(this,param_1);
  if (iVar1 != 0) {
    FUN_005f4d90(param_1,(int)this + 0x18);
    FUN_005f4d90(param_1,(int)this + 0x1c);
    if (iVar1 != 0) {
      iVar1 = FUN_00540310((void *)((int)this + 0x20),param_1);
      if (iVar1 != 0) {
        iVar1 = FUN_00540310((void *)((int)this + 0x28),param_1);
        if (iVar1 != 0) {
          iVar1 = (**(code **)(*(int *)((int)this + 0x30) + 4))(param_1);
        }
      }
    }
  }
  return iVar1;
}

