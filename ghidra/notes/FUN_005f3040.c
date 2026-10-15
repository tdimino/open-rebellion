
short __fastcall FUN_005f3040(int param_1)

{
  short sVar1;
  int iVar2;
  
  if (*(char **)(param_1 + 4) != (char *)0x0) {
    iVar2 = FUN_005f3630(*(char **)(param_1 + 4));
    sVar1 = FUN_00479ac0();
    return (short)iVar2 + sVar1;
  }
  return 0;
}

