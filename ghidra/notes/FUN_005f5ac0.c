
undefined4 __thiscall FUN_005f5ac0(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  if (param_1 != 0) {
    if (param_1 == *(int *)((int)this + 8)) {
      iVar1 = *(int *)(*(int *)((int)this + 8) + 8);
      *(int *)((int)this + 8) = iVar1;
      if (iVar1 != 0) {
        *(undefined4 *)(iVar1 + 4) = 0;
      }
    }
    else {
      iVar1 = *(int *)(param_1 + 4);
      iVar2 = *(int *)(param_1 + 8);
      if (iVar1 != 0) {
        *(int *)(iVar1 + 8) = iVar2;
      }
      if (iVar2 != 0) {
        *(int *)(iVar2 + 4) = iVar1;
      }
    }
    *(undefined4 *)(param_1 + 4) = 0;
    *(undefined4 *)(param_1 + 8) = 0;
    uVar3 = 1;
    *(int *)((int)this + 4) = *(int *)((int)this + 4) + -1;
  }
  return uVar3;
}

