
uint __thiscall FUN_00409fe0(void *this,void *param_1,undefined4 param_2,int param_3)

{
  uint uVar1;
  
  uVar1 = 1;
  *(undefined4 *)((int)this + 4) = 1;
  *(undefined4 *)((int)this + 8) = 0;
  if (DAT_006b1218 != (undefined4 *)0x0) {
    (**(code **)*DAT_006b1218)(1);
    DAT_006b1218 = (undefined4 *)0x0;
  }
  FUN_005975a0();
  if ((*(int *)((int)param_1 + 4) < 1) || (3 < *(int *)((int)param_1 + 4))) {
    uVar1 = 0;
  }
  if (uVar1 != 0) {
    uVar1 = FUN_0040adb0(param_1,param_2,param_3);
    if (uVar1 != 0) {
      *(undefined4 *)((int)this + 8) = 1;
    }
  }
  return uVar1;
}

