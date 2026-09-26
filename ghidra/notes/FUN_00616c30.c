
void * __thiscall FUN_00616c30(void *this,undefined4 param_1)

{
  uint uVar1;
  int iVar2;
  
  iVar2 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
  if (*(int *)(iVar2 + 0x30) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)(iVar2 + 0x34));
  }
  iVar2 = (**(code **)(**(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this) + 0x10))(param_1,1);
  if (iVar2 == -1) {
    iVar2 = *(int *)(*(int *)this + 4);
    uVar1 = *(uint *)(iVar2 + 8 + (int)this);
    if (*(int *)(iVar2 + 0x34 + (int)this) < 0) {
      FUN_00615ee0((LPCRITICAL_SECTION)((int)this + iVar2 + 0x38));
    }
    *(uint *)((int)this + iVar2 + 8) = uVar1 | 2;
    if (*(int *)((int)this + iVar2 + 0x34) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + iVar2 + 0x38));
    }
  }
  iVar2 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
  if (*(int *)(iVar2 + 0x30) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)(iVar2 + 0x34));
  }
  return this;
}

