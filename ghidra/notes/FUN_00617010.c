
undefined4 __thiscall FUN_00617010(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = FUN_00617fc0(this);
  if (iVar1 == -1) {
    return 0xffffffff;
  }
  iVar1 = FUN_006171b0((int)this);
  if (iVar1 == -1) {
    return 0xffffffff;
  }
  if (*(int *)((int)this + 8) == 0) {
    *(undefined4 *)((int)this + 0x18) = *(undefined4 *)((int)this + 0x10);
    *(undefined4 *)((int)this + 0x1c) = *(undefined4 *)((int)this + 0x10);
    *(undefined4 *)((int)this + 0x20) = *(undefined4 *)((int)this + 0x14);
  }
  if (param_1 != -1) {
    if ((*(int *)((int)this + 8) == 0) &&
       (*(undefined1 **)((int)this + 0x1c) < *(undefined1 **)((int)this + 0x20))) {
      **(undefined1 **)((int)this + 0x1c) = (char)param_1;
      *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 1;
      return 1;
    }
    iVar1 = FUN_0061b640(*(uint *)((int)this + 0x4c),(char *)&param_1,1);
    if (iVar1 != 1) {
      return 0xffffffff;
    }
  }
  return 1;
}

