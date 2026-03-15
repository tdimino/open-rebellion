
int * __fastcall FUN_004c8ba0(int param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = *(int *)(param_1 + 0x3c);
  if (iVar1 == 2) {
    piVar2 = FUN_004c8e90(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 3;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)(param_1 + 0x20) = 1;
      *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) + 1;
    }
  }
  else {
    if (iVar1 != 3) {
      if (iVar1 != 4) {
        *(undefined4 *)(param_1 + 0x3c) = 2;
        return (int *)0x0;
      }
      piVar2 = FUN_004c9cb0(param_1);
      *(undefined4 *)(param_1 + 0x3c) = 0;
      *(undefined4 *)(param_1 + 0x38) = 0x3f9;
      *(undefined4 *)(param_1 + 0x20) = 1;
      return piVar2;
    }
    piVar2 = FUN_004c94c0(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 4;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)(param_1 + 0x20) = 1;
      *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) + 1;
      return piVar2;
    }
  }
  return piVar2;
}

