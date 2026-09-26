
undefined4 __fastcall FUN_00439a10(int param_1)

{
  int iVar1;
  int *piVar2;
  undefined4 uVar3;
  undefined4 auStack_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062d8a8;
  local_c = ExceptionList;
  uVar3 = 0;
  iVar1 = *(int *)(param_1 + 0xc);
  if (iVar1 == 4) {
    iVar1 = *(int *)(param_1 + 0x34);
    if (iVar1 == 0) {
      ExceptionList = &local_c;
      iVar1 = FUN_0049df70(param_1 + 0x18);
      *(int *)(param_1 + 0x34) = iVar1;
      if (iVar1 != 0) {
        ExceptionList = local_c;
        return 0;
      }
    }
    else if ((*(int *)(param_1 + 0x188) < *(int *)(param_1 + 0x18c)) ||
            (*(int *)(iVar1 + 0x18) == 0x17)) {
      ExceptionList = &local_c;
      *(undefined4 *)(iVar1 + 0x30) = 0;
      piVar2 = (int *)(**(code **)(**(int **)(param_1 + 0x34) + 0x14))();
      if (piVar2 != (int *)0x0) {
        FUN_00520570(auStack_14);
        uStack_4 = 0;
        iVar1 = (**(code **)(*piVar2 + 0x1c))(auStack_14);
        if (iVar1 == 0) {
          (**(code **)(**(int **)(param_1 + 0x34) + 0x18))(auStack_14,piVar2);
        }
        else {
          FUN_004878f0(*(void **)(param_1 + 0x144),piVar2);
          if (*(int *)(*(int *)(param_1 + 0x34) + 0x18) != 0x17) {
            *(int *)(param_1 + 0x188) = *(int *)(param_1 + 0x188) + 1;
          }
        }
        uStack_4 = 0xffffffff;
        FUN_00619730();
      }
      if (*(int *)(*(int *)(param_1 + 0x34) + 0x20) == 0) {
        ExceptionList = local_c;
        return 0;
      }
      iVar1 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x34) + 0x1c));
      if (iVar1 != 2) {
        *(undefined4 *)(*(int *)(param_1 + 0x34) + 0x1c) = 0;
      }
      *(undefined4 *)(param_1 + 0x34) = 0;
    }
    *(undefined4 *)(param_1 + 0xc) = 5;
    uVar3 = 1;
  }
  else if (iVar1 == 5) {
    ExceptionList = &local_c;
    iVar1 = FUN_0049cc80((int *)(param_1 + 0x3c));
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0xc) = 6;
      ExceptionList = local_c;
      return 0;
    }
  }
  else {
    if (iVar1 != 6) {
      *(undefined4 *)(param_1 + 0xc) = 5;
      return 0;
    }
    ExceptionList = &local_c;
    if (*(int *)(param_1 + 0x34) == 0) {
      ExceptionList = &local_c;
      iVar1 = thunk_FUN_005f5060(param_1 + 0x18);
      *(int *)(param_1 + 0x34) = iVar1;
    }
    (**(code **)(**(int **)(param_1 + 0x34) + 0x10))();
    iVar1 = *(int *)(*(int *)(param_1 + 0x34) + 0x10);
    *(int *)(param_1 + 0x34) = iVar1;
    if (iVar1 == 0) {
      *(undefined4 *)(param_1 + 0xc) = 4;
      ExceptionList = local_c;
      return 0;
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

