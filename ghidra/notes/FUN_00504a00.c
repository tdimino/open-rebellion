
int __thiscall FUN_00504a00(void *this,int param_1,int param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  uint *puVar3;
  undefined4 *puVar4;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640ce8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  bVar1 = FUN_0053a010((int)this);
  iVar2 = CONCAT31(extraout_var,bVar1);
  FUN_004ece30(&local_10);
  local_4 = 0;
  if (iVar2 != 0) {
    iVar2 = FUN_004f6b70(this,(uint *)&local_10);
    if ((iVar2 != 0) && (param_1 != 0)) {
      puVar3 = (uint *)FUN_004ece30(&param_1);
      iVar2 = param_2;
      local_4._0_1_ = 1;
      FUN_004f8aa0(this,s__TroopRegDestroyedRunningBlockad_006a9860,
                   s_TroopRegDestroyedRunningBlockade_006a9888,(uint *)&local_10,puVar3,(char *)0x0,
                   param_2);
      local_4._0_1_ = 0;
      FUN_00619730();
      puVar4 = (undefined4 *)FUN_004ece30(&param_1);
      local_4._0_1_ = 2;
      iVar2 = FUN_0053fe40(0x340,this,&local_10,puVar4,iVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar2;
}

