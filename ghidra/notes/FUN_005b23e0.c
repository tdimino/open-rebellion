
void __thiscall FUN_005b23e0(int param_1,uint param_2)

{
  void *pvVar1;
  uint uVar2;
  
  uVar2 = param_2;
  pvVar1 = (void *)FUN_00596bd0();
  pvVar1 = (void *)FUN_005a6340(pvVar1,uVar2);
  *(uint *)(param_1 + 0x380) = param_2;
  FUN_005b2570(pvVar1,param_1);
  *(undefined4 *)(param_1 + 0x3ac) = 0;
  return;
}

