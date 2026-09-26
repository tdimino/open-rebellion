
void __thiscall FUN_005b2440(int param_1,uint param_2)

{
  void *pvVar1;
  
  pvVar1 = (void *)FUN_00596bd0();
  pvVar1 = (void *)FUN_005a6340(pvVar1,param_2);
  *(undefined4 *)(param_1 + 0x380) = 0;
  *(undefined4 *)(param_1 + 0x3ac) = 0;
  if (pvVar1 != (void *)0x0) {
    FUN_005b25d0(pvVar1,*(uint *)(param_1 + 0xec));
  }
  return;
}

