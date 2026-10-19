
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005a8b10(void *this,int param_1,float param_2)

{
  if (param_1 < 2) {
    *(float *)(*(int *)((int)this + param_1 * 4 + 8) + 0x7c) = param_2 * _DAT_0066c288;
  }
  return;
}

