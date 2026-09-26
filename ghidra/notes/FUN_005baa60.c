
undefined4 __thiscall FUN_005baa60(void *this,undefined4 *param_1,uint param_2)

{
  int iVar1;
  
  if ((*(int *)((int)this + 0x18) != 0) && (param_1 != (undefined4 *)0x0)) {
    if (0x13c < (uint)param_1[7]) {
      (**(code **)*param_1)(1);
      return 0;
    }
    param_1[0xf] = (param_2 & 1) << 3 | param_1[0xf] & 0xfffffff7;
    FUN_005bab50(this);
    iVar1 = FUN_005bad50(this,(int)param_1);
    if (iVar1 == 0) {
      if ((*(byte *)(param_1 + 0xf) & 8) == 0) {
        FUN_005f5440((void *)((int)this + 0xc),param_1);
        return param_1[6];
      }
      (**(code **)*param_1)(1);
      return 0;
    }
    iVar1 = FUN_005f5500(this,param_1[6]);
    if (iVar1 == 0) {
      FUN_005f5440(this,param_1);
      return param_1[6];
    }
  }
  return 0;
}

