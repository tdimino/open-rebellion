
void __thiscall FUN_00519060(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  
  iVar2 = FUN_004f9650(this,param_1);
  if ((iVar2 != 0) && ((*(uint *)((int)this + 0x24) & 0x30) == 0)) {
    bVar1 = FUN_005197f0((int)this);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      iVar2 = (**(code **)(*(int *)((int)this + 0x58) + 0xc))(param_1);
      if (iVar2 != 0) {
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        FUN_005f60f0(param_1);
        iVar2 = FUN_005589b0(param_1);
        if (iVar2 != 0) {
          FUN_004fd5a0(param_1);
        }
      }
    }
  }
  return;
}

