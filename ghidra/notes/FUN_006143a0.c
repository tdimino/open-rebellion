
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int FUN_006143a0(int param_1,int param_2,undefined4 param_3,int param_4)

{
  int iVar1;
  
  iVar1 = -0x7fffbffb;
  if (DAT_006be700 == 1) {
    if (DAT_006be6d4 != (int *)0x0) {
      iVar1 = (**(code **)(*DAT_006be6d4 + 0x54))(DAT_006be6d4,param_1,param_2,param_3,0,0);
    }
    if (iVar1 != 0) goto LAB_00614412;
    _DAT_006be6f8 = param_3;
  }
  else if (param_4 == 1) {
    iVar1 = FUN_00614430(DAT_006be6ec,param_1,param_2);
  }
  else {
    iVar1 = 0;
  }
  if (iVar1 == 0) {
    _DAT_006be6f0 = param_1;
    _DAT_006be6f4 = param_2;
  }
LAB_00614412:
  FUN_00614530(iVar1,(HWND)0x0);
  return iVar1;
}

