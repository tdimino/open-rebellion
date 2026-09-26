
void __thiscall FUN_0048a060(void *this,int *param_1)

{
  undefined4 uVar1;
  int iVar2;
  uint uVar3;
  
  if (param_1 != (int *)0x0) {
    uVar1 = (**(code **)(*param_1 + 0x10))();
    switch(uVar1) {
    case 1:
      FUN_005f5440((void *)((int)this + 0x1c),param_1);
      *(uint *)((int)this + 8) = *(uint *)((int)this + 8) | 1;
      return;
    case 2:
      FUN_005f5440((void *)((int)this + 0x1c),param_1);
      *(uint *)((int)this + 8) = *(uint *)((int)this + 8) | 2;
      return;
    case 3:
    case 4:
    case 5:
    case 8:
      iVar2 = FUN_00583c40((int)(param_1 + 0x11));
      if (iVar2 == 0) {
        (**(code **)*param_1)(1);
      }
      else {
        uVar3 = FUN_0048a1c0(this,param_1[0xd]);
        FUN_005f5440((void *)((int)this + 0x10),param_1);
        *(uint *)((int)this + 0x50) = *(uint *)((int)this + 0x50) | param_1[0xd];
        FUN_0041db20(param_1[0xd]);
        if ((param_1[10] != 0) && ((uVar3 & 4) == 0)) {
          (**(code **)(**(int **)((int)this + 4) + 0x14))(param_1[10]);
          FUN_0048a2a0((int)this);
          FUN_0041d870();
          return;
        }
      }
      FUN_0048a2a0((int)this);
      FUN_0041d870();
      return;
    case 6:
      FUN_005f5440((void *)((int)this + 0x28),param_1);
      return;
    case 7:
      FUN_005f5440((void *)((int)this + 0x10),param_1);
      FUN_0041d7d0((LPARAM)param_1);
      return;
    default:
      (**(code **)*param_1)(1);
    }
  }
  return;
}

