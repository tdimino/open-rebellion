
void * __thiscall FUN_006173b0(void *this,undefined4 param_1,uint param_2)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = FUN_00617a90(this,1);
  if (iVar1 != 0) {
                    /* WARNING: Load size is inaccurate */
    uVar2 = (**(code **)(**(int **)(*(int *)(*this + 4) + 4 + (int)this) + 0x18))(param_1,param_2);
    *(uint *)((int)this + 8) = uVar2;
    if (uVar2 < param_2) {
                    /* WARNING: Load size is inaccurate */
      *(uint *)(*(int *)(*this + 4) + 8 + (int)this) =
           *(uint *)(*(int *)(*this + 4) + 8 + (int)this) | 3;
    }
                    /* WARNING: Load size is inaccurate */
    iVar1 = *(int *)(*(int *)(*this + 4) + 4 + (int)this);
    if (*(int *)(iVar1 + 0x30) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
    }
                    /* WARNING: Load size is inaccurate */
    if (*(int *)((int)this + *(int *)(*this + 4) + 0x34) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*this + 4) + 0x38));
    }
  }
  return this;
}

