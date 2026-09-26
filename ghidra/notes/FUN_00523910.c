
int __thiscall FUN_00523910(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  
  pvVar1 = param_1;
  iVar2 = FUN_004f9450(this,param_1);
  if (iVar2 != 0) {
    FUN_005f4db0(pvVar1,(int)this + 0x58);
    FUN_005f4db0(pvVar1,(int)this + 0x5c);
    param_1 = *(void **)((int)this + 0x60);
    FUN_005f4db0(pvVar1,&param_1);
    param_1 = *(void **)((int)this + 100);
    FUN_005f4db0(pvVar1,&param_1);
    param_1 = *(void **)((int)this + 0x68);
    FUN_005f4db0(pvVar1,&param_1);
    if (iVar2 != 0) {
      iVar3 = FUN_004ecea0((void *)((int)this + 0x6c),pvVar1);
      iVar2 = 0;
      if (iVar3 != 0) {
        iVar3 = FUN_004ecea0((void *)((int)this + 0x70),pvVar1);
        iVar2 = 0;
        if (iVar3 != 0) {
          iVar3 = FUN_004ecea0((void *)((int)this + 0x74),pvVar1);
          iVar2 = 0;
          if (iVar3 != 0) {
            iVar3 = FUN_004ecea0((void *)((int)this + 0x78),pvVar1);
            iVar2 = 0;
            if (iVar3 != 0) {
              iVar3 = FUN_004ecea0((void *)((int)this + 0x7c),pvVar1);
              iVar2 = 0;
              if (iVar3 != 0) {
                iVar2 = FUN_004ecea0((void *)((int)this + 0x80),pvVar1);
                if (iVar2 != 0) {
                  (**(code **)(*(int *)((int)this + 0x84) + 0x18))(pvVar1);
                  if (iVar2 != 0) {
                    (**(code **)(*(int *)((int)this + 0x8c) + 0x18))(pvVar1);
                    if (iVar2 != 0) {
                      (**(code **)(*(int *)((int)this + 0x94) + 0x18))(pvVar1);
                      if (iVar2 != 0) {
                        (**(code **)(*(int *)((int)this + 0x9c) + 0x18))(pvVar1);
                        if (iVar2 != 0) {
                          FUN_005f4db0(pvVar1,(int)this + 0xa4);
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  return iVar2;
}

