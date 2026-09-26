
int __thiscall FUN_004ed9d0(void *this,int param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063ee38;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar2 = FUN_004f9860(this,param_1,param_2);
  if (iVar2 != 0) {
    if ((*(int *)((int)param_2 + 4) == -1) && ((*(byte *)((int)this + 0x78) & 0x80) != 0)) {
      local_14 = 0x30;
      local_10 = 1;
      local_4 = 1;
      FUN_00520580(param_2,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if (iVar2 != 0) {
      if ((*(int *)((int)param_2 + 4) == -1) && (*(short *)((int)this + 0x94) != 0)) {
        local_14 = 0x30;
        local_10 = 3;
        local_4 = 2;
        FUN_00520580(param_2,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      if (iVar2 != 0) {
        if ((*(int *)((int)param_2 + 4) == -1) && ((*(byte *)((int)this + 0xac) & 1) != 0)) {
          local_14 = 0x30;
          local_10 = 2;
          local_4 = 3;
          FUN_00520580(param_2,&local_14);
          local_4 = 0xffffffff;
          FUN_00619730();
        }
        if ((iVar2 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
          bVar1 = FUN_0053eb50((int)*(short *)((int)this + 0x8c));
          if (CONCAT31(extraout_var,bVar1) == 0) {
            local_14 = 0x30;
            local_10 = 6;
            local_4 = 4;
            FUN_00520580(param_2,&local_14);
            local_4 = 0xffffffff;
            FUN_00619730();
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return iVar2;
}

