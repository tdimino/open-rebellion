
float10 __thiscall FUN_005a8ad0(void *param_1,int param_2)

{
  if (param_2 < 2) {
    return (float10)*(float *)(*(int *)((int)param_1 + param_2 * 4 + 8) + 0x7c);
  }
  return (float10)*(float *)(*(int *)((int)param_1 + 8) + 0x7c);
}

