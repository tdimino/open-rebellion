
void __thiscall FUN_00477e80(void *this,undefined4 *param_1)

{
  int *piVar1;
  
  piVar1 = (int *)0x0;
  *param_1 = 0;
  if ((((((*(int *)((int)this + 0x6c) < 0) || (*(int *)((int)this + 0x70) < 0)) ||
        (*(int *)((int)this + 0x74) < 0)) ||
       ((*(int *)((int)this + 0x78) < 0 || (*(int *)((int)this + 0x80) < 0)))) ||
      (*(int *)((int)this + 0x7c) < 0)) ||
     ((*(int *)((int)this + 0x84) < 0 || (*(int *)((int)this + 0x90) < 0)))) {
    piVar1 = FUN_004782a0(this);
    if ((piVar1 == (int *)0x0) &&
       ((((*(int *)((int)this + 0x6c) < 0 || (*(int *)((int)this + 0x70) < 0)) ||
         (*(int *)((int)this + 0x74) < 0)) ||
        (((*(int *)((int)this + 0x78) < 0 || (*(int *)((int)this + 0x80) < 0)) ||
         ((*(int *)((int)this + 0x7c) < 0 ||
          ((*(int *)((int)this + 0x84) < 0 || (*(int *)((int)this + 0x90) < 0)))))))))) {
      piVar1 = FUN_00478690(this);
    }
    if (*(int *)((int)this + 0x6c) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 1;
    }
    if (*(int *)((int)this + 0x70) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 2;
    }
    if (*(int *)((int)this + 0x74) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 4;
    }
    if (*(int *)((int)this + 0x78) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 8;
    }
    if (*(int *)((int)this + 0x80) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x10;
    }
    if (*(int *)((int)this + 0x88) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x20;
    }
    if (*(int *)((int)this + 0x8c) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x40;
    }
    if (*(int *)((int)this + 0x90) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x80;
    }
    if (*(int *)((int)this + 0x84) < 0) {
      *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x800;
    }
  }
  if ((piVar1 == (int *)0x0) &&
     ((((0 < *(int *)((int)this + 0x6c) || (0 < *(int *)((int)this + 0x70))) ||
       (0 < *(int *)((int)this + 0x74))) ||
      (((0 < *(int *)((int)this + 0x78) || (0 < *(int *)((int)this + 0x80))) ||
       ((0 < *(int *)((int)this + 0x7c) ||
        ((0 < *(int *)((int)this + 0x84) || (0 < *(int *)((int)this + 0x90))))))))))) {
    FUN_00478aa0(this);
  }
  *param_1 = 1;
  return;
}

