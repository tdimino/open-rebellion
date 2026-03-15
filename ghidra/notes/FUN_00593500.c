
int __cdecl FUN_00593500(uint param_1,void *param_2,undefined4 *param_3)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00650980;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = 0;
  iVar2 = FUN_00592600(param_1,param_2,param_3);
  if ((*(int *)(param_1 + 4) != 0) && (*(int *)(param_1 + 0xc) != 0)) {
    if ((iVar2 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
      local_14 = 0x10;
      local_10 = 0x40;
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
        local_10 = 0x27;
        local_4 = 2;
        FUN_00520580(param_2,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *param_3 = 0xffffffff;
      }
    }
    if (((iVar2 != 0) && (*(int *)((int)param_2 + 4) == -1)) &&
       ((*(byte *)(*(int *)(param_1 + 0x38) + 0x50) & 4) == 0)) {
      local_14 = 0x40;
      local_10 = 0x34;
      local_4 = 3;
      FUN_00520580(param_2,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_3 = 0xffffffff;
    }
  }
  ExceptionList = local_c;
  return iVar2;
}

