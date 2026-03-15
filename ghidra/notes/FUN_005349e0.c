
void __thiscall FUN_005349e0(void *this,undefined4 param_1,int param_2,undefined4 param_3)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  int local_4;
  
  uVar1 = *(uint *)((int)this + 0x24);
                    /* WARNING: Load size is inaccurate */
  piVar3 = &local_4;
  local_4 = 0;
  iVar2 = (**(code **)(*this + 0x1f0))();
  iVar2 = FUN_0055bfa0(uVar1 >> 6 & 3,iVar2,param_2,piVar3);
  if (iVar2 != 0) {
    if (local_4 != 0) {
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x208))(param_3);
      return;
    }
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 0x20c))(param_1,param_3);
  }
  return;
}

