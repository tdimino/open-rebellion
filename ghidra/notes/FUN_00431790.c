
void __thiscall FUN_00431790(void *this,undefined4 param_1,undefined4 param_2)

{
  *(undefined4 *)((int)this + 0x30) = param_1;
  *(undefined4 *)((int)this + 0x34) = param_2;
  *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) & 0xdfffffff;
  FUN_004321d0((int)this);
  return;
}

