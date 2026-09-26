
int * __fastcall FUN_004e25c0(void *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  uint *puVar4;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 1:
    iVar1 = FUN_004e2760((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (iVar1 != 0) + 2;
    return (int *)0x0;
  case 2:
    piVar3 = FUN_004e2940((int)param_1);
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 3;
    return piVar3;
  case 3:
    if (*(int *)((int)param_1 + 0x38) == 0x3eb) {
      iVar1 = FUN_004e2f10((int)param_1);
    }
    else {
      iVar1 = FUN_004e2c70((int)param_1);
    }
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    break;
  case 4:
    if (*(int *)((int)param_1 + 0x38) == 0x3eb) {
      puVar4 = (uint *)((int)param_1 + 0x44);
    }
    else {
      puVar4 = (uint *)((int)param_1 + 0x40);
    }
    piVar3 = FUN_004e31b0(param_1,puVar4);
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 8;
    return piVar3;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 1;
    return (int *)0x0;
  case 8:
    if (*(int *)((int)param_1 + 0x38) == 0x3eb) {
      puVar4 = (uint *)((int)param_1 + 0x44);
    }
    else {
      puVar4 = (uint *)((int)param_1 + 0x40);
    }
    piVar2 = FUN_004e4300(param_1,puVar4);
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 9;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 9:
    if (*(int *)((int)param_1 + 0x38) == 0x3eb) {
      *(uint *)((int)param_1 + 0x50) = *(uint *)((int)param_1 + 0x50) & 0xffffff | 0x2d000000;
      *(undefined4 *)((int)param_1 + 0x50) = 0x2d000002;
    }
    else {
      *(uint *)((int)param_1 + 0x50) = *(uint *)((int)param_1 + 0x50) & 0xffffff | 0x2c000000;
      *(undefined4 *)((int)param_1 + 0x50) = 0x2c000001;
    }
    *(undefined4 *)((int)param_1 + 0x54) = 1;
    piVar3 = FUN_004e4440((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar3;
}

