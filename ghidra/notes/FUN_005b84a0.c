
undefined4 __thiscall FUN_005b84a0(void *this,int param_1)

{
  if (*(uint *)((int)this + 8) < *(uint *)((int)this + 0x10)) {
    *(undefined4 *)(param_1 + 0x164) = *(undefined4 *)((int)this + 4);
    *(undefined4 *)(param_1 + 0x170) = 1;
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    FUN_005c5f20(*(void **)((int)this + 0x20),param_1);
    return 1;
  }
  return 0;
}

