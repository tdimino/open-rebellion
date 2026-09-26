
void __fastcall FUN_0040d980(undefined4 *param_1)

{
  int *piVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062a2d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00658788;
  local_4 = 0;
  param_1[0x45] = 0;
  param_1[0x46] = 1;
  KillTimer((HWND)param_1[6],0x191);
  KillTimer((HWND)param_1[6],400);
  if (param_1[8] != 0) {
    FUN_00597400(*(undefined4 *)(param_1[8] + 0x18));
  }
  if ((undefined4 *)param_1[0x5c] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5c])(1);
  }
  if ((undefined4 *)param_1[0x5d] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5d])(1);
  }
  if ((undefined4 *)param_1[0x5e] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5e])(1);
  }
  if ((undefined4 *)param_1[0x5f] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5f])(1);
  }
  if ((undefined4 *)param_1[0x60] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x60])(1);
  }
  if (param_1[0x61] != 0) {
    for (piVar1 = (int *)(**(code **)(**(int **)(param_1[0x61] + 0xa0) + 8))(); piVar1 != (int *)0x0
        ; piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
      if ((undefined4 *)piVar1[0x15] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)piVar1[0x15])(1);
      }
    }
    FUN_00609920((void *)param_1[0x61]);
    if ((undefined4 *)param_1[0x61] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x61])(1);
    }
  }
  if (param_1[0x62] != 0) {
    for (piVar1 = (int *)(**(code **)(**(int **)(param_1[0x62] + 0xa0) + 8))(); piVar1 != (int *)0x0
        ; piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
      if ((undefined4 *)piVar1[0x15] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)piVar1[0x15])(1);
      }
    }
    FUN_00609920((void *)param_1[0x62]);
    if ((undefined4 *)param_1[0x62] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x62])(1);
    }
  }
  if ((undefined4 *)param_1[99] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[99])(1);
  }
  if ((undefined4 *)param_1[100] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[100])(1);
  }
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

