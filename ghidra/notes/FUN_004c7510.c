
int * __fastcall FUN_004c7510(int param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_005f4960((undefined4 *)(param_1 + 0x1c));
  if (iVar1 == 1) {
    piVar2 = FUN_004c7580(param_1);
    return piVar2;
  }
  *(undefined4 *)(param_1 + 0x34) = 0;
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x20) = 1;
  return (int *)0x0;
}

