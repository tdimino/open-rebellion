
undefined4 __thiscall FUN_005f59f0(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  if (*(int *)((int)this + 8) == 0) {
    *(int *)((int)this + 8) = param_1;
    *(undefined4 *)(param_1 + 4) = 0;
    *(undefined4 *)(*(int *)((int)this + 8) + 8) = 0;
  }
  else {
                    /* WARNING: Load size is inaccurate */
    iVar2 = (**(code **)(*this + 4))(param_1,*(int *)((int)this + 8));
    if (iVar2 != 0) {
      *(int *)(*(int *)((int)this + 8) + 4) = param_1;
      *(undefined4 *)(param_1 + 8) = *(undefined4 *)((int)this + 8);
      *(int *)((int)this + 8) = param_1;
      *(int *)((int)this + 4) = *(int *)((int)this + 4) + 1;
      return 1;
    }
    iVar2 = *(int *)((int)this + 8);
    if (iVar2 == param_1) {
      return 0;
    }
    iVar1 = *(int *)(iVar2 + 8);
                    /* WARNING: Load size is inaccurate */
    while ((iVar1 != 0 && (iVar3 = (**(code **)(*this + 4))(param_1,iVar1), iVar3 == 0))) {
      if (iVar1 == param_1) {
        return 0;
      }
      iVar2 = iVar1;
      iVar1 = *(int *)(iVar1 + 8);
    }
    *(int *)(iVar2 + 8) = param_1;
    *(int *)(param_1 + 4) = iVar2;
    *(int *)(param_1 + 8) = iVar1;
    if (iVar1 != 0) {
      *(int *)(iVar1 + 4) = param_1;
      *(int *)((int)this + 4) = *(int *)((int)this + 4) + 1;
      return 1;
    }
  }
  *(int *)((int)this + 4) = *(int *)((int)this + 4) + 1;
  return 1;
}

