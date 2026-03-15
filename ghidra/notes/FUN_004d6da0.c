
int * __fastcall FUN_004d6da0(int param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int *piVar3;
  
  switch(*(undefined4 *)(param_1 + 0x3c)) {
  case 1:
    break;
  case 2:
    piVar3 = FUN_004d7330(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0x3ee;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return piVar3;
  case 3:
    bVar1 = FUN_004d76a0(param_1);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      *(undefined4 *)(param_1 + 0x3c) = 0;
      *(undefined4 *)(param_1 + 0x38) = 0x3ee;
    }
    else {
      *(undefined4 *)(param_1 + 0x3c) = 7;
    }
  case 7:
    piVar3 = FUN_004d8520(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0x3ec;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return piVar3;
  default:
    *(undefined4 *)(param_1 + 0x3c) = 1;
    return (int *)0x0;
  }
  iVar2 = FUN_004d71e0(param_1);
  if (iVar2 != 0) {
    *(undefined4 *)(param_1 + 0x3c) = 2;
    return (int *)0x0;
  }
  *(undefined4 *)(param_1 + 0x3c) = 0;
  *(undefined4 *)(param_1 + 0x38) = 0x3ee;
  return (int *)0x0;
}

