
int * __fastcall FUN_004dabe0(uint param_1)

{
  int iVar1;
  uint uVar2;
  int *piVar3;
  
  iVar1 = FUN_005f4960((undefined4 *)(param_1 + 0x1c));
  if (iVar1 != 1) {
    *(undefined4 *)(param_1 + 0x38) = 0;
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return (int *)0x0;
  }
  if (*(int *)(param_1 + 0x38) != 0) {
    if (*(int *)(param_1 + 0x38) == 0x3ec) {
      piVar3 = FUN_004db2f0(param_1);
      return piVar3;
    }
    *(undefined4 *)(param_1 + 0x38) = 0;
    return (int *)0x0;
  }
  iVar1 = FUN_004daca0(param_1);
  if (iVar1 != 0) {
    *(undefined4 *)(param_1 + 0x38) = 0x3ec;
    return (int *)0x0;
  }
  iVar1 = FUN_004daf10(param_1);
  if (iVar1 != 0) {
    *(undefined4 *)(param_1 + 0x38) = 0x3ec;
    *(undefined4 *)(param_1 + 0x3c) = 0;
    return (int *)0x0;
  }
  if (*(int *)(*(int *)(param_1 + 0x2c) + 0xc) == 0x80) {
    uVar2 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x2c) + 4));
    if ((uVar2 & 0x40) == 0) goto LAB_004dac4e;
  }
  FUN_004dcc40(param_1);
LAB_004dac4e:
  FUN_004dd120(param_1);
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x3c) = 0;
  *(undefined4 *)(param_1 + 0x20) = 1;
  return (int *)0x0;
}

