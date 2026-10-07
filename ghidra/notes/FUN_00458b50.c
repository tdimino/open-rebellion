
void __thiscall FUN_00458b50(void *this,undefined4 param_1)

{
  *(undefined4 *)((int)this + 0x268) = param_1;
  if ((*(uint *)((int)this + 0x164) & 8) == 0) {
    *(uint *)((int)this + 0x164) = *(uint *)((int)this + 0x164) & 0xfffffffb | 8;
    FUN_00458640(this,*(int **)((int)this + 0x248));
  }
  return;
}

