
int * __fastcall FUN_004ccb50(void *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 2:
    iVar1 = FUN_004cd580((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 3;
      return (int *)0x0;
    }
    break;
  case 3:
    *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) =
         *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) & 0xffffffef;
    if ((*(int *)(*(int *)((int)param_1 + 0x2c) + 0xc) == 0x10) &&
       (iVar1 = FUN_004cd860((int)param_1), iVar1 != 0)) {
      FUN_004199b0(*(int *)((int)param_1 + 0x2c));
      *(undefined4 *)((int)param_1 + 0x3c) = 6;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0xb;
    return (int *)0x0;
  case 4:
    iVar1 = FUN_004cdfd0((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar1;
    if (iVar1 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3f2;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 2;
    return (int *)0x0;
  case 6:
    piVar3 = FUN_004cda90((uint)param_1);
    if (piVar3 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 4;
    return piVar3;
  case 7:
    piVar3 = FUN_004ce070((int)param_1);
    break;
  case 9:
    piVar2 = FUN_004ce5b0(param_1,(uint *)((int)param_1 + 0x60));
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 10;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 10:
    piVar3 = FUN_004ce6f0((int)param_1);
    break;
  case 0xb:
    FUN_004ce810((int)param_1);
    FUN_004ced60((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3f2;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar3;
}

