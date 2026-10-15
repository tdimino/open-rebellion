
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __thiscall
FUN_005c9160(void *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5)

{
  float10 fVar1;
  
  if (*(int **)((int)param_1 + 0x2c) != (int *)0x0) {
    fVar1 = (float10)(**(code **)(**(int **)((int)param_1 + 0x2c) + 4))
                               (param_2,param_3,param_4,param_5);
    return fVar1;
  }
  return (float10)_DAT_0066cbdc;
}

