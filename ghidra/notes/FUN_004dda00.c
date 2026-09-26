
int * __fastcall FUN_004dda00(void *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 5:
    *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) =
         *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) & 0xffffffdf;
    if ((*(int *)(*(int *)((int)param_1 + 0x2c) + 0xc) == 0x20) &&
       (iVar1 = FUN_004dffd0((int)param_1), iVar1 != 0)) {
      FUN_004199b0(*(int *)((int)param_1 + 0x2c));
      *(undefined4 *)((int)param_1 + 0x3c) = 9;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0xe;
    return (int *)0x0;
  case 6:
    iVar1 = FUN_004dfcf0((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 5;
      return (int *)0x0;
    }
    break;
  case 7:
    iVar1 = FUN_004e06b0((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar1;
    if (iVar1 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ef;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 6;
    return (int *)0x0;
  case 9:
    piVar3 = FUN_004e0170((uint)param_1);
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 7;
    return piVar3;
  case 10:
    piVar3 = FUN_004e0750((int)param_1);
    break;
  case 0xc:
    piVar2 = FUN_004e0c90(param_1,(uint *)((int)param_1 + 0x58));
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xd;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 0xd:
    piVar3 = FUN_004e0dd0((int)param_1);
    break;
  case 0xe:
    FUN_004e15b0((int)param_1);
    FUN_004e1b00((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3ef;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar3;
}

