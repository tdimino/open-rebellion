
uint __thiscall FUN_005a02c0(int param_1,int param_2)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  
  uVar1 = 0;
  piVar3 = (int *)(param_1 + 0x154);
  do {
    if (*piVar3 == param_2) {
      return uVar1;
    }
    uVar1 = uVar1 + 1;
    piVar3 = piVar3 + 1;
  } while (uVar1 < 4);
  FUN_00596bd0(param_2);
  iVar2 = FUN_005a6470(param_2);
  if (iVar2 == 0) {
    return 0xffffffff;
  }
  uVar1 = FUN_005c7150();
  return uVar1;
}

