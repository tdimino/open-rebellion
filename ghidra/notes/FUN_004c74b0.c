
void __thiscall FUN_004c74b0(void *param_1,void *param_2)

{
  FUN_004c7dc0(param_1,param_2);
  FUN_005f4db0(param_2,(int)param_1 + 0x34);
  FUN_005f4db0(param_2,(int)param_1 + 0x3c);
  FUN_005f4db0(param_2,(int)param_1 + 0x38);
  FUN_004ecea0((void *)((int)param_1 + 0x40),param_2);
  FUN_004ecea0((void *)((int)param_1 + 0x44),param_2);
  (**(code **)(*(int *)((int)param_1 + 0x4c) + 0xc))(param_2);
  FUN_004ecea0((void *)((int)param_1 + 0x48),param_2);
  return;
}

