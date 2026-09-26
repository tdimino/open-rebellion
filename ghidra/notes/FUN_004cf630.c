
int * __fastcall FUN_004cf630(void *param_1)

{
  int iVar1;
  int *piVar2;
  void *local_4;
  
  local_4 = param_1;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 1:
    FUN_004cf700((int)param_1);
    *(undefined4 *)((int)param_1 + 0x3c) = 2;
    return (int *)0x0;
  case 2:
    iVar1 = FUN_004cf760((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 3;
      return (int *)0x0;
    }
    break;
  case 3:
    iVar1 = FUN_004cf7c0((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 4;
      return (int *)0x0;
    }
    break;
  case 4:
    piVar2 = FUN_004cfac0(param_1,&local_4);
    if (piVar2 != (int *)0x0) {
      if (local_4 == (void *)0x0) {
        *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      }
      *(undefined4 *)((int)param_1 + 0x20) = 1;
    }
    if (local_4 == (void *)0x0) {
      return piVar2;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return piVar2;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 1;
    return (int *)0x0;
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return (int *)0x0;
}

