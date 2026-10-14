
void __thiscall FUN_005a7490(void *this,undefined4 *param_1,int param_2)

{
  int iVar1;
  
  if (param_2 < 2) {
    iVar1 = *(int *)((int)this + param_2 * 4 + 8);
  }
  else {
    iVar1 = *(int *)((int)this + 8);
  }
  *param_1 = *(undefined4 *)(iVar1 + 0x48);
  param_1[1] = *(undefined4 *)(iVar1 + 0x4c);
  param_1[2] = *(undefined4 *)(iVar1 + 0x50);
  param_1[3] = *(undefined4 *)(iVar1 + 0x54);
  return;
}

