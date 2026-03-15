
int * __fastcall FUN_004cf2c0(void *param_1)

{
  int iVar1;
  int *piVar2;
  void *local_4;
  
  piVar2 = (int *)0x0;
  local_4 = param_1;
  if (*(int *)((int)param_1 + 0x3c) == 2) {
    iVar1 = FUN_004cf350((int)param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 3;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
  }
  else {
    if (*(int *)((int)param_1 + 0x3c) != 3) {
      *(undefined4 *)((int)param_1 + 0x3c) = 2;
      return (int *)0x0;
    }
    piVar2 = FUN_004cf3b0(param_1,&local_4);
    if (piVar2 != (int *)0x0) {
      if (local_4 == (void *)0x0) {
        *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      }
      *(undefined4 *)((int)param_1 + 0x20) = 1;
    }
    if (local_4 != (void *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      return piVar2;
    }
  }
  return piVar2;
}

