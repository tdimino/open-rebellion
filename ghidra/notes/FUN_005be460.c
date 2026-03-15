
void __thiscall FUN_005be460(void *this,int param_1)

{
  void *this_00;
  int iVar1;
  
  FUN_005a6660((void *)(param_1 + 0xd4),*(uint *)(param_1 + 0x168));
  iVar1 = param_1;
  this_00 = (void *)FUN_00596bd0();
  FUN_005a64e0(this_00,iVar1);
  *(undefined4 *)(param_1 + 0x124) = 0;
  if (*(int *)(param_1 + 0x168) != *(int *)((int)this + 0x9ec)) {
    *(undefined4 *)(param_1 + 0x11c) = 1;
    *(undefined4 *)(param_1 + 0x41c) = *(undefined4 *)(param_1 + 0xec);
    return;
  }
  *(undefined4 *)(param_1 + 0x11c) = 0;
  *(undefined4 *)(param_1 + 0x41c) = *(undefined4 *)(param_1 + 0xec);
  return;
}

