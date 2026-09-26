
void __thiscall FUN_004189b0(void *this,uint param_1)

{
  *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 2;
  *(undefined4 *)((int)this + 400) = 0;
  *(undefined4 *)((int)this + 0x194) = 0;
  if (*(int *)this == 1) {
    if ((param_1 & 0x300) != 0) {
      *(undefined4 *)((int)this + 400) = 1;
    }
    if ((param_1 & 0x1000) != 0) {
      *(int *)((int)this + 400) = *(int *)((int)this + 400) + 1;
    }
    if ((param_1 & 0x2000) != 0) {
      *(int *)((int)this + 400) = *(int *)((int)this + 400) + 1;
    }
    if ((param_1 & 1) != 0) {
      *(undefined4 *)((int)this + 0x194) = 1;
    }
    if ((param_1 & 0x10) != 0) {
      *(int *)((int)this + 0x194) = *(int *)((int)this + 0x194) + 1;
    }
    if ((param_1 & 0x20) != 0) {
      *(int *)((int)this + 0x194) = *(int *)((int)this + 0x194) + 1;
      return;
    }
  }
  else {
    if ((param_1 & 1) != 0) {
      *(undefined4 *)((int)this + 400) = 1;
    }
    if ((param_1 & 0x10) != 0) {
      *(int *)((int)this + 400) = *(int *)((int)this + 400) + 1;
    }
    if ((param_1 & 0x20) != 0) {
      *(int *)((int)this + 400) = *(int *)((int)this + 400) + 1;
    }
    if ((param_1 & 0x300) != 0) {
      *(undefined4 *)((int)this + 0x194) = 1;
    }
    if ((param_1 & 0x1000) != 0) {
      *(int *)((int)this + 0x194) = *(int *)((int)this + 0x194) + 1;
    }
    if ((param_1 & 0x2000) != 0) {
      *(int *)((int)this + 0x194) = *(int *)((int)this + 0x194) + 1;
    }
  }
  return;
}

