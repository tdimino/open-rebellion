
uint __thiscall FUN_00530dd0(void *this,void *param_1)

{
  uint uVar1;
  
  uVar1 = FUN_004f9340(this,param_1);
  if (uVar1 != 0) {
    uVar1 = FUN_005332a0((int)this);
    if (uVar1 != 0) {
      uVar1 = FUN_00533300((int)this);
      if (uVar1 != 0) {
        uVar1 = FUN_0051c950((void *)((int)this + 0x58),param_1);
        if (uVar1 != 0) {
          uVar1 = FUN_0051c950((void *)((int)this + 0x60),param_1);
          if (uVar1 != 0) {
            uVar1 = FUN_0051c950((void *)((int)this + 0x68),param_1);
            if (uVar1 != 0) {
              FUN_005f4d90(param_1,(int)this + 0x70);
              FUN_005f4d90(param_1,(int)this + 0x74);
              FUN_005f4d90(param_1,(int)this + 0x78);
              FUN_005f4d90(param_1,(int)this + 0x7c);
              FUN_005f4d90(param_1,(int)this + 0x80);
              FUN_005f4d90(param_1,(int)this + 0x84);
              if (uVar1 != 0) {
                uVar1 = FUN_00583aa0(*(void **)((int)this + 0x88),param_1);
                if ((uVar1 != 0) &&
                   (uVar1 = *(uint *)(*(int *)((int)this + 0x88) + 0x10), uVar1 != 0)) {
                  uVar1 = FUN_00583aa0(*(void **)((int)this + 0x8c),param_1);
                  if ((uVar1 != 0) &&
                     (uVar1 = *(uint *)(*(int *)((int)this + 0x8c) + 0x10), uVar1 != 0)) {
                    FUN_005f4d90(param_1,(int)this + 0x90);
                    FUN_005f4d90(param_1,(int)this + 0x94);
                    FUN_005f4d90(param_1,(int)this + 0x98);
                    FUN_005f4d90(param_1,(int)this + 0x9c);
                    FUN_005f4d90(param_1,(int)this + 0xa0);
                    FUN_005f4d90(param_1,(int)this + 0xa4);
                    FUN_005f4d90(param_1,(int)this + 0xa8);
                    FUN_005f4d90(param_1,(int)this + 0xac);
                    FUN_005f4d90(param_1,(int)this + 0xb0);
                    FUN_005f4d90(param_1,(int)this + 0xb4);
                    FUN_005f4d90(param_1,(int)this + 0xb8);
                    FUN_005f4d90(param_1,(int)this + 0xbc);
                    FUN_005f4d90(param_1,(int)this + 0xc0);
                    FUN_005f4d90(param_1,(int)this + 0xc4);
                    if (uVar1 != 0) {
                      uVar1 = FUN_004f40b0(*(void **)((int)this + 200),param_1);
                      if ((uVar1 != 0) &&
                         (uVar1 = *(uint *)(*(int *)((int)this + 200) + 0xc), uVar1 != 0)) {
                        uVar1 = FUN_00540310((void *)((int)this + 0xcc),param_1);
                        if (uVar1 != 0) {
                          uVar1 = FUN_00540310((void *)((int)this + 0xd4),param_1);
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
  return uVar1;
}

