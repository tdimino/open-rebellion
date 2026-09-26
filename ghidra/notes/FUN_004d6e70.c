
int * __fastcall FUN_004d6e70(void *param_1)

{
  undefined4 uVar1;
  int iVar2;
  int *piVar3;
  int *piVar4;
  uint uVar5;
  
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 1:
    iVar2 = FUN_004d76f0((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 0xfffffffd) + 9;
    return (int *)0x0;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 4;
    return (int *)0x0;
  case 4:
    iVar2 = FUN_004d7ac0((int)param_1);
    if (iVar2 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 1;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
    return (int *)0x0;
  case 5:
    uVar1 = FUN_004d8460((int)param_1);
    *(undefined4 *)((int)param_1 + 0x3c) = uVar1;
    return (int *)0x0;
  case 6:
    piVar4 = FUN_004d7cf0((int)param_1);
    if (piVar4 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return piVar4;
  case 7:
    piVar4 = FUN_004d8520((int)param_1);
    if (piVar4 == (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 9;
      return (int *)0x0;
    }
    break;
  case 9:
    if (*(int *)(*(int *)((int)param_1 + 0x2c) + 0x1d4) <
        *(int *)(*(int *)((int)param_1 + 0x2c) + 0x1d0)) {
      uVar5 = *(uint *)((int)param_1 + 0x5c) & 0xffffff | 0x2d000000;
      *(uint *)((int)param_1 + 0x5c) = uVar5;
      *(uint *)((int)param_1 + 0x5c) = CONCAT31((uint3)(uVar5 >> 8) & 0xff0000,2);
      iVar2 = FUN_004d99d0((int)param_1);
    }
    else {
      *(uint *)((int)param_1 + 0x5c) = *(uint *)((int)param_1 + 0x5c) & 0xffffff | 0x2c000000;
      *(undefined4 *)((int)param_1 + 0x5c) = 0x2c000001;
      iVar2 = FUN_004d98a0((int)param_1);
    }
    if (iVar2 == 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
    }
    else {
      *(undefined4 *)((int)param_1 + 0x60) = 1;
      *(undefined4 *)((int)param_1 + 0x3c) = 10;
    }
  case 10:
    piVar4 = FUN_004d9b00((int)param_1);
    if (piVar4 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0xb;
    return piVar4;
  case 0xb:
    piVar3 = FUN_004d9e70(param_1,(uint *)((int)param_1 + 0x54));
    piVar4 = (int *)0x0;
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xc;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar3;
    }
    break;
  case 0xc:
    piVar4 = FUN_004d9fb0((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar4;
}

