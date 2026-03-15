
int * __fastcall FUN_004db2f0(uint param_1)

{
  undefined1 uVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  uint uVar3;
  int *piVar4;
  
  piVar4 = (int *)0x0;
  switch(*(undefined4 *)(param_1 + 0x3c)) {
  case 1:
    *(uint *)(*(int *)(param_1 + 0x2c) + 8) = *(uint *)(*(int *)(param_1 + 0x2c) + 8) & 0xffffff7f;
    if (*(int *)(*(int *)(param_1 + 0x2c) + 0xc) != 0x80) {
      *(undefined4 *)(param_1 + 0x3c) = 10;
      return (int *)0x0;
    }
    FUN_004199b0(*(int *)(param_1 + 0x2c));
    *(undefined4 *)(param_1 + 0x3c) = 2;
    return (int *)0x0;
  case 2:
    iVar2 = FUN_004db510(param_1);
    if (iVar2 == 0) {
      *(undefined4 *)(param_1 + 0x38) = 0;
      *(undefined4 *)(param_1 + 0x3c) = 0;
      *(undefined4 *)(param_1 + 0x20) = 1;
      return (int *)0x0;
    }
    *(undefined4 *)(param_1 + 0x3c) = 3;
    return (int *)0x0;
  case 3:
    piVar4 = FUN_004db820(param_1);
    if (piVar4 != (int *)0x0) {
      *(undefined4 *)(param_1 + 0x20) = 1;
      *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) + 1;
    }
    *(undefined4 *)(param_1 + 0x3c) = 4;
    return piVar4;
  case 4:
    iVar2 = FUN_004dbd60(param_1);
    *(int *)(param_1 + 0x3c) = iVar2;
    if (iVar2 == 0) {
      *(undefined4 *)(param_1 + 0x38) = 0;
      *(undefined4 *)(param_1 + 0x20) = 1;
      return (int *)0x0;
    }
    break;
  case 5:
    piVar4 = FUN_004dbe70(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return piVar4;
  case 6:
    if (*(int *)(*(int *)(param_1 + 0x2c) + 0x1d4) < *(int *)(*(int *)(param_1 + 0x2c) + 0x1d0)) {
      uVar3 = *(uint *)(param_1 + 0x48) & 0xffffff | 0x2d000000;
      *(uint *)(param_1 + 0x48) = uVar3;
      *(uint *)(param_1 + 0x48) = CONCAT31((uint3)(uVar3 >> 8) & 0xff0000,2);
      uVar1 = FUN_004dc3f0(param_1);
      iVar2 = CONCAT31(extraout_var,uVar1);
    }
    else {
      *(uint *)(param_1 + 0x48) = *(uint *)(param_1 + 0x48) & 0xffffff | 0x2c000000;
      *(undefined4 *)(param_1 + 0x48) = 0x2c000001;
      uVar1 = FUN_004dc150(param_1);
      iVar2 = CONCAT31(extraout_var_00,uVar1);
    }
    if (iVar2 == 0) {
      *(undefined4 *)(param_1 + 0x3c) = 0;
      *(undefined4 *)(param_1 + 0x38) = 0;
    }
    else {
      *(undefined4 *)(param_1 + 0x4c) = 1;
      *(undefined4 *)(param_1 + 0x3c) = 7;
    }
  case 7:
    piVar4 = FUN_004dc650(param_1);
    if (piVar4 != (int *)0x0) {
      *(undefined4 *)(param_1 + 0x20) = 1;
      *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) + 1;
    }
    *(undefined4 *)(param_1 + 0x3c) = 8;
    return piVar4;
  case 8:
    piVar4 = FUN_004dc9f0(param_1);
    if (piVar4 != (int *)0x0) {
      *(undefined4 *)(param_1 + 0x3c) = 9;
      *(undefined4 *)(param_1 + 0x20) = 1;
      *(int *)(param_1 + 0x34) = *(int *)(param_1 + 0x34) + 1;
      return piVar4;
    }
    goto LAB_004db4bf;
  case 9:
    piVar4 = FUN_004dcb30(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return piVar4;
  case 10:
    FUN_004dcc40(param_1);
LAB_004db4bf:
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return piVar4;
  default:
    *(undefined4 *)(param_1 + 0x3c) = 1;
  }
  return (int *)0x0;
}

