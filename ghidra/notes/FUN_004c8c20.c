
int * __fastcall FUN_004c8c20(int param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int *piVar3;
  
  switch(*(undefined4 *)(param_1 + 0x3c)) {
  case 5:
    FUN_004ca5e0(param_1);
    *(undefined4 *)(param_1 + 0x3c) = 9;
    return (int *)0x0;
  default:
    *(undefined4 *)(param_1 + 0x3c) = 8;
    break;
  case 8:
    iVar2 = FUN_004ca300(param_1);
    *(uint *)(param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 0xfffffff8) + 0xd;
    return (int *)0x0;
  case 9:
    iVar2 = FUN_004ca910(param_1);
    *(int *)(param_1 + 0x3c) = iVar2;
    if (iVar2 == 0) {
      *(undefined4 *)(param_1 + 0x38) = 0x3ec;
      *(undefined4 *)(param_1 + 0x20) = 1;
      return (int *)0x0;
    }
    break;
  case 10:
    piVar3 = FUN_004cab20(param_1);
    goto LAB_004c8cc2;
  case 0xd:
    bVar1 = FUN_004caad0(param_1);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      *(undefined4 *)(param_1 + 0x3c) = 10;
      return (int *)0x0;
    }
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0x3ec;
    return (int *)0x0;
  case 0xf:
    piVar3 = FUN_004cba90(param_1);
LAB_004c8cc2:
    *(undefined4 *)(param_1 + 0x3c) = 0;
    *(undefined4 *)(param_1 + 0x38) = 0x3ec;
    *(undefined4 *)(param_1 + 0x20) = 1;
    return piVar3;
  }
  return (int *)0x0;
}

