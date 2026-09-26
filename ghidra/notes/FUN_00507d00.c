
undefined4 __thiscall FUN_00507d00(void *this,undefined4 param_1,void *param_2)

{
  int iVar1;
  int iVar2;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006416f8;
  local_c = ExceptionList;
  local_34 = 1;
  local_30 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)((int)param_2 + 4) == -1) {
    if ((*(byte *)((int)this + 0x50) & 8) != 0) {
      local_34 = 1;
      local_30 = 0x22;
      local_4 = 1;
      FUN_00520580(param_2,&local_34);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if (*(int *)((int)param_2 + 4) == -1) {
      if ((*(byte *)((int)this + 0x88) & 0x20) == 0) {
        local_34 = 0x90;
        local_30 = 6;
        local_4 = 2;
        FUN_00520580(param_2,&local_34);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      if (*(int *)((int)param_2 + 4) == -1) {
        FUN_005278c0(local_2c,this,1);
        local_4 = 3;
        iVar1 = FUN_00513180((int)local_2c);
        iVar2 = FUN_0055a0b0();
        if (iVar2 <= iVar1) {
          local_34 = 0x90;
          local_30 = 5;
          local_4._0_1_ = 4;
          FUN_00520580(param_2,&local_34);
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_005279c0(local_2c);
      }
    }
  }
  ExceptionList = local_c;
  return 1;
}

