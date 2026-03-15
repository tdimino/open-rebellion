
undefined4 * __cdecl FUN_00628c80(int *param_1)

{
  int iVar1;
  int *piVar2;
  undefined4 *puVar3;
  char *pcVar4;
  int iVar5;
  undefined4 *puVar6;
  
  iVar5 = 0;
  if (param_1 != (int *)0x0) {
    iVar1 = *param_1;
    piVar2 = param_1;
    while (iVar1 != 0) {
      piVar2 = piVar2 + 1;
      iVar5 = iVar5 + 1;
      iVar1 = *piVar2;
    }
    puVar3 = (undefined4 *)FUN_00619170(iVar5 * 4 + 4);
    if (puVar3 == (undefined4 *)0x0) {
      __amsg_exit(9);
    }
    pcVar4 = (char *)*param_1;
    puVar6 = puVar3;
    while (pcVar4 != (char *)0x0) {
      param_1 = param_1 + 1;
      pcVar4 = FUN_00628dc0(pcVar4);
      *puVar6 = pcVar4;
      puVar6 = puVar6 + 1;
      pcVar4 = (char *)*param_1;
    }
    *puVar6 = 0;
    return puVar3;
  }
  return (undefined4 *)0x0;
}

