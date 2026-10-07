
void __thiscall FUN_0043a2a0(void *this,int param_1)

{
  if (((DAT_006b28b0 & 0x10000000) != 0) && (param_1 == *(int *)((int)this + 0x180))) {
    DAT_006b28b0 = DAT_006b28b0 | 1;
  }
  DAT_006b28b0 = DAT_006b28b0 & 0xefffffff;
  return;
}

