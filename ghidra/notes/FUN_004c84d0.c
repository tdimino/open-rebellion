
int __fastcall FUN_004c84d0(void *param_1)

{
  int iVar1;
  void *local_4;
  
  iVar1 = 0;
  local_4 = param_1;
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 1:
    FUN_004c85b0((int)param_1);
    *(undefined4 *)((int)param_1 + 0x3c) = 2;
    return 0;
  case 2:
    iVar1 = FUN_004c8630((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = 4 - (uint)(iVar1 != 0);
    return 0;
  case 3:
    FUN_004c86b0((int)param_1);
    *(undefined4 *)((int)param_1 + 0x3c) = 4;
    return 0;
  case 4:
    iVar1 = FUN_004c8740((int)param_1);
    if (iVar1 == 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      return 0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return 0;
  case 5:
    iVar1 = FUN_004c87c0(param_1,&local_4);
    if (iVar1 != 0) {
      if (local_4 == (void *)0x0) {
        *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      }
      *(undefined4 *)((int)param_1 + 0x20) = 1;
    }
    if (local_4 != (void *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      return iVar1;
    }
    break;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 1;
  }
  return iVar1;
}

