
int __thiscall FUN_0050a300(void *this,int *param_1,void *param_2)

{
  undefined4 *this_00;
  int *piVar1;
  void *pvVar2;
  bool bVar3;
  bool bVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *pvVar5;
  void *pvVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641a58;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar3 = FUN_0053a000((int)this);
  piVar1 = param_1;
  if (CONCAT31(extraout_var,bVar3) != 0) {
    this_00 = (undefined4 *)((int)this + 0x84);
    bVar4 = FUN_00558950(this_00,param_1);
    if (CONCAT31(extraout_var_00,bVar4) == 0) {
      FUN_00558840(&param_1,this_00);
      local_4 = 0;
      FUN_00558860(this_00,piVar1);
      pvVar2 = param_2;
      pvVar6 = param_2;
      pvVar5 = (void *)FUN_00539fd0(this,1);
      FUN_0050fa90(pvVar5,pvVar6);
      pvVar6 = pvVar2;
      pvVar5 = (void *)FUN_00539fd0(this,2);
      FUN_0050fa90(pvVar5,pvVar6);
      (**(code **)(*(int *)this + 0x208))(&param_1,this_00,pvVar2);
      local_4 = 0xffffffff;
      thunk_FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar3);
}

