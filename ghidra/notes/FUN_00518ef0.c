
int __thiscall FUN_00518ef0(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int iVar3;
  
  iVar2 = FUN_004f9450(this,param_1);
  if ((iVar2 != 0) && ((*(uint *)((int)this + 0x24) & 0x30) == 0)) {
    bVar1 = FUN_005197f0((int)this);
    iVar2 = 0;
    if (CONCAT31(extraout_var,bVar1) != 0) {
      iVar3 = (**(code **)(*(int *)((int)this + 0x58) + 8))(param_1);
      iVar2 = 0;
      if (iVar3 != 0) {
        FUN_005f4db0(param_1,(int)this + 0x6c);
        FUN_005f4db0(param_1,(int)this + 0x70);
        FUN_005f4db0(param_1,(int)this + 0x74);
        FUN_005f4db0(param_1,(int)this + 0x78);
        FUN_005f4db0(param_1,(int)this + 0x7c);
        FUN_005f4db0(param_1,(int)this + 0x80);
        FUN_005f4db0(param_1,(int)this + 0x84);
        FUN_005f4db0(param_1,(int)this + 0x88);
        iVar3 = FUN_004ecea0((void *)((int)this + 0x8c),param_1);
        iVar2 = 0;
        if (iVar3 != 0) {
          iVar2 = FUN_004fd540((void *)((int)this + 0x90),param_1);
          if (iVar2 != 0) {
            FUN_005f5810(*(void **)((int)this + 0xac),param_1);
          }
        }
      }
    }
  }
  return iVar2;
}

