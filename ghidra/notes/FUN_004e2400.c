
int * __fastcall FUN_004e2400(void *param_1)

{
  int iVar1;
  int *piVar2;
  
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 1:
    iVar1 = FUN_004e2760((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (iVar1 != 0) + 2;
    return (int *)0x0;
  case 2:
    piVar2 = FUN_004e2940((int)param_1);
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 3;
    return piVar2;
  case 3:
    iVar1 = FUN_004e3530((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 7;
      return (int *)0x0;
    }
    break;
  case 4:
    piVar2 = FUN_004e3a40((uint)param_1);
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return piVar2;
  case 5:
    iVar1 = FUN_004e3f80((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar1;
    if (iVar1 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x38) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  case 6:
    piVar2 = FUN_004e4020((int)param_1);
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x38) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return piVar2;
  case 7:
    *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) =
         *(uint *)(*(int *)((int)param_1 + 0x2c) + 8) & 0xffffffbf;
    if ((*(int *)(*(int *)((int)param_1 + 0x2c) + 0xc) == 0x40) &&
       (iVar1 = FUN_004e3810((int)param_1), iVar1 != 0)) {
      FUN_004199b0(*(int *)((int)param_1 + 0x2c));
      *(undefined4 *)((int)param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 10;
    return (int *)0x0;
  case 8:
    piVar2 = FUN_004e4300(param_1,(uint *)((int)param_1 + 0x48));
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 9;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 9:
    piVar2 = FUN_004e4440((int)param_1);
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x38) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return piVar2;
  case 10:
    FUN_004e4540((int)param_1);
    FUN_004e4a20((int)param_1);
    break;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 1;
    return (int *)0x0;
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return (int *)0x0;
}

