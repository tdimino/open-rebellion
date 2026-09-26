
void * __thiscall FUN_00618240(void *this,LPCSTR param_1,uint param_2,uint param_3)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  
  if (*(int *)((int)this + 0x4c) == -1) {
    uVar1 = (-(uint)((param_2 & 0x80) != 0) & 0x4000) + 0x4000;
    if ((param_2 & 0x20) == 0) {
      uVar1 = uVar1 | 0x100;
    }
    if ((param_2 & 0x40) != 0) {
      uVar1 = uVar1 | 0x400;
    }
    if ((param_2 & 8) != 0) {
      param_2 = param_2 | 2;
      uVar1 = uVar1 | 8;
    }
    if ((param_2 & 0x10) != 0) {
      param_2 = param_2 | 2;
      uVar1 = uVar1 | 0x200;
    }
    if ((param_2 & 2) == 0) {
      if ((param_2 & 1) == 0) {
        return (void *)0x0;
      }
    }
    else {
      if ((param_2 & 1) == 0) {
        uVar1 = uVar1 | 1;
      }
      else {
        uVar1 = uVar1 | 2;
      }
      if ((param_2 & 0x4d) == 0) {
        param_2 = param_2 | 0x10;
        uVar1 = uVar1 | 0x200;
      }
    }
    uVar4 = param_3 & (DAT_0066e588 | DAT_0066e584 | DAT_0066e580);
    uVar2 = 0x40;
    if (uVar4 != 0) {
      if ((int)uVar4 < 0xa01) {
        if (uVar4 == 0xa00) {
          uVar2 = 0x20;
        }
        else if (uVar4 == 0x800) {
          uVar2 = 0x10;
        }
      }
      else if (uVar4 == 0xc00) {
        uVar2 = 0x30;
      }
      else if (uVar4 == 0xe00) {
        uVar2 = 0x40;
      }
    }
    uVar1 = FUN_0061c540(param_1,uVar1,uVar2,0x180);
    *(uint *)((int)this + 0x4c) = uVar1;
    if (uVar1 != 0xffffffff) {
      if (*(int *)((int)this + 0x30) < 0) {
        FUN_00615ee0((LPCRITICAL_SECTION)((int)this + 0x34));
      }
      *(undefined4 *)((int)this + 0x50) = 1;
      if ((*(int *)((int)this + 8) == 0) && (*(int *)((int)this + 0x14) == 0)) {
        iVar3 = FUN_00618b70(0x200);
        if (iVar3 == 0) {
          *(undefined4 *)((int)this + 8) = 1;
        }
        else {
          FUN_00618020(this,iVar3,iVar3 + 0x200,1);
        }
      }
      if (((param_2 & 4) != 0) &&
         (iVar3 = (**(code **)(*(int *)this + 0xc))(0,2,param_2), iVar3 == -1)) {
        FUN_00616f90(this);
        if (-1 < *(int *)((int)this + 0x30)) {
          return (void *)0x0;
        }
        FUN_00615ef0((LPCRITICAL_SECTION)((int)this + 0x34));
        return (void *)0x0;
      }
      if (*(int *)((int)this + 0x30) < 0) {
        FUN_00615ef0((LPCRITICAL_SECTION)((int)this + 0x34));
      }
      return this;
    }
  }
  return (void *)0x0;
}

