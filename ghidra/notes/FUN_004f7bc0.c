
void __thiscall FUN_004f7bc0(void *param_1,void *param_2)

{
  if (((*(uint *)((int)param_1 + 0x50) & 2) != 0) && ((*(uint *)((int)param_1 + 0x50) & 8) == 0)) {
    FUN_004f76b0(param_1,1,param_2);
    return;
  }
  FUN_004f76b0(param_1,0,param_2);
  return;
}

