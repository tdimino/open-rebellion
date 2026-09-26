
int * __fastcall FUN_004c8d20(void *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 7:
    iVar1 = FUN_004cacc0((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 8;
      return (int *)0x0;
    }
    break;
  case 8:
    *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) =
         *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) & 0xfffffffd;
    if ((*(int *)(*(int *)((int)param_1 + 0x2c) + 0xc) == 2) &&
       (iVar1 = FUN_004cafa0((int)param_1), iVar1 != 0)) {
      FUN_004199b0(*(int *)((int)param_1 + 0x2c));
      *(undefined4 *)((int)param_1 + 0x3c) = 0xb;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0x10;
    return (int *)0x0;
  case 9:
    iVar1 = FUN_004cb710((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar1;
    if (iVar1 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3f8;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 7;
    return (int *)0x0;
  case 0xb:
    piVar3 = FUN_004cb1d0((uint)param_1);
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 9;
    return piVar3;
  case 0xc:
    piVar3 = FUN_004cb7b0((int)param_1);
    break;
  case 0xe:
    piVar2 = FUN_004cbbb0(param_1,(uint *)((int)param_1 + 0x58));
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xf;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 0xf:
    piVar3 = FUN_004cbcf0((int)param_1);
    break;
  case 0x10:
    FUN_004cbe10((int)param_1);
    FUN_004cc360((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3f8;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar3;
}

