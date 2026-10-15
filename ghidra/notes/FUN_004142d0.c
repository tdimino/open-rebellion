
void __thiscall
FUN_004142d0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            int param_5,undefined4 param_6)

{
  *(undefined4 *)((int)this + 0x108) = param_1;
  *(undefined4 *)((int)this + 0xe8) = param_3;
  *(undefined4 *)((int)this + 0xe4) = param_2;
  *(undefined4 *)((int)this + 0x104) = param_6;
  *(undefined4 *)((int)this + 0xf0) = param_4;
  *(undefined4 *)((int)this + 0xec) = 0;
  if (param_5 == 1) {
    *(undefined4 *)((int)this + 0xec) = 0;
  }
  else if (param_5 == 2) {
    *(undefined4 *)((int)this + 0xec) = 1;
    return;
  }
  return;
}

