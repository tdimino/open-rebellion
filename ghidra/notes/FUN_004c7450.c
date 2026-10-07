
void __thiscall FUN_004c7450(void *param_1,void *param_2)

{
  FUN_004c7d70(param_1,param_2);
  FUN_005f4d90(param_2,(int)param_1 + 0x34);
  FUN_005f4d90(param_2,(int)param_1 + 0x3c);
  FUN_005f4d90(param_2,(int)param_1 + 0x38);
  FUN_004ece90((void *)((int)param_1 + 0x40),param_2);
  FUN_004ece90((void *)((int)param_1 + 0x44),param_2);
  (**(code **)(*(int *)((int)param_1 + 0x4c) + 8))(param_2);
  FUN_004ece90((void *)((int)param_1 + 0x48),param_2);
  return;
}

