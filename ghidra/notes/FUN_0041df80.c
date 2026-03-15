
undefined4 __thiscall FUN_0041df80(void *this,int *param_1)

{
  int iVar1;
  void *this_00;
  
  *param_1 = 1;
  iVar1 = *(int *)((int)this + 0xc);
  if (iVar1 == 1) {
    this_00 = (void *)FUN_0051ce00();
    iVar1 = FUN_0051d800(this_00,param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)this + 0xc) = 2;
    }
  }
  else if (iVar1 == 2) {
    iVar1 = FUN_0041cce0(param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)this + 0xc) = 3;
      return 0;
    }
  }
  else if ((iVar1 == 3) && (*(int *)((int)this + 0x20) != 0)) {
    *(undefined4 *)((int)this + 0xc) = 0;
    return 1;
  }
  return 0;
}

