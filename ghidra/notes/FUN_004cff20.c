
int * __fastcall FUN_004cff20(int param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)(param_1 + 0x3c)) {
  case 1:
    iVar2 = FUN_004d0480(param_1);
    *(uint *)(param_1 + 0x3c) = 6 - (uint)(iVar2 != 0);
    return (int *)0x0;
  case 2:
    piVar3 = FUN_004d0610(param_1);
    break;
  default:
    *(undefined4 *)(param_1 + 0x3c) = 9;
    return (int *)0x0;
  case 5:
    iVar2 = FUN_004d0590(param_1);
    *(int *)(param_1 + 0x3c) = iVar2;
    if (iVar2 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)(param_1 + 0x38) = 0x3ec;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return (int *)0x0;
  case 6:
    bVar1 = FUN_004d0570(param_1);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      *(undefined4 *)(param_1 + 0x3c) = 2;
      return (int *)0x0;
    }
    break;
  case 9:
    iVar2 = FUN_004d01a0(param_1);
    *(uint *)(param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 0xfffffffb) + 6;
    return (int *)0x0;
  case 0xb:
    piVar3 = FUN_004d15f0(param_1);
  }
  *(undefined4 *)(param_1 + 0x3c) = 0;
  *(undefined4 *)(param_1 + 0x38) = 0x3ec;
  *(undefined4 *)(param_1 + 0x20) = 1;
  return piVar3;
}

