
undefined4 __thiscall FUN_0060b5d0(void *this,int param_1,int param_2)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  
  uVar2 = 1;
  if (param_1 < param_2) {
    iVar1 = *(int *)((int)this + 0xdc);
    iVar3 = param_2 - param_1;
    if (iVar3 < iVar1) {
      if (param_1 < 0) {
        param_1 = 0;
        param_2 = iVar3;
      }
      if (iVar1 < param_2) {
        *(int *)((int)this + 0xe4) = iVar1;
        *(int *)((int)this + 0xe0) = param_1 + (iVar1 - param_2);
        goto LAB_0060b628;
      }
    }
    else {
      param_1 = 0;
      param_2 = iVar1;
    }
    *(int *)((int)this + 0xe0) = param_1;
    *(int *)((int)this + 0xe4) = param_2;
  }
  else {
    uVar2 = 0;
  }
LAB_0060b628:
  if ((*(int *)((int)this + 0xe0) == 0) &&
     (*(int *)((int)this + 0xe4) == *(int *)((int)this + 0xdc))) {
    (**(code **)(*(int *)this + 8))();
    return uVar2;
  }
  (**(code **)(*(int *)this + 4))(5);
  return uVar2;
}

