
int __thiscall FUN_005104e0(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int iVar3;
  undefined4 local_4;
  
  bVar1 = FUN_0053a010((int)this);
  local_4 = CONCAT31(extraout_var,bVar1);
  if (local_4 != 0) {
    iVar2 = FUN_00539fd0(this,0);
    iVar3 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar3 != 0) {
      iVar2 = FUN_0053a610(0x10000,*(uint *)(iVar2 + 0x88),(uint *)((int)this + 0x88));
      if (iVar2 != 0) {
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
                    /* WARNING: Load size is inaccurate */
            local_4 = (**(code **)(*this + 0x2d4))(*(uint *)((int)this + 0x88) >> 0x10 & 1,param_1);
          }
        }
      }
    }
  }
  return local_4;
}

