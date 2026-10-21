
int __thiscall FUN_005f56b0(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  
  *(undefined4 *)((int)this + 4) = 0xffffffff;
  if ((param_1 < 1) || (0x7ffffffe < param_1)) {
    param_1 = FUN_005f5750();
  }
  *(int *)((int)this + 4) = param_1;
  iVar2 = 0x10;
  do {
    iVar1 = FUN_005f57a0(*(int *)((int)this + 4));
    iVar2 = iVar2 + -1;
    *(int *)((int)this + 4) = iVar1;
  } while (iVar2 != 0);
  return param_1;
}

