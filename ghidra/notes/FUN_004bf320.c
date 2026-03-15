
int * __fastcall FUN_004bf320(int param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_005f4960((undefined4 *)(param_1 + 0x1c));
  if (iVar1 != 1) {
    *(undefined4 *)(param_1 + 0x38) = 0;
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return (int *)0x0;
  }
  if (*(int *)(param_1 + 0x38) == 0x3fb) {
    FUN_004bf390(param_1);
    return (int *)0x0;
  }
  if (*(int *)(param_1 + 0x38) != 0x3fc) {
    *(undefined4 *)(param_1 + 0x38) = 0x3fb;
    return (int *)0x0;
  }
  if (*(int *)(param_1 + 0x30) == 1) {
    piVar2 = FUN_004bf3e0(param_1);
    return piVar2;
  }
  piVar2 = (int *)FUN_004bf480(param_1);
  return piVar2;
}

