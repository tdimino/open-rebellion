
int __thiscall FUN_00521600(void *this,int param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006437e8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar2 = FUN_004f9860(this,param_1,param_2);
  if (*(int *)((int)param_2 + 4) == -1) {
    bVar1 = FUN_00520b20((int)this);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      local_14 = 0x40;
      local_10 = 0x40;
      local_4 = 1;
      FUN_00520580(param_2,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if (*(int *)((int)param_2 + 4) == -1) {
      bVar1 = FUN_00520b30((int)this);
      if (CONCAT31(extraout_var_00,bVar1) == 0) {
        local_14 = 0x40;
        local_10 = 0x41;
        local_4 = 2;
        FUN_00520580(param_2,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  ExceptionList = local_c;
  return iVar2;
}

