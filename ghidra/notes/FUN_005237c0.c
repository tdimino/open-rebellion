
uint __thiscall FUN_005237c0(void *this,void *param_1)

{
  void *pvVar1;
  uint uVar2;
  int iVar3;
  
  pvVar1 = param_1;
  uVar2 = FUN_004f9340(this,param_1);
  if (uVar2 != 0) {
    FUN_005f4d90(pvVar1,(int)this + 0x58);
    FUN_005f4d90(pvVar1,(int)this + 0x5c);
    FUN_005f4d90(pvVar1,&param_1);
    *(void **)((int)this + 0x60) = param_1;
    FUN_005f4d90(pvVar1,&param_1);
    *(void **)((int)this + 100) = param_1;
    FUN_005f4d90(pvVar1,&param_1);
    *(void **)((int)this + 0x68) = param_1;
    if (uVar2 != 0) {
      iVar3 = FUN_004ece90((void *)((int)this + 0x6c),pvVar1);
      uVar2 = 0;
      if (iVar3 != 0) {
        iVar3 = FUN_004ece90((void *)((int)this + 0x70),pvVar1);
        uVar2 = 0;
        if (iVar3 != 0) {
          iVar3 = FUN_004ece90((void *)((int)this + 0x74),pvVar1);
          uVar2 = 0;
          if (iVar3 != 0) {
            iVar3 = FUN_004ece90((void *)((int)this + 0x78),pvVar1);
            uVar2 = 0;
            if (iVar3 != 0) {
              iVar3 = FUN_004ece90((void *)((int)this + 0x7c),pvVar1);
              uVar2 = 0;
              if (iVar3 != 0) {
                uVar2 = FUN_004ece90((void *)((int)this + 0x80),pvVar1);
                if (uVar2 != 0) {
                  (**(code **)(*(int *)((int)this + 0x84) + 0x10))(pvVar1);
                  if (uVar2 != 0) {
                    (**(code **)(*(int *)((int)this + 0x8c) + 0x10))(pvVar1);
                    if (uVar2 != 0) {
                      (**(code **)(*(int *)((int)this + 0x94) + 0x10))(pvVar1);
                      if (uVar2 != 0) {
                        (**(code **)(*(int *)((int)this + 0x9c) + 0x10))(pvVar1);
                        if (uVar2 != 0) {
                          FUN_005f4d90(pvVar1,(int)this + 0xa4);
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
  return uVar2;
}

