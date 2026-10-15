
undefined4 FUN_005f6be0(undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  if (DAT_006be328 != 0) {
    if (param_4 == 2) {
      uVar1 = FUN_005f7940(param_2,param_3,*(undefined4 *)(DAT_006be328 + 0x18),param_1);
      return uVar1;
    }
    if (param_4 == 1) {
      uVar1 = FUN_005f71f0(param_2,param_3,*(undefined4 *)(DAT_006be328 + 0x18),param_1);
    }
  }
  return uVar1;
}

