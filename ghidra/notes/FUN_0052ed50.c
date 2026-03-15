
int __thiscall FUN_0052ed50(void *this,int *param_1,void *param_2)

{
  undefined4 *this_00;
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *pvVar3;
  void *pvVar4;
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006452f8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    this_00 = (undefined4 *)((int)this + 0x60);
    bVar2 = FUN_0051ca60(this_00,param_1);
    if (CONCAT31(extraout_var_00,bVar2) != 0) {
      FUN_00520580(local_14,this_00);
      local_4 = 0;
      FUN_00520580(this_00,param_1);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_00531710(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_00531710(pvVar3,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x1c0))(local_14,this_00,param_2);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar1);
}

