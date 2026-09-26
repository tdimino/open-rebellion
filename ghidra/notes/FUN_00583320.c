
undefined4 __cdecl FUN_00583320(int param_1,uint *param_2,void *param_3)

{
  uint uVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064f0e0;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((*(int *)((int)param_3 + 4) == -1) &&
     ((*param_2 != (*(uint *)(param_1 + 0x48) & *param_2) ||
      (param_2[1] != (*(uint *)(param_1 + 0x4c) & param_2[1]))))) {
    local_14 = 0x40;
    local_10 = 1;
    local_4 = 1;
    FUN_00520580(param_3,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (*(int *)((int)param_3 + 4) != -1) {
    ExceptionList = local_c;
    return 1;
  }
  uVar1 = param_2[2];
  if (uVar1 == 0) {
LAB_0058340c:
    if (param_2[3] == 0) {
      local_14 = 1;
      local_10 = 0x16;
      local_4 = 3;
      FUN_00520580(param_3,&local_14);
      goto LAB_005834a8;
    }
    if (uVar1 != 0) goto LAB_00583441;
  }
  else {
    if (param_2[3] != 0) {
      local_14 = 1;
      local_10 = 0x14;
      local_4 = 2;
      FUN_00520580(param_3,&local_14);
      goto LAB_005834a8;
    }
    if (uVar1 == 0) goto LAB_0058340c;
LAB_00583441:
    if (*(int *)(param_1 + 0x40) == 0) {
      local_14 = 0x40;
      local_10 = 1;
      local_4 = 4;
      FUN_00520580(param_3,&local_14);
      goto LAB_005834a8;
    }
  }
  if (param_2[3] == 0) {
    ExceptionList = local_c;
    return 1;
  }
  if (*(int *)(param_1 + 0x44) != 0) {
    ExceptionList = local_c;
    return 1;
  }
  local_14 = 0x40;
  local_10 = 1;
  local_4 = 5;
  FUN_00520580(param_3,&local_14);
LAB_005834a8:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

