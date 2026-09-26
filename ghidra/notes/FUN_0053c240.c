
undefined4 __thiscall FUN_0053c240(void *this,undefined4 *param_1)

{
  uint uVar1;
  int *piVar2;
  int iVar3;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006463f8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  uVar1 = FUN_0053c4b0(this,param_1);
  if (uVar1 != 0) {
    piVar2 = (int *)thunk_FUN_005f5060(*(int *)((int)this + 0x44));
    do {
      if (param_1[1] != -1) {
        ExceptionList = local_c;
        return 1;
      }
      if (piVar2 == (int *)0x0) {
        ExceptionList = local_c;
        return 1;
      }
      iVar3 = (**(code **)(*piVar2 + 0x20))(param_1);
      piVar2 = (int *)piVar2[4];
    } while (iVar3 != 0);
  }
  ExceptionList = local_c;
  return 1;
}

