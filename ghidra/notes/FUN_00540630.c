
void __thiscall FUN_00540630(void *this,uint param_1)

{
  *(uint *)((int)this + 0x24) =
       (param_1 ^ *(uint *)((int)this + 0x24)) & 0xf ^ *(uint *)((int)this + 0x24);
  return;
}

