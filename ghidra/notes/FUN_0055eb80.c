
undefined4 __cdecl FUN_0055eb80(int param_1,int *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  
  *param_2 = 0;
  if (0 < param_1) {
    do {
      bVar1 = FUN_0053e2f0(DAT_006bb77c);
      param_1 = param_1 + -1;
      *param_2 = *param_2 + (uint)(CONCAT31(extraout_var,bVar1) != 0);
    } while (param_1 != 0);
  }
  return 1;
}

