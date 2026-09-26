
undefined4 __cdecl FUN_00584b60(int *param_1,undefined4 param_2)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  
  bVar1 = true;
  iVar2 = 0;
  piVar3 = &DAT_006bbb54;
  do {
    if (DAT_006bbb48 <= iVar2) break;
    if ((piVar3[-1] == *param_1) && (*piVar3 == param_1[1])) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    bVar1 = !bVar1;
    FUN_00619730();
    iVar2 = iVar2 + 1;
    piVar3 = piVar3 + 3;
  } while (bVar1);
  if ((bVar1) && (DAT_006bbb48 + 1U < 0x3d)) {
    (&DAT_006bbb50)[DAT_006bbb48 * 3] = *param_1;
    (&DAT_006bbb54)[DAT_006bbb48 * 3] = param_1[1];
    (&DAT_006bbb58)[DAT_006bbb48 * 3] = param_2;
    DAT_006bbb48 = DAT_006bbb48 + 1;
  }
  return 0;
}

