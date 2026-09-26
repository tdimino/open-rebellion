
void * __thiscall FUN_00617800(void *this,int param_1,int param_2,int param_3)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = 0;
  iVar1 = FUN_00617a90(this,1);
  if (iVar1 != 0) {
    if (param_2 != 0) {
      if (param_2 != 1) {
        do {
          iVar1 = FUN_006184e0(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
          if (iVar1 == -1) {
            *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
                 *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 1;
            if (uVar2 == 0) {
              *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
                   *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
            }
            break;
          }
          if (iVar1 == param_3) {
            if (*(int *)((int)this + 4) != 0) {
              *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
              FUN_00618490(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
            }
            break;
          }
          if (param_1 != 0) {
            *(char *)(uVar2 + param_1) = (char)iVar1;
          }
          FUN_00618490(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
          uVar2 = uVar2 + 1;
        } while (uVar2 < param_2 - 1U);
      }
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + uVar2;
    }
    iVar1 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
    if (*(int *)(iVar1 + 0x30) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
    }
    if (*(int *)(*(int *)(*(int *)this + 4) + 0x34 + (int)this) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*(int *)this + 4) + 0x38));
    }
  }
  if ((param_1 != 0) && (param_2 != 0)) {
    *(undefined1 *)(uVar2 + param_1) = 0;
  }
  *(undefined4 *)((int)this + 4) = 0;
  return this;
}

