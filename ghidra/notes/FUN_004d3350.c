
int * __fastcall FUN_004d3350(void *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 2:
    iVar1 = FUN_004d5000((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 5;
      return (int *)0x0;
    }
    break;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 2;
    return (int *)0x0;
  case 5:
    *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) =
         *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) & 0xfffffff7;
    if ((*(int *)(*(int *)((int)param_1 + 0x2c) + 0xc) == 8) &&
       (iVar1 = FUN_004d52e0((int)param_1), iVar1 != 0)) {
      FUN_004199b0(*(int *)((int)param_1 + 0x2c));
      *(undefined4 *)((int)param_1 + 0x3c) = 8;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0xd;
    return (int *)0x0;
  case 6:
    iVar1 = FUN_004d5a50((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar1;
    if (iVar1 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3f1;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  case 8:
    piVar3 = FUN_004d5510((uint)param_1);
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 6;
    return piVar3;
  case 9:
    piVar3 = FUN_004d5af0((int)param_1);
    break;
  case 0xb:
    piVar2 = FUN_004d6030(param_1,(uint *)((int)param_1 + 0x58));
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xc;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 0xc:
    piVar3 = FUN_004d6170((int)param_1);
    break;
  case 0xd:
    FUN_004d6290((int)param_1);
    FUN_004d67e0((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3f1;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar3;
}

