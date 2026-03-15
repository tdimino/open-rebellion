
int __cdecl FUN_005868c0(uint param_1,void *param_2,undefined4 *param_3)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064f778;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = 0;
  iVar2 = FUN_00592600(param_1,param_2,param_3);
  if ((((*(int *)(param_1 + 4) != 0) && (*(int *)(param_1 + 0xc) != 0)) && (iVar2 != 0)) &&
     (*(int *)((int)param_2 + 4) == -1)) {
    local_14 = 0x90;
    local_10 = 0x98;
    local_4 = 1;
    uVar3 = (**(code **)(**(int **)(param_1 + 0x38) + 4))();
    if ((uVar3 < local_14) || (local_10 <= uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (!bVar1) {
      local_14 = 0x40;
      local_10 = 0x24;
      local_4 = 2;
      FUN_00520580(param_2,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_3 = 0xffffffff;
    }
  }
  if (((*(int *)(param_1 + 0xc) != 0) && (iVar2 != 0)) && (*(int *)((int)param_2 + 4) == -1)) {
    uVar3 = *(uint *)(*(int *)(param_1 + 0x38) + 0x88) >> 2 & 1;
    if ((uVar3 == 0) || (*(int *)(*(int *)(param_1 + 0x14) + 0x88) != 0)) {
      if (uVar3 != 0) {
        ExceptionList = pvStack_c;
        return iVar2;
      }
      if (*(int *)(*(int *)(param_1 + 0x14) + 0x8c) != 0) {
        ExceptionList = pvStack_c;
        return iVar2;
      }
      local_14 = 0x40;
      local_10 = 0x20;
      local_4 = 4;
      FUN_00520580(param_2,&local_14);
    }
    else {
      local_14 = 0x40;
      local_10 = 0x20;
      local_4 = 3;
      FUN_00520580(param_2,&local_14);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    *param_3 = 6;
  }
  ExceptionList = pvStack_c;
  return iVar2;
}

