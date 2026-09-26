
int * __fastcall FUN_004d3220(void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int *piVar3;
  int *piVar4;
  
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 2:
    iVar2 = FUN_004d4370((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 0xfffffffc) + 10;
    return (int *)0x0;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return (int *)0x0;
  case 5:
    iVar2 = FUN_004d3ff0((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 0xfffffff8) + 10;
    return (int *)0x0;
  case 6:
    iVar2 = FUN_004d46e0((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar2;
    if (iVar2 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  case 7:
    piVar4 = FUN_004d4800((int)param_1);
    break;
  case 10:
    bVar1 = FUN_004d47b0((int)param_1);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 7;
    return (int *)0x0;
  case 0xb:
    piVar3 = FUN_004d5dd0(param_1,(uint *)((int)param_1 + 0x54));
    piVar4 = (int *)0x0;
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xc;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar3;
    }
    break;
  case 0xc:
    piVar4 = FUN_004d5f10((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar4;
}

