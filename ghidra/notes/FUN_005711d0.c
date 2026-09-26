
int __thiscall FUN_005711d0(void *this,void *param_1)

{
  undefined4 *this_00;
  void *this_01;
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int iVar3;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064c1f8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  local_10 = CONCAT31(extraout_var,bVar1);
  if (local_10 != 0) {
    iVar2 = FUN_00539fd0(this,0);
    this_01 = param_1;
    iVar3 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar3 != 0) {
      this_00 = (undefined4 *)((int)this + 0xa8);
      if (*(int *)((int)this + 0xa8) != *(int *)(iVar2 + 0xa8)) {
        FUN_004f26d0(&param_1,this_00);
        local_4 = 0;
        FUN_004f26d0(this_00,(undefined4 *)(iVar2 + 0xa8));
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(this_01,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
            local_10 = (**(code **)(*(int *)this + 0x28c))(&param_1,this_00,this_01);
          }
        }
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  ExceptionList = local_c;
  return local_10;
}

