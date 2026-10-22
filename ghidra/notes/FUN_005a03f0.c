
void __thiscall FUN_005a03f0(void *this,uint param_1,undefined4 param_2)

{
  if (param_1 < 8) {
    *(undefined4 *)((int)this + param_1 * 4 + 0x164) = param_2;
    FUN_0059fc50(this,0xffffffff);
  }
  return;
}

