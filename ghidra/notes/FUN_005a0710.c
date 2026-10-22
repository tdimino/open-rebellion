
void __fastcall FUN_005a0710(void *param_1)

{
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)FUN_005f5e90((int)param_1 + 0xfc);
  while (puVar1 != (undefined4 *)0x0) {
    FUN_005a04f0(param_1,(int)puVar1);
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)*puVar1)(1);
    }
    puVar1 = (undefined4 *)FUN_005f5e90((int)param_1 + 0xfc);
  }
  return;
}

