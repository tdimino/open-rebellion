
int __thiscall FUN_0055ae60(void *this,void *param_1)

{
  undefined4 *this_00;
  bool bVar1;
  undefined3 extraout_var;
  void *this_01;
  int iVar2;
  int *piVar3;
  undefined3 extraout_var_00;
  undefined4 *puVar4;
  int local_20;
  undefined1 local_1c [8];
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649b58;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  local_20 = CONCAT31(extraout_var,bVar1);
  if (local_20 != 0) {
    this_01 = (void *)FUN_00539fd0(this,0);
    iVar2 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar2 != 0) {
      piVar3 = (int *)FUN_0052e740(this_01,local_1c);
      this_00 = (undefined4 *)((int)this + 100);
      local_4 = 0;
      bVar1 = FUN_0051ca60(this_00,piVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (CONCAT31(extraout_var_00,bVar1) != 0) {
        FUN_00520580(local_1c,this_00);
        local_4 = 1;
        puVar4 = (undefined4 *)FUN_0052e740(this_01,local_14);
        local_4._0_1_ = 2;
        FUN_00520580(this_00,puVar4);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
                    /* WARNING: Load size is inaccurate */
            local_20 = (**(code **)(*this + 0x20c))(local_1c,this_00,param_1);
          }
        }
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  ExceptionList = local_c;
  return local_20;
}

