
uint __cdecl FUN_0056d4f0(uint param_1,void *param_2,undefined4 *param_3)

{
  uint uVar1;
  undefined4 *puVar2;
  uint uVar3;
  uint *puVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064bbc8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar2 = param_3;
  *param_3 = 0;
  uVar3 = FUN_00594320(param_1,param_2,param_3);
  if (*(int *)(param_1 + 4) != 0) {
    if (((*(int *)(param_1 + 0xc) != 0) && (uVar3 != 0)) && (*(int *)((int)param_2 + 4) == -1)) {
      puVar4 = FUN_004025b0(*(void **)(param_1 + 0x38),(uint *)&param_3);
      uVar1 = *puVar4;
      local_4 = 1;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      if (uVar1 != 0x98000481) {
        local_14 = 0x40;
        local_10 = 0x31;
        local_4 = 2;
        FUN_00520580(param_2,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *puVar2 = 0xffffffff;
      }
    }
    if (*(int *)(param_1 + 8) == 0) {
      if (uVar3 != 0) {
        if (*(int *)((int)param_2 + 4) == -1) {
          uVar3 = FUN_0056d910(*(int *)(param_1 + 0x18),param_1,param_2,puVar2);
        }
        if (uVar3 != 0) {
          if (*(int *)((int)param_2 + 4) == -1) {
            uVar3 = FUN_0056d910(*(int *)(param_1 + 0x1c),param_1,param_2,puVar2);
          }
          if ((uVar3 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
            uVar3 = FUN_0056d910(*(int *)(param_1 + 0x20),param_1,param_2,puVar2);
          }
        }
      }
    }
    else if (uVar3 != 0) {
      if (*(int *)((int)param_2 + 4) == -1) {
        uVar3 = FUN_0056d6d0(*(int *)(param_1 + 0x24),param_1,param_2,puVar2);
      }
      if (uVar3 != 0) {
        if (*(int *)((int)param_2 + 4) == -1) {
          uVar3 = FUN_0056d6d0(*(int *)(param_1 + 0x28),param_1,param_2,puVar2);
        }
        if ((uVar3 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
          uVar3 = FUN_0056d6d0(*(int *)(param_1 + 0x2c),param_1,param_2,puVar2);
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

