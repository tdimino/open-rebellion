
int __cdecl FUN_00523450(int param_1,void *param_2,undefined4 *param_3)

{
  uint uVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00643b10;
  local_c = ExceptionList;
  iVar4 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = 0;
  if ((*(int *)(param_1 + 0xc) != 0) &&
     ((*(int *)(param_1 + 0x38) == 0 || (*(int *)(param_1 + 0x34) == 0)))) {
    iVar4 = 0;
  }
  if (*(int *)(param_1 + 4) != 0) {
    if (iVar4 != 0) {
      iVar4 = FUN_00583320(*(int *)(param_1 + 0x14),*(uint **)(param_1 + 0x30),param_2);
    }
    if (*(int *)(param_1 + 0xc) == 0) {
      ExceptionList = local_c;
      return iVar4;
    }
    if (((iVar4 != 0) && (*(int *)((int)param_2 + 4) == -1)) &&
       ((*(byte *)(*(int *)(param_1 + 0x34) + 0x50) & 0x10) != 0)) {
      local_14 = 0x40;
      local_10 = 0x13;
      local_4 = 1;
      FUN_00520580(param_2,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_3 = 0xffffffff;
    }
  }
  if (*(int *)(param_1 + 0xc) != 0) {
    if (((iVar4 != 0) && (*(int *)((int)param_2 + 4) == -1)) &&
       ((*(int *)(*(int *)(param_1 + 0x14) + 0x6c) != 0 &&
        ((*(byte *)(*(int *)(param_1 + 0x34) + 0x50) & 8) != 0)))) {
      local_14 = 0x40;
      local_10 = 0x12;
      local_4 = 2;
      FUN_00520580(param_2,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_3 = 7;
    }
    if (((*(int *)(param_1 + 0xc) != 0) && (iVar4 != 0)) && (*(int *)((int)param_2 + 4) == -1)) {
      uVar1 = *(uint *)(param_1 + 0x10);
      uVar3 = *(uint *)(*(int *)(param_1 + 0x38) + 0x24) >> 6 & 3;
      if (uVar1 == uVar3) {
        iVar2 = *(int *)(*(int *)(param_1 + 0x14) + 0x7c);
      }
      else {
        if (uVar1 == 3) {
          uVar1 = 3;
        }
        else {
          uVar1 = 2 - (uVar1 != 1);
        }
        if (uVar1 == uVar3) {
          iVar2 = *(int *)(*(int *)(param_1 + 0x14) + 0x84);
        }
        else {
          iVar2 = *(int *)(*(int *)(param_1 + 0x14) + 0x80);
        }
      }
      if (iVar2 == 0) {
        local_14 = 0x40;
        local_10 = 0x23;
        local_4 = 3;
        FUN_00520580(param_2,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *param_3 = 8;
      }
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

