
int * __fastcall FUN_004cca20(void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int *piVar3;
  int *piVar4;
  
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 2:
    iVar2 = FUN_004cd040((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 0xfffffffc) + 8;
    return (int *)0x0;
  case 3:
    iVar2 = FUN_004cccc0((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 0xfffffffa) + 8;
    return (int *)0x0;
  case 4:
    iVar2 = FUN_004cd230((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar2;
    if (iVar2 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  case 5:
    piVar4 = FUN_004cd310((int)param_1);
    break;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 3;
    return (int *)0x0;
  case 8:
    bVar1 = FUN_004cd2f0((int)param_1);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return (int *)0x0;
  case 9:
    piVar3 = FUN_004ce350(param_1,(uint *)((int)param_1 + 0x5c));
    piVar4 = (int *)0x0;
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 10;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar3;
    }
    break;
  case 10:
    piVar4 = FUN_004ce490((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar4;
}

