
int * __fastcall FUN_004d7060(void *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 1:
    iVar1 = FUN_004d8ad0((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    break;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 1;
    return (int *)0x0;
  case 4:
    *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) =
         *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) & 0xfffffffe;
    if ((*(int *)(*(int *)((int)param_1 + 0x2c) + 0xc) == 1) &&
       (iVar1 = FUN_004d8db0((int)param_1), iVar1 != 0)) {
      FUN_004199b0(*(int *)((int)param_1 + 0x2c));
      *(undefined4 *)((int)param_1 + 0x3c) = 6;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0xd;
    return (int *)0x0;
  case 5:
    iVar1 = FUN_004d9520((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar1;
    if (iVar1 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ed;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  case 6:
    piVar3 = FUN_004d8fe0((uint)param_1);
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return piVar3;
  case 8:
    piVar3 = FUN_004d95c0((int)param_1);
    break;
  case 0xb:
    piVar2 = FUN_004d9e70(param_1,(uint *)((int)param_1 + 0x58));
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xc;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 0xc:
    piVar3 = FUN_004d9fb0((int)param_1);
    break;
  case 0xd:
    FUN_004da0d0((int)param_1);
    FUN_004da610((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3ed;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar3;
}

