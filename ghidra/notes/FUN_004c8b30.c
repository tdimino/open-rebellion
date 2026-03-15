
int * __fastcall FUN_004c8b30(void *param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_005f4960((undefined4 *)((int)param_1 + 0x1c));
  if (iVar1 != 1) {
    *(undefined4 *)((int)param_1 + 0x38) = 0;
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  }
  iVar1 = *(int *)((int)param_1 + 0x38);
  if (iVar1 == 0x3ec) {
    piVar2 = FUN_004c8d20(param_1);
    return piVar2;
  }
  if (iVar1 == 0x3f8) {
    piVar2 = FUN_004c8ba0((int)param_1);
    return piVar2;
  }
  if (iVar1 != 0x3f9) {
    *(undefined4 *)((int)param_1 + 0x38) = 0x3f8;
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    return (int *)0x0;
  }
  piVar2 = FUN_004c8c20((int)param_1);
  return piVar2;
}

