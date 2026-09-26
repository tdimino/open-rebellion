
void * __thiscall FUN_00617b90(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  
  iVar1 = FUN_00617a90(this,0);
  if (iVar1 != 0) {
    iVar1 = *(int *)(*(int *)(*(int *)this + 4) + 0x30 + (int)this);
    *(undefined4 *)(*(int *)(*(int *)this + 4) + 0x30 + (int)this) = 0;
    if (param_1 == 0) {
      *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
           *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
    }
    else {
      uVar4 = 0;
      if (iVar1 != 1) {
        do {
          iVar2 = FUN_006184e0(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
          if (iVar2 == -1) {
            *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
                 *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 1;
            if (uVar4 != 0) goto LAB_00617c5c;
            *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
                 *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
            break;
          }
          uVar3 = FUN_00619ff0(iVar2);
          if (uVar3 != 0) break;
          *(char *)(uVar4 + param_1) = (char)iVar2;
          FUN_00618490(*(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this));
          uVar4 = uVar4 + 1;
        } while (uVar4 < iVar1 - 1U);
      }
      if (uVar4 == 0) {
        *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
             *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
      }
      else {
LAB_00617c5c:
        *(undefined1 *)(uVar4 + param_1) = 0;
      }
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

