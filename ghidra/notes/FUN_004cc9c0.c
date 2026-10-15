
int * __fastcall FUN_004cc9c0(void *param_1)

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
  if (*(int *)((int)param_1 + 0x38) != 0x3ec) {
    if (*(int *)((int)param_1 + 0x38) != 0x3f2) {
      *(undefined4 *)((int)param_1 + 0x38) = 0x3f2;
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      return (int *)0x0;
    }
    piVar2 = FUN_004cca20(param_1);
    return piVar2;
  }
  piVar2 = FUN_004ccb50(param_1);
  return piVar2;
}

