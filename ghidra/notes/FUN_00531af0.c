
int __thiscall FUN_00531af0(void *this,void *param_1)

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
      iVar3 = *(int *)((int)this + 0x7c);
      if (iVar3 != *(int *)(iVar2 + 0x7c)) {
        *(int *)((int)this + 0x7c) = *(int *)(iVar2 + 0x7c);
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
                    /* WARNING: Load size is inaccurate */
            local_4 = (**(code **)(*this + 0x228))(iVar3,*(undefined4 *)((int)this + 0x7c),param_1);
          }
        }
      }
    }
  }
  return local_4;
}

