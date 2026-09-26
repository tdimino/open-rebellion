
int * __fastcall FUN_004c7580(int param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)(param_1 + 0x38)) {
  case 1:
    iVar1 = FUN_004c7860(param_1);
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x38) = 3;
      return (int *)0x0;
    }
    break;
  case 2:
    iVar1 = FUN_004c7650(param_1);
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x38) = 1;
      return (int *)0x0;
    }
    break;
  case 3:
    iVar1 = FUN_004c79e0(param_1);
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x38) = 4;
      return (int *)0x0;
    }
    break;
  case 4:
    piVar2 = FUN_004c7a90(param_1);
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)(param_1 + 0x38) = 5;
      *(undefined4 *)(param_1 + 0x20) = 1;
      *(int *)(param_1 + 0x30) = *(int *)(param_1 + 0x30) + 1;
      return piVar2;
    }
    break;
  case 5:
    piVar3 = FUN_004c7bd0(param_1);
    break;
  default:
    *(undefined4 *)(param_1 + 0x38) = 2;
    return (int *)0x0;
  }
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x20) = 1;
  return piVar3;
}

