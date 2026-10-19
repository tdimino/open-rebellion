
float10 __thiscall FUN_005a9610(void *param_1,int param_2)

{
  if (param_2 == 0) {
    (**(code **)(**(int **)((int)param_1 + 0xc) + 0x14))(0);
    return (float10)*(float *)(*(int *)((int)param_1 + 0xc) + 0x3c);
  }
  (**(code **)(**(int **)((int)param_1 + 8) + 0x14))(0);
  return (float10)*(float *)(*(int *)((int)param_1 + 8) + 0x3c);
}

