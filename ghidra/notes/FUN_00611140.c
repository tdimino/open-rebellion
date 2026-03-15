
int __cdecl FUN_00611140(undefined4 param_1,undefined4 *param_2)

{
  int iVar1;
  
  if (DAT_006be668 == (int *)0x0) {
    iVar1 = 6;
  }
  else {
    iVar1 = (**(code **)(*DAT_006be668 + 0xc))(DAT_006be668,param_1,param_2,0);
    if (iVar1 == 0) {
      if (0 < DAT_006be67c) {
        if (DAT_006be67c < 0x2711) {
          DAT_006be67c = -DAT_006be67c;
          (**(code **)(*(int *)*param_2 + 0x3c))((int *)*param_2,DAT_006be67c);
          return 0;
        }
        DAT_006be67c = 0;
      }
      (**(code **)(*(int *)*param_2 + 0x3c))((int *)*param_2,DAT_006be67c);
      return 0;
    }
  }
  return iVar1;
}

