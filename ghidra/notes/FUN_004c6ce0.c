
int * __fastcall FUN_004c6ce0(int param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)(param_1 + 0x38)) {
  case 1:
    iVar1 = FUN_004c6d90(param_1);
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x38) = 2;
      return (int *)0x0;
    }
    break;
  case 2:
    iVar1 = FUN_004c6f00(param_1);
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x38) = 3;
      return (int *)0x0;
    }
    break;
  case 3:
    piVar2 = FUN_004c70e0(param_1);
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)(param_1 + 0x38) = 4;
      *(undefined4 *)(param_1 + 0x20) = 1;
      *(int *)(param_1 + 0x30) = *(int *)(param_1 + 0x30) + 1;
      return piVar2;
    }
    break;
  case 4:
    piVar3 = FUN_004c7220(param_1);
    break;
  default:
    *(undefined4 *)(param_1 + 0x38) = 1;
    return (int *)0x0;
  }
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x20) = 1;
  return piVar3;
}

