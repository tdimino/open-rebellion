
uint __thiscall FUN_0054fd90(void *this,int param_1,int param_2,int param_3)

{
  uint uVar1;
  
  uVar1 = 1;
  if (*(int *)(param_2 + 4) != 0) {
    uVar1 = FUN_0054fca0(this,param_1,param_2,param_3);
    if (uVar1 != 0) {
      if (param_3 == 0) {
        if ((*(int *)((int)this + 0x10) == -1) && (*(int *)(param_1 + 4) == 0)) {
          *(undefined4 *)(param_1 + 8) = *(undefined4 *)((int)this + 8);
        }
      }
      else {
        uVar1 = (uint)(*(int *)((int)this + 0x10) == -1);
        if (((uVar1 != 0) && (uVar1 = (uint)(*(int *)((int)this + 0x18) == -1), uVar1 != 0)) &&
           (*(int *)(param_1 + 4) == 0)) {
          *(undefined4 *)(param_1 + 8) = *(undefined4 *)((int)this + 8);
          return uVar1;
        }
      }
    }
  }
  return uVar1;
}

