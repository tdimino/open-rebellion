
int * __fastcall FUN_004d3170(int param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int *piVar3;
  
  switch(*(undefined4 *)(param_1 + 0x3c)) {
  case 1:
    iVar2 = FUN_004d34c0(param_1);
    if (iVar2 == 0) {
      *(undefined4 *)(param_1 + 0x3c) = 0;
      *(undefined4 *)(param_1 + 0x38) = 0x3f2;
      return (int *)0x0;
    }
    *(undefined4 *)(param_1 + 0x3c) = 2;
    return (int *)0x0;
  case 2:
    bVar1 = FUN_004d3610(param_1);
    *(uint *)(param_1 + 0x3c) = 4 - (uint)(CONCAT31(extraout_var,bVar1) != 0);
    return (int *)0x0;
  case 3:
    piVar3 = FUN_004d3a90(param_1);
    if (piVar3 == (int *)0x0) {
      *(undefined4 *)(param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    break;
  case 4:
    piVar3 = FUN_004d3db0(param_1);
    break;
  default:
    *(undefined4 *)(param_1 + 0x3c) = 1;
    return (int *)0x0;
  }
  *(undefined4 *)(param_1 + 0x3c) = 0;
  *(undefined4 *)(param_1 + 0x38) = 0x3f2;
  *(undefined4 *)(param_1 + 0x20) = 1;
  return piVar3;
}

