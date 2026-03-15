
int * __fastcall FUN_004dd7a0(void *param_1)

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
    piVar2 = FUN_004dda00(param_1);
    return piVar2;
  }
  if (iVar1 == 0x3ef) {
    piVar2 = FUN_004dd840((int)param_1);
    return piVar2;
  }
  if (iVar1 != 0x3f0) {
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ef;
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    return (int *)0x0;
  }
  piVar2 = FUN_004dd8d0(param_1);
  return piVar2;
}

