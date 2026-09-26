
uint __thiscall FUN_0051d2c0(void *this,int *param_1)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  undefined3 extraout_var;
  
  piVar1 = param_1;
  FUN_005f4d70(param_1);
  FUN_00568a80(piVar1);
  FUN_005f4db0(piVar1,(int)this + 4);
  FUN_005f4db0(piVar1,(int)this + 8);
  FUN_005f4db0(piVar1,(int)this + 0xc);
  FUN_005f4db0(piVar1,(int)this + 0x10);
  FUN_005f4db0(piVar1,(int)this + 0x14);
  FUN_005f4db0(piVar1,(int)this + 0x18);
  FUN_005f4db0(piVar1,(int)this + 0x1c);
  FUN_005f4db0(piVar1,(int)this + 0x20);
  FUN_005f4db0(piVar1,(int)this + 0x24);
  FUN_005f4db0(piVar1,(int)this + 0x28);
  FUN_005f4db0(piVar1,(int)this + 0x2c);
  FUN_005f4db0(piVar1,(int)this + 0x30);
  FUN_005f4db0(piVar1,(int)this + 0x34);
  FUN_005f4db0(piVar1,(int)this + 0x38);
  FUN_005f4db0(piVar1,(int)this + 0x3c);
  FUN_005f4db0(piVar1,(int)this + 0x40);
  FUN_005f4db0(piVar1,(int)this + 0x44);
  FUN_005f4db0(piVar1,(int)this + 0x48);
  FUN_005f4db0(piVar1,(int)this + 0x4c);
  iVar3 = FUN_00539970((void *)((int)this + 0x50),piVar1);
  uVar4 = 0;
  if (iVar3 != 0) {
    iVar3 = FUN_00539970((void *)((int)this + 0x6c),piVar1);
    uVar4 = 0;
    if (iVar3 != 0) {
      uVar4 = FUN_00539970((void *)((int)this + 0x88),piVar1);
      if (uVar4 != 0) {
        FUN_005f4d70(piVar1);
        if (uVar4 != 0) {
          uVar4 = (uint)(*(void **)((int)this + 0xa4) != (void *)0x0);
          if (uVar4 != 0) {
            FUN_00536f70(*(void **)((int)this + 0xa4),piVar1);
            FUN_005f4d70(piVar1);
            if (uVar4 != 0) {
              uVar4 = 0;
              if (*(int **)((int)this + 0xa8) != (int *)0x0) {
                uVar4 = (**(code **)(**(int **)((int)this + 0xa8) + 0x10))(piVar1);
                if (uVar4 != 0) {
                  FUN_005f4d70(piVar1);
                  if (uVar4 != 0) {
                    uVar4 = 0;
                    if (*(int **)((int)this + 0xac) != (int *)0x0) {
                      uVar4 = (**(code **)(**(int **)((int)this + 0xac) + 0x10))(piVar1);
                      if (uVar4 != 0) {
                        FUN_005f4d70(piVar1);
                        if (uVar4 != 0) {
                          uVar4 = 0;
                          if (*(int **)((int)this + 0xb0) != (int *)0x0) {
                            uVar4 = (**(code **)(**(int **)((int)this + 0xb0) + 0x10))(piVar1);
                            if (uVar4 != 0) {
                              FUN_005f4d70(piVar1);
                              if (uVar4 != 0) {
                                uVar4 = 0;
                                if (*(void **)((int)this + 0xb4) != (void *)0x0) {
                                  uVar4 = FUN_00568980(*(void **)((int)this + 0xb4),piVar1);
                                  if (uVar4 != 0) {
                                    FUN_005f4d70(piVar1);
                                    if (uVar4 != 0) {
                                      uVar4 = 0;
                                      if (*(void **)((int)this + 0xb8) != (void *)0x0) {
                                        uVar4 = FUN_00568980(*(void **)((int)this + 0xb8),piVar1);
                                        if (uVar4 != 0) {
                                          FUN_005f4d70(piVar1);
                                          if (uVar4 != 0) {
                                            uVar4 = 0;
                                            if (*(int **)((int)this + 0xbc) != (int *)0x0) {
                                              uVar4 = (**(code **)(**(int **)((int)this + 0xbc) +
                                                                  0x14))(piVar1);
                                              if (uVar4 != 0) {
                                                FUN_005f4d70(piVar1);
                                                if (uVar4 != 0) {
                                                  uVar4 = 0;
                                                  if (*(int **)((int)this + 0xc0) != (int *)0x0) {
                                                    bVar2 = FUN_00513df0(piVar1,*(int **)((int)this
                                                                                         + 0xc0));
                                                    uVar4 = CONCAT31(extraout_var,bVar2);
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
  }
  param_1 = (int *)0x1;
  if (uVar4 != 0) {
    FUN_005f4db0(piVar1,&param_1);
    if (uVar4 != 0) {
      uVar4 = FUN_005685a0(piVar1);
    }
  }
  return uVar4;
}

