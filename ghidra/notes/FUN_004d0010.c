
int * __fastcall FUN_004d0010(void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int *piVar3;
  int *piVar4;
  
  piVar4 = (int *)0x0;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 3:
    iVar2 = FUN_004d0860((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar2 != 0) & 2) + 7;
    return (int *)0x0;
  case 4:
    piVar4 = FUN_004d0d30((uint)param_1);
    if (piVar4 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return piVar4;
  case 5:
    iVar2 = FUN_004d1270((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar2;
    if (iVar2 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3f4;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 3;
    return (int *)0x0;
  case 7:
    bVar1 = FUN_004d0d10((int)param_1);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x38) = 0x3f4;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 8;
    return (int *)0x0;
  case 8:
    piVar4 = FUN_004d1310((int)param_1);
    break;
  case 9:
    *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) =
         *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) & 0xfffffffb;
    if ((*(int *)(*(int *)((int)param_1 + 0x2c) + 0xc) == 4) &&
       (iVar2 = FUN_004d0b40((int)param_1), iVar2 != 0)) {
      FUN_004199b0(*(int *)((int)param_1 + 0x2c));
      *(undefined4 *)((int)param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0xc;
    return (int *)0x0;
  case 10:
    piVar3 = FUN_004d1710(param_1,(uint *)((int)param_1 + 0x5c));
    piVar4 = (int *)0x0;
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xb;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar3;
    }
    break;
  case 0xb:
    piVar4 = FUN_004d1850((int)param_1);
    break;
  case 0xc:
    FUN_004d1970((int)param_1);
    FUN_004d1ed0((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3f4;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar4;
}

