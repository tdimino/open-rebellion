
int __thiscall FUN_00540870(void *this,void *param_1)

{
  int iVar1;
  
  iVar1 = FUN_00583e40(this,param_1);
  if (iVar1 != 0) {
    iVar1 = FUN_00540310((void *)((int)this + 0x18),param_1);
    if (iVar1 != 0) {
      iVar1 = FUN_00540310((void *)((int)this + 0x20),param_1);
      if (iVar1 != 0) {
        iVar1 = FUN_00540310((void *)((int)this + 0x28),param_1);
        if (iVar1 != 0) {
          iVar1 = FUN_004ece90((void *)((int)this + 0x30),param_1);
          if (iVar1 != 0) {
            FUN_005f4d90(param_1,(int)this + 0x34);
          }
        }
      }
    }
  }
  return iVar1;
}

