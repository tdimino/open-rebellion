
void * __thiscall FUN_00616b70(void *this,int *param_1)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = FUN_00617a90(this,0);
  if (iVar1 != 0) {
    uVar2 = FUN_00618440(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
    while (uVar2 != 0xffffffff) {
      if ((undefined1 *)param_1[7] < (undefined1 *)param_1[8]) {
        *(undefined1 *)param_1[7] = (char)uVar2;
        uVar2 = uVar2 & 0xff;
        param_1[7] = param_1[7] + 1;
      }
      else {
        uVar2 = (**(code **)(*param_1 + 0x1c))(uVar2);
      }
      if (uVar2 == 0xffffffff) {
        *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
             *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
      }
      uVar2 = FUN_00618440(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
    }
    iVar1 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
    if (*(int *)(iVar1 + 0x30) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
    }
    if (*(int *)((int)this + *(int *)(*(int *)this + 4) + 0x34) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*(int *)this + 4) + 0x38));
    }
  }
  return this;
}

