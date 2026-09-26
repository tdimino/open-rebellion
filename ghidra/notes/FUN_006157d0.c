
undefined4 __thiscall FUN_006157d0(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  
  if (*(uint *)((int)this + 0x20) <= *(uint *)((int)this + 0x1c)) {
    if (*(int *)((int)this + 0x4c) == 0) {
      return 0xffffffff;
    }
    iVar2 = FUN_00615690(this);
    if (iVar2 == -1) {
      return 0xffffffff;
    }
    if (*(int *)((int)this + 0x20) == 0) {
      iVar2 = (*(int *)((int)this + 0x2c) - *(int *)((int)this + 0x24)) + *(int *)((int)this + 0x10)
      ;
      *(int *)((int)this + 0x18) = iVar2;
      *(int *)((int)this + 0x1c) = iVar2;
      *(undefined4 *)((int)this + 0x20) = *(undefined4 *)((int)this + 0x14);
    }
    else {
      iVar2 = *(int *)((int)this + 0x18);
      iVar1 = *(int *)((int)this + 0x1c);
      *(int *)((int)this + 0x1c) = iVar2;
      *(int *)((int)this + 0x20) = *(int *)((int)this + 0x14);
      if (*(int *)((int)this + 0x14) != 0) {
        *(int *)((int)this + 0x1c) = iVar2 + (iVar1 - iVar2);
      }
    }
  }
  if ((param_1 != -1) &&
     (**(undefined1 **)((int)this + 0x1c) = (char)param_1, *(int *)((int)this + 0x20) != 0)) {
    *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 1;
  }
  return 1;
}

