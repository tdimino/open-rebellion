
void * __thiscall FUN_00617430(void *this,undefined4 param_1,int param_2)

{
  int iVar1;
  
  iVar1 = FUN_00615fc0(this);
  if (iVar1 != 0) {
    iVar1 = (**(code **)(**(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this) + 0x14))
                      (param_1,param_2);
    if (iVar1 != param_2) {
      *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
           *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
    }
    FUN_00616040(this);
  }
  return this;
}

