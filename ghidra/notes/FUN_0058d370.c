
void __thiscall FUN_0058d370(void *this,undefined4 *param_1,int *param_2)

{
  *param_1 = 1;
  *param_2 = 0;
  param_2[1] = 0;
  param_2[2] = 0;
  param_2[3] = 0;
  if (*(int *)((int)this + 0x10) != 0) {
    *param_2 = *(int *)((int)this + 0x10);
    return;
  }
  if (*(int *)((int)this + 0x14) != 0) {
    param_2[1] = *(int *)((int)this + 0x14);
    return;
  }
  if (*(int *)((int)this + 0x18) != 0) {
    param_2[2] = 1;
    return;
  }
  if (*(int *)((int)this + 0x1c) != 0) {
    param_2[3] = 1;
    return;
  }
  *param_1 = 0;
  return;
}

