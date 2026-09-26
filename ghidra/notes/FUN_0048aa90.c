
void __thiscall FUN_0048aa90(void *this,undefined4 *param_1)

{
  undefined4 uVar1;
  
  if (param_1 != (undefined4 *)0x0) {
    if (param_1[6] == 0) {
      uVar1 = (**(code **)(*(int *)this + 0x18))();
      FUN_005f5600(param_1,uVar1);
    }
    FUN_0041cee0(param_1);
  }
  return;
}

