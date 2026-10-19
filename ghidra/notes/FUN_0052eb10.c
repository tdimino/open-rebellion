
undefined4 __thiscall FUN_0052eb10(void *this,int param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  
  uVar1 = 1;
  *param_2 = 0;
  if (param_1 == 0) {
    *param_2 = *(undefined4 *)((int)this + 0xac);
    return uVar1;
  }
  if (param_1 != 1) {
    if (param_1 != 2) {
      return 0;
    }
    *param_2 = *(undefined4 *)((int)this + 0xb0);
    return uVar1;
  }
  *param_2 = *(undefined4 *)((int)this + 0xb4);
  return uVar1;
}

