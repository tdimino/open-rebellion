
void * __thiscall FUN_00458b80(void *this,void *param_1,undefined4 param_2,int param_3)

{
  undefined1 uVar1;
  undefined4 *puVar2;
  void *this_00;
  int *piVar3;
  short sVar4;
  short sVar5;
  bool bVar6;
  undefined4 local_18;
  undefined4 *local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630616;
  local_c = ExceptionList;
  local_14 = (undefined4 *)((int)this + 0x144);
  ExceptionList = &local_c;
  FUN_004f26d0(&local_18,local_14);
  local_4 = 1;
  sVar5 = (short)param_2;
  sVar4 = (short)param_3;
  param_2 = CONCAT22(sVar4,sVar5);
  if (*(int *)((int)this + 0x160) != 0x67) {
    FUN_004ece30(&param_3);
    param_2 = CONCAT22(sVar4 + -0x4d,sVar5 + -8);
    local_4._0_1_ = 2;
    puVar2 = (undefined4 *)FUN_00609d80(*(void **)((int)this + 0x174),&param_2,param_2);
    this_00 = (void *)FUN_00609d10(*(void **)((int)this + 0x174),*puVar2);
    if (this_00 != (void *)0x0) {
      piVar3 = FUN_0042d170(this_00,local_10);
      bVar6 = *piVar3 != param_3;
      local_4._0_1_ = 3;
      uVar1 = (undefined1)local_4;
      local_4._0_1_ = 3;
      puVar2 = local_14;
      if (bVar6) {
        puVar2 = FUN_0042d170(this_00,&param_2);
        local_4._0_1_ = 4;
        uVar1 = (undefined1)local_4;
      }
      local_4._0_1_ = uVar1;
      FUN_004f26d0(&local_18,puVar2);
      local_4 = 3;
      if (bVar6) {
        FUN_00619730();
      }
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  FUN_004f26d0(param_1,&local_18);
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

