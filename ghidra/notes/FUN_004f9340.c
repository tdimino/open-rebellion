
uint __thiscall FUN_004f9340(void *this,void *param_1)

{
  void *pvVar1;
  bool bVar2;
  uint uVar3;
  int iVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  
  pvVar1 = param_1;
  uVar3 = FUN_0053a080(this,param_1);
  if (uVar3 != 0) {
    FUN_005f4d90(pvVar1,&param_1);
    if (param_1 == (void *)0x0) {
      if (*(undefined4 **)((int)this + 0x34) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0x34))(1);
        *(undefined4 *)((int)this + 0x34) = 0;
      }
    }
    else {
      uVar3 = FUN_004fc680((int)this);
      if (uVar3 == 0) {
        return 0;
      }
      FUN_005f3580(*(void **)((int)this + 0x34),pvVar1);
    }
    if (uVar3 != 0) {
      FUN_005f4d90(pvVar1,(int)this + 0x50);
      FUN_004ece90((void *)((int)this + 0x38),pvVar1);
      FUN_004ece90((void *)((int)this + 0x3c),pvVar1);
      FUN_005f4d90(pvVar1,(int)this + 0x40);
      FUN_005f4d90(pvVar1,(int)this + 0x44);
      FUN_005f4d90(pvVar1,(int)this + 0x48);
      FUN_005f4d90(pvVar1,(int)this + 0x4c);
      if (uVar3 != 0) {
        if ((*(uint *)((int)this + 0x24) & 0x30) == 0) {
          iVar4 = FUN_004fc5e0((int)this);
          if (iVar4 == 0) {
            return 0;
          }
          bVar2 = FUN_004fc5c0(this);
          if (CONCAT31(extraout_var,bVar2) == 0) {
            return 0;
          }
          uVar3 = (**(code **)(**(int **)((int)this + 0x54) + 4))(pvVar1);
        }
        bVar2 = uVar3 != 0;
        uVar3 = 0;
        if (bVar2) {
          bVar2 = FUN_0053f370(this);
          uVar3 = CONCAT31(extraout_var_00,bVar2);
        }
      }
    }
  }
  return uVar3;
}

