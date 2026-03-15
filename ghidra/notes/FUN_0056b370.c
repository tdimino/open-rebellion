
uint __cdecl FUN_0056b370(uint param_1,void *param_2,undefined4 *param_3)

{
  uint uVar1;
  int iVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064b8a0;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = 0;
  uVar1 = FUN_005868c0(param_1,param_2,param_3);
  if (*(int *)(param_1 + 4) != 0) {
    if (*(int *)(param_1 + 8) == 0) {
      if (uVar1 != 0) {
        if (*(int *)((int)param_2 + 4) == -1) {
          uVar1 = FUN_0056b680(*(int *)(param_1 + 0x18),param_1,param_2,param_3);
        }
        if (uVar1 != 0) {
          if (*(int *)((int)param_2 + 4) == -1) {
            uVar1 = FUN_0056b680(*(int *)(param_1 + 0x1c),param_1,param_2,param_3);
          }
          if ((uVar1 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
            uVar1 = FUN_0056b680(*(int *)(param_1 + 0x20),param_1,param_2,param_3);
          }
        }
      }
    }
    else if (uVar1 != 0) {
      if (*(int *)((int)param_2 + 4) == -1) {
        uVar1 = FUN_0056b550(*(int *)(param_1 + 0x24),param_1,param_2,param_3);
      }
      if (uVar1 != 0) {
        if (*(int *)((int)param_2 + 4) == -1) {
          uVar1 = FUN_0056b550(*(int *)(param_1 + 0x28),param_1,param_2,param_3);
        }
        if ((uVar1 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
          uVar1 = FUN_0056b550(*(int *)(param_1 + 0x2c),param_1,param_2,param_3);
        }
      }
    }
  }
  if (((*(int *)(param_1 + 0xc) != 0) && (uVar1 != 0)) && (*(int *)((int)param_2 + 4) == -1)) {
    if (*(int *)(param_1 + 0x10) == 1) {
      iVar2 = 1;
    }
    else if (*(int *)(param_1 + 0x10) == 2) {
      iVar2 = 2;
    }
    else {
      iVar2 = 0;
    }
    uVar1 = (uint)(iVar2 != 0);
    if (uVar1 != 0) {
      iVar2 = FUN_00506f30(iVar2);
      uVar1 = (uint)(iVar2 != 0);
      if ((uVar1 != 0) && (*(int *)(iVar2 + 0xb8) != 0)) {
        local_14 = 0x40;
        local_10 = 0x35;
        local_4 = 1;
        FUN_00520580(param_2,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *param_3 = 0x10;
      }
    }
  }
  ExceptionList = local_c;
  return uVar1;
}

