
void __thiscall FUN_004abe10(void *this,undefined4 param_1,undefined4 param_2,int param_3)

{
  *(undefined4 *)((int)this + 0x2c) = param_1;
  *(undefined4 *)((int)this + 0x30) = param_2;
  *(int *)((int)this + 0x34) = param_3;
  if (param_3 == 0) {
    *(undefined4 *)((int)this + 0x34) = param_2;
  }
  return;
}

