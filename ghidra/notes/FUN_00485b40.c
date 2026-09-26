
undefined4 __fastcall FUN_00485b40(void *param_1)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  undefined4 local_18;
  undefined4 auStack_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00634858;
  local_c = ExceptionList;
  iVar2 = *(int *)((int)param_1 + 0x50);
  local_18 = 0;
  if (iVar2 == 3) {
    bVar1 = false;
    if (*(int *)((int)param_1 + 0x420) < *(int *)((int)param_1 + 0x428)) {
      if (*(int *)((int)param_1 + 0x80) == 0) {
        ExceptionList = &local_c;
        iVar2 = FUN_004be980((int)param_1 + 100);
        *(int *)((int)param_1 + 0x80) = iVar2;
        if (iVar2 != 0) {
          ExceptionList = local_c;
          return local_18;
        }
      }
      else {
        ExceptionList = &local_c;
        *(undefined4 *)(*(int *)((int)param_1 + 0x80) + 0x34) = 0;
        piVar3 = (int *)(**(code **)(**(int **)((int)param_1 + 0x80) + 0x14))();
        if (piVar3 != (int *)0x0) {
          FUN_00520570(auStack_14);
          uStack_4 = 0;
          bVar1 = true;
          iVar2 = (**(code **)(*piVar3 + 0x1c))(auStack_14);
          if (iVar2 == 0) {
            (**(code **)(**(int **)((int)param_1 + 0x80) + 0x18))(&local_18,piVar3);
          }
          else {
            FUN_0048aa90(param_1,piVar3);
            iVar2 = (**(code **)(*piVar3 + 0xc))();
            if (iVar2 != 0x203) {
              *(int *)((int)param_1 + 0x420) = *(int *)((int)param_1 + 0x420) + 1;
            }
          }
          uStack_4 = 0xffffffff;
          FUN_00619730();
        }
        if (*(int *)(*(int *)((int)param_1 + 0x80) + 0x20) == 0) {
          ExceptionList = local_c;
          return local_18;
        }
        *(undefined4 *)(*(int *)((int)param_1 + 0x80) + 0x1c) = 0;
        *(undefined4 *)((int)param_1 + 0x80) = 0;
        if ((!bVar1) && ((*(uint *)((int)param_1 + 0x88) & 0x80000000) == 0)) {
          ExceptionList = local_c;
          return local_18;
        }
      }
    }
    *(undefined4 *)((int)param_1 + 0x50) = 4;
    ExceptionList = local_c;
    return 1;
  }
  if (iVar2 == 4) {
    ExceptionList = &local_c;
    iVar2 = FUN_00418500((void *)((int)param_1 + 0x84));
    if (iVar2 == 0) {
      ExceptionList = local_c;
      return local_18;
    }
    *(undefined4 *)((int)param_1 + 0x50) = 5;
    ExceptionList = local_c;
    return local_18;
  }
  if (iVar2 != 5) {
    *(undefined4 *)((int)param_1 + 0x50) = 4;
    return 0;
  }
  if (*(int *)((int)param_1 + 0x418) == 0) {
    ExceptionList = &local_c;
    if (*(int *)((int)param_1 + 0x80) == 0) {
      ExceptionList = &local_c;
      iVar2 = thunk_FUN_005f5060((int)param_1 + 100);
      *(int *)((int)param_1 + 0x80) = iVar2;
    }
    (**(code **)(**(int **)((int)param_1 + 0x80) + 0x10))();
    *(undefined4 *)((int)param_1 + 0x80) = *(undefined4 *)(*(int *)((int)param_1 + 0x80) + 0x10);
  }
  if (*(int *)((int)param_1 + 0x80) != 0) {
    ExceptionList = local_c;
    return local_18;
  }
  *(undefined4 *)((int)param_1 + 0x50) = 3;
  ExceptionList = local_c;
  return local_18;
}

