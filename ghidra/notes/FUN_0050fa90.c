
int __thiscall FUN_0050fa90(void *this,void *param_1)

{
  undefined4 *this_00;
  bool bVar1;
  undefined3 extraout_var;
  void *this_01;
  int iVar2;
  int *piVar3;
  undefined3 extraout_var_00;
  undefined4 *puVar4;
  undefined1 local_18 [4];
  int local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642068;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  local_14 = CONCAT31(extraout_var,bVar1);
  if (local_14 != 0) {
    this_01 = (void *)FUN_00539fd0(this,0);
    iVar2 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar2 != 0) {
      piVar3 = FUN_0050fbc0(this_01,local_18);
      this_00 = (undefined4 *)((int)this + 0x84);
      local_4 = 0;
      bVar1 = FUN_00558950(this_00,piVar3);
      local_4 = 0xffffffff;
      thunk_FUN_00619730();
      if (CONCAT31(extraout_var_00,bVar1) == 0) {
        FUN_00558840(local_18,this_00);
        local_4 = 1;
        puVar4 = FUN_0050fbc0(this_01,local_10);
        local_4._0_1_ = 2;
        FUN_00558860(this_00,puVar4);
        local_4 = CONCAT31(local_4._1_3_,1);
        thunk_FUN_00619730();
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
            local_14 = (**(code **)(*(int *)this + 0x294))(local_18,this_00,param_1);
          }
        }
        local_4 = 0xffffffff;
        thunk_FUN_00619730();
      }
    }
  }
  ExceptionList = local_c;
  return local_14;
}

