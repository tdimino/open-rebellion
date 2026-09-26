
int * __fastcall FUN_004bf3e0(int param_1)

{
  int *piVar1;
  int iVar2;
  
  iVar2 = *(int *)(param_1 + 0x3c);
  if (iVar2 == 3) {
    iVar2 = FUN_004bf9e0(param_1);
    if (iVar2 != 0) {
      *(undefined4 *)(param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return (int *)0x0;
  }
  if (iVar2 != 4) {
    if (iVar2 != 5) {
      *(undefined4 *)(param_1 + 0x3c) = 3;
      return (int *)0x0;
    }
    piVar1 = FUN_004c05e0(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return piVar1;
  }
  piVar1 = FUN_004bfe30(param_1);
  if (piVar1 != (int *)0x0) {
    *(undefined4 *)(param_1 + 0x20) = 1;
    *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) + 1;
  }
  *(undefined4 *)(param_1 + 0x3c) = 5;
  return piVar1;
}

