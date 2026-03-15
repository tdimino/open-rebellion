
int * __fastcall FUN_004dd840(int param_1)

{
  int *piVar1;
  
  switch(*(undefined4 *)(param_1 + 0x3c)) {
  case 1:
    piVar1 = FUN_004ddb70(param_1);
    if (piVar1 == (int *)0x0) {
      *(undefined4 *)(param_1 + 0x3c) = 2;
      return (int *)0x0;
    }
    break;
  case 2:
    piVar1 = FUN_004de130(param_1);
    if (piVar1 == (int *)0x0) {
      *(undefined4 *)(param_1 + 0x3c) = 3;
      return (int *)0x0;
    }
    break;
  case 3:
    piVar1 = FUN_004de6f0(param_1);
    if (piVar1 == (int *)0x0) {
      *(undefined4 *)(param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    break;
  case 4:
    piVar1 = FUN_004deba0(param_1);
    break;
  default:
    *(undefined4 *)(param_1 + 0x3c) = 1;
    return (int *)0x0;
  }
  *(undefined4 *)(param_1 + 0x3c) = 0;
  *(undefined4 *)(param_1 + 0x38) = 0x3f0;
  *(undefined4 *)(param_1 + 0x20) = 1;
  return piVar1;
}

