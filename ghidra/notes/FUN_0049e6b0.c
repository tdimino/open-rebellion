
int * __fastcall FUN_0049e6b0(void *param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = *(int *)((int)param_1 + 0x38);
  if (iVar1 == 1) {
    if ((*(uint *)((int)param_1 + 0x3c) & 0xff000000) == 0) {
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      return (int *)0x0;
    }
    iVar1 = FUN_0049e7a0(param_1);
    if (iVar1 != 0) {
      *(undefined4 *)((int)param_1 + 0x38) = 2;
      return (int *)0x0;
    }
  }
  else if (iVar1 == 2) {
    piVar2 = FUN_0049e960((int)param_1);
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x38) = 3;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x30) = *(int *)((int)param_1 + 0x30) + 1;
      return piVar2;
    }
  }
  else {
    if (iVar1 != 3) {
      *(undefined4 *)((int)param_1 + 0x38) = 1;
      return (int *)0x0;
    }
    iVar1 = FUN_0049eb50((int)param_1);
    piVar2 = FUN_0049ea40(param_1,iVar1);
    if (piVar2 != (int *)0x0) {
      FUN_0049ec40(param_1,piVar2);
      DAT_006b28b0 = DAT_006b28b0 | 0x10000000;
      FUN_0043a290(*(void **)((int)param_1 + 0x54),piVar2);
      *(undefined4 *)((int)param_1 + 0x38) = 0;
      *(undefined4 *)((int)param_1 + 0x34) = 0;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(undefined4 *)((int)param_1 + 0x1c) = 2;
      return (int *)0x0;
    }
  }
  *(undefined4 *)((int)param_1 + 0x38) = 0;
  *(undefined4 *)((int)param_1 + 0x34) = 0;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  *(undefined4 *)((int)param_1 + 0x1c) = 2;
  return (int *)0x0;
}

