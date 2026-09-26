
int * __fastcall FUN_004e2120(void *param_1)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  
  iVar2 = FUN_005f4960((undefined4 *)((int)param_1 + 0x1c));
  if (iVar2 != 1) {
    *(undefined4 *)((int)param_1 + 0x38) = 0;
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  }
  uVar1 = *(uint *)((int)param_1 + 0x38);
  if (uVar1 == 0) {
    iVar2 = FUN_004e21d0((int)param_1);
    if (iVar2 != 0) {
      *(uint *)((int)param_1 + 0x38) =
           (*(int *)(*(int *)((int)param_1 + 0x2c) + 0x1d4) <=
           *(int *)(*(int *)((int)param_1 + 0x2c) + 0x1d0)) + 0x3ea;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
    return (int *)0x0;
  }
  if (0x3e9 < uVar1) {
    if (uVar1 < 0x3ec) {
      piVar3 = FUN_004e25c0(param_1);
      return piVar3;
    }
    if (uVar1 == 0x3ec) {
      piVar3 = FUN_004e2400(param_1);
      return piVar3;
    }
  }
  *(undefined4 *)((int)param_1 + 0x38) = 0;
  return (int *)0x0;
}

