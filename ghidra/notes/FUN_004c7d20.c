
void * __thiscall FUN_004c7d20(void *this,undefined4 param_1,undefined4 param_2)

{
  FUN_004ec940(this,param_1);
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined ***)this = &PTR_FUN_0065c670;
  *(undefined4 *)((int)this + 0x2c) = param_2;
  return this;
}

