
int __thiscall FUN_004f06a0(void *this,void *param_1)

{
  undefined4 *this_00;
  int iVar1;
  bool bVar2;
  undefined3 extraout_var;
  void *this_01;
  int iVar3;
  int *piVar4;
  undefined4 *puVar5;
  undefined1 local_18 [4];
  int local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ef40;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar2 = FUN_0053a010((int)this);
  local_14 = CONCAT31(extraout_var,bVar2);
  if (local_14 != 0) {
    this_01 = (void *)FUN_00539fd0(this,0);
    iVar3 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar3 != 0) {
      piVar4 = FUN_004f07c0(this_01,local_18);
      iVar3 = *(int *)((int)this + 0x9c);
      iVar1 = *piVar4;
      this_00 = (undefined4 *)((int)this + 0x9c);
      FUN_00619730();
      if (iVar3 != iVar1) {
        FUN_004f26d0(local_18,this_00);
        local_4 = 0;
        puVar5 = FUN_004f07c0(this_01,local_10);
        local_4._0_1_ = 1;
        FUN_004f26d0(this_00,puVar5);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        iVar3 = FUN_004fd3d0();
        if (iVar3 != 0) {
          iVar3 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar3 != 0) {
            local_14 = (**(code **)(*(int *)this + 0x3b4))(local_18,this_00,param_1);
          }
        }
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  ExceptionList = local_c;
  return local_14;
}

