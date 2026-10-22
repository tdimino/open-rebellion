
undefined4 __thiscall FUN_005f5e40(void *this,int param_1)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  if (param_1 != 0) {
    if (*(int *)((int)this + 8) != 0) {
      *(int *)(*(int *)((int)this + 8) + 8) = param_1;
      *(undefined4 *)(param_1 + 4) = *(undefined4 *)((int)this + 8);
      *(int *)((int)this + 8) = param_1;
      *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + 1;
      return 1;
    }
    *(int *)((int)this + 4) = param_1;
    *(int *)((int)this + 8) = param_1;
    *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + 1;
    uVar1 = 1;
  }
  return uVar1;
}

