
void __thiscall FUN_0041bc10(void *this,undefined4 param_1,int param_2,uint *param_3,uint *param_4)

{
  if ((*param_3 & 2) != 0) {
    *(int *)((int)this + 0x2c4) = *(int *)((int)this + 0x2c4) + *(int *)(param_2 + 0x24);
    if ((*param_3 & 0x20000000) == 0) {
      if ((*param_4 & 0x10000000) == 0) {
        if (((*param_4 & 0x20000000) != 0) &&
           (*(int *)((int)this + 0x2e0) = *(int *)((int)this + 0x2e0) + 1,
           (*param_4 & 0x800000) != 0)) {
          *(int *)((int)this + 0x2f0) = *(int *)((int)this + 0x2f0) + 1;
        }
      }
      else {
        *(int *)((int)this + 0x2dc) = *(int *)((int)this + 0x2dc) + 1;
        if ((*param_4 & 0x800000) != 0) {
          *(int *)((int)this + 0x2ec) = *(int *)((int)this + 0x2ec) + 1;
        }
      }
      if (((*param_4 & 0x40000000) != 0) &&
         (*(int *)((int)this + 0x2e4) = *(int *)((int)this + 0x2e4) + 1, (*param_4 & 0x800000) != 0)
         ) {
        *(int *)((int)this + 0x2f4) = *(int *)((int)this + 0x2f4) + 1;
      }
      if (((*param_4 & 0x80000000) != 0) &&
         (*(int *)((int)this + 0x2e8) = *(int *)((int)this + 0x2e8) + 1, (*param_4 & 0x800000) != 0)
         ) {
        *(int *)((int)this + 0x2f8) = *(int *)((int)this + 0x2f8) + 1;
      }
    }
    else {
      if ((*param_3 & 0x40) != 0) {
        *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x200;
      }
      if ((*param_4 & 0x2000) != 0) {
        *(int *)((int)this + 0x2c8) = *(int *)((int)this + 0x2c8) + 1;
      }
      if ((*param_4 & 0x800) != 0) {
        *(int *)((int)this + 0x2cc) = *(int *)((int)this + 0x2cc) + 1;
      }
      if ((*param_4 & 0x1000) != 0) {
        *(int *)((int)this + 0x2d0) = *(int *)((int)this + 0x2d0) + 1;
        return;
      }
    }
  }
  return;
}

