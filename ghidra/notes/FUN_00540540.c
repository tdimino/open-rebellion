
undefined4 __cdecl FUN_00540540(int param_1,undefined4 param_2)

{
  int *piVar1;
  int iVar2;
  bool bVar3;
  
  bVar3 = true;
  iVar2 = 0;
  piVar1 = &DAT_006b90f8;
  do {
    if (DAT_006b90f0 <= iVar2) break;
    bVar3 = *piVar1 != param_1;
    iVar2 = iVar2 + 1;
    piVar1 = piVar1 + 2;
  } while (bVar3);
  if ((bVar3) && (DAT_006b90f0 + 1U < 0x3d)) {
    (&DAT_006b90f8)[DAT_006b90f0 * 2] = param_1;
    (&DAT_006b90fc)[DAT_006b90f0 * 2] = param_2;
    DAT_006b90f0 = DAT_006b90f0 + 1;
  }
  return 0;
}

