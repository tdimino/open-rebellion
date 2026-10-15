
void __thiscall FUN_00487f80(void *this,int param_1)

{
  if (*(int *)((int)this + 0xc0) != 0) {
    if (param_1 != 0) {
      DAT_006b28b0 = DAT_006b28b0 | 0x4000;
      return;
    }
    DAT_006b28b0 = DAT_006b28b0 & 0xffffbfff;
  }
  return;
}

