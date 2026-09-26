
int __cdecl FUN_00618b90(int param_1)

{
  SIZE_T SVar1;
  int *piVar2;
  
  FUN_0061d700();
  SVar1 = FUN_0061d8f0((undefined *)DAT_006bed1c);
  if (SVar1 < (uint)((int)DAT_006bed18 + (4 - (int)DAT_006bed1c))) {
    SVar1 = FUN_0061d8f0((undefined *)DAT_006bed1c);
    piVar2 = FUN_0061d740(DAT_006bed1c,SVar1 + 0x10);
    if (piVar2 == (int *)0x0) {
      FUN_0061d710();
      return 0;
    }
    DAT_006bed18 = piVar2 + ((int)DAT_006bed18 - (int)DAT_006bed1c >> 2);
    DAT_006bed1c = piVar2;
  }
  *DAT_006bed18 = param_1;
  DAT_006bed18 = DAT_006bed18 + 1;
  FUN_0061d710();
  return param_1;
}

