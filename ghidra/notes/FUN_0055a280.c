
int __thiscall FUN_0055a280(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  
  pvVar1 = param_1;
  iVar2 = FUN_00553f60(this,param_1);
  if (iVar2 != 0) {
    iVar2 = FUN_00540310((void *)((int)this + 0x18),param_1);
    if (iVar2 != 0) {
      iVar2 = FUN_00540310((void *)((int)this + 0x20),param_1);
      if (iVar2 != 0) {
        iVar2 = FUN_00540310((void *)((int)this + 0x28),param_1);
        if (iVar2 != 0) {
          iVar2 = FUN_00540310((void *)((int)this + 0x30),param_1);
          if (iVar2 != 0) {
            iVar2 = FUN_00540310((void *)((int)this + 0x38),param_1);
            if (iVar2 != 0) {
              iVar2 = FUN_00540310((void *)((int)this + 0x40),param_1);
              if (iVar2 != 0) {
                iVar3 = (int)this + 0x54;
                param_1 = (void *)0x3;
                do {
                  FUN_005f4d90(pvVar1,iVar3 + -0xc);
                  FUN_005f4d90(pvVar1,iVar3);
                  FUN_005f4d90(pvVar1,iVar3 + 0x14);
                  iVar3 = iVar3 + 4;
                  param_1 = (void *)((int)param_1 + -1);
                } while (param_1 != (void *)0x0);
                FUN_005f4d90(pvVar1,(int)this + 0x60);
                FUN_005f4d90(pvVar1,(int)this + 100);
                FUN_005f4d90(pvVar1,(int)this + 0x74);
                FUN_005f4d90(pvVar1,(int)this + 0x78);
                FUN_005f4d90(pvVar1,(int)this + 0x7c);
                FUN_005f4d90(pvVar1,(int)this + 0x80);
              }
            }
          }
        }
      }
    }
  }
  return iVar2;
}

