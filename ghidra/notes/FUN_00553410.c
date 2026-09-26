
undefined4 __cdecl
FUN_00553410(int param_1,uint param_2,uint param_3,int param_4,int param_5,void *param_6)

{
  uint uVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649000;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_6,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((*(int *)((int)param_6 + 4) == -1) && ((*(uint *)(param_1 + 0x24) >> 6 & 3) != param_2)) {
    local_14 = 1;
    local_10 = 0x24;
    local_4 = 1;
    FUN_00520580(param_6,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (*(int *)((int)param_6 + 4) == -1) {
    if ((param_5 != 0) && ((*(uint *)(param_1 + 0x24) >> 6 & 3) != param_3)) {
      local_14 = 1;
      local_10 = 0x28;
      local_4 = 2;
      FUN_00520580(param_6,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if (*(int *)((int)param_6 + 4) == -1) {
      uVar1 = *(uint *)(param_1 + 0x50);
      if (((uVar1 & 4) == 0) && (param_4 != 0)) {
        local_14 = 1;
        local_10 = 0x20;
        local_4 = 3;
        FUN_00520580(param_6,&local_14);
      }
      else if ((uVar1 & 8) == 0) {
        if ((uVar1 & 0x10) == 0) {
          ExceptionList = local_c;
          return 1;
        }
        local_14 = 1;
        local_10 = 0x21;
        local_4 = 5;
        FUN_00520580(param_6,&local_14);
      }
      else {
        local_14 = 1;
        local_10 = 0x22;
        local_4 = 4;
        FUN_00520580(param_6,&local_14);
      }
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return 1;
}

