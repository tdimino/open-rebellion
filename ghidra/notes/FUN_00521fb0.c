
int __thiscall FUN_00521fb0(void *this,void *param_1,void *param_2)

{
  void *this_00;
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *pvVar3;
  void *pvVar4;
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643928;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    this_00 = (void *)((int)this + 0x94);
    bVar2 = FUN_004f4430(this_00,(int)param_1);
    if (CONCAT31(extraout_var_00,bVar2) != 0) {
      FUN_004f4390(local_14,(int)this_00);
      local_4 = 0;
      FUN_004f43b0(this_00,param_1);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_005246f0(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_005246f0(pvVar3,pvVar4);
      (**(code **)(*(int *)this + 0x218))(local_14,this_00,param_2);
      local_4 = 0xffffffff;
      FUN_004f4380(local_14);
    }
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar1);
}

