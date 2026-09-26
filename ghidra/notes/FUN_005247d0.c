
int __thiscall FUN_005247d0(void *this,void *param_1)

{
  void *this_00;
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int iVar3;
  undefined3 extraout_var_00;
  int local_18;
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643ca8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  local_18 = CONCAT31(extraout_var,bVar1);
  if (local_18 != 0) {
    iVar2 = FUN_00539fd0(this,0);
    iVar3 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar3 != 0) {
      this_00 = (void *)((int)this + 0x9c);
      bVar1 = FUN_004f4430(this_00,iVar2 + 0x9c);
      if (CONCAT31(extraout_var_00,bVar1) != 0) {
        FUN_004f4390(local_14,(int)this_00);
        local_4 = 0;
        FUN_004f43b0(this_00,(void *)(iVar2 + 0x9c));
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
            local_18 = (**(code **)(*(int *)this + 0x264))(local_14,this_00,param_1);
          }
        }
        local_4 = 0xffffffff;
        FUN_004f4380(local_14);
      }
    }
  }
  ExceptionList = local_c;
  return local_18;
}

