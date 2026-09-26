
undefined4 __thiscall FUN_00617a90(void *this,int param_1)

{
  uint uVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  
  if (*(int *)((int)this + *(int *)(*(int *)this + 4) + 0x34) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)this + *(int *)(*(int *)this + 4) + 0x38));
  }
  if (param_1 != 0) {
    *(undefined4 *)((int)this + 8) = 0;
  }
  iVar4 = *(int *)(*(int *)this + 4);
  uVar1 = *(uint *)((int)this + iVar4 + 8);
  if (uVar1 != 0) {
    *(uint *)((int)this + iVar4 + 8) = uVar1 | 2;
    goto LAB_00617b60;
  }
  piVar2 = *(int **)((int)this + iVar4 + 0x20);
  if (piVar2 != (int *)0x0) {
    if (param_1 != 0) {
      iVar4 = *(int *)((int)this + iVar4 + 4);
      uVar1 = *(uint *)(iVar4 + 0x2c);
      uVar3 = *(uint *)(iVar4 + 0x28);
      if (uVar3 < uVar1) {
        iVar4 = uVar1 - uVar3;
      }
      else {
        iVar4 = 0;
      }
      if (param_1 <= iVar4) goto LAB_00617afc;
    }
    FUN_00616140(piVar2);
  }
LAB_00617afc:
  iVar4 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
  if (*(int *)(iVar4 + 0x30) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)(iVar4 + 0x34));
  }
  if ((param_1 == 0) && ((*(byte *)(*(int *)(*(int *)this + 4) + 0x24 + (int)this) & 1) != 0)) {
    FUN_00617cb0(this);
    uVar1 = *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this);
    if (uVar1 != 0) {
      *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) = uVar1 | 2;
      iVar4 = *(int *)(*(int *)(*(int *)this + 4) + 4 + (int)this);
      if (*(int *)(iVar4 + 0x30) < 0) {
        FUN_00615ef0((LPCRITICAL_SECTION)(iVar4 + 0x34));
      }
LAB_00617b60:
      if (*(int *)((int)this + *(int *)(*(int *)this + 4) + 0x34) < 0) {
        FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*(int *)this + 4) + 0x38));
      }
      return 0;
    }
  }
  return 1;
}

