
void __thiscall FUN_00402120(int param_1,undefined4 param_2)

{
  int iVar1;
  
  FUN_0058f0d0(param_2);
  FUN_005f4db0(param_2,param_1 + 0x1c);
  FUN_005f4db0(param_2,param_1 + 0x20);
  FUN_005f4db0(param_2,param_1 + 0x24);
  FUN_005f4db0(param_2,param_1 + 0x28);
  FUN_005f4db0(param_2,param_1 + 0x2c);
  FUN_005f49a0(param_2);
  FUN_005f49a0(param_2);
  FUN_004ecea0(param_2);
  param_1 = param_1 + 0x40;
  iVar1 = 10;
  do {
    FUN_005f4db0(param_2,param_1);
    param_1 = param_1 + 4;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  return;
}

