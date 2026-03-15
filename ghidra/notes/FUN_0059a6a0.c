
/* WARNING: Removing unreachable block (ram,0x0059a6b6) */

undefined4 __thiscall FUN_0059a6a0(void *this,float param_1)

{
  void *this_00;
  int iVar1;
  
  if (*(int *)((int)this + 0x690) == 0) {
    *(undefined4 *)((int)this + 0x690) = 1;
    this_00 = (void *)FUN_005aaf90();
    FUN_005aacb0(this_00,param_1);
    iVar1 = *(int *)((int)this + 0x690) + -1;
    *(int *)((int)this + 0x690) = iVar1;
    if (iVar1 < 0) {
      *(undefined4 *)((int)this + 0x690) = 0;
    }
    iVar1 = FUN_005aaf90();
    if (*(int *)(iVar1 + 0x8c4) != 0) {
      return 1;
    }
  }
  return 0;
}

