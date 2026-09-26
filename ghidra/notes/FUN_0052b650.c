
int __thiscall FUN_0052b650(void *this,void *param_1)

{
  void *this_00;
  bool bVar1;
  undefined3 extraout_var;
  void *this_01;
  int iVar2;
  void *pvVar3;
  int local_1c;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644af0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  local_1c = CONCAT31(extraout_var,bVar1);
  if (local_1c != 0) {
    this_01 = (void *)FUN_00539fd0(this,0);
    iVar2 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar2 != 0) {
      pvVar3 = FUN_0052b760(this_01,local_18);
      this_00 = (void *)((int)this + 0x80);
      local_4 = 0;
      iVar2 = FUN_005f3390(this_00,(int)pvVar3);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      if (iVar2 != 0) {
        FUN_005f2f90(local_18,(int)this_00);
        local_4 = 1;
        FUN_005f3090(this_00,(int)this_01 + 0x80);
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
            local_1c = (**(code **)(*(int *)this + 600))(local_18,this_00,param_1);
          }
        }
        local_4 = 0xffffffff;
        FUN_005f2ff0(local_18);
      }
    }
  }
  ExceptionList = local_c;
  return local_1c;
}

