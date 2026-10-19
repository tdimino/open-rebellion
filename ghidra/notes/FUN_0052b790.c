
void __thiscall FUN_0052b790(void *param_1,undefined4 param_2,undefined4 param_3,void *param_4)

{
  uint uVar1;
  
  uVar1 = FUN_004fae40(param_1,param_2,param_3,param_4);
  if (uVar1 != 0) {
    if ((*(byte *)((int)param_1 + 0x50) & 2) != 0) {
      uVar1 = FUN_0052a2c0(param_1,param_4);
    }
    if (uVar1 != 0) {
      FUN_0052a360(param_1,0,param_4);
    }
  }
  return;
}

