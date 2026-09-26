
int __thiscall FUN_00529980(void *this,int param_1,void *param_2)

{
  void *this_00;
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  void *pvVar3;
  void *pvVar4;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644818;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    this_00 = (void *)((int)this + 0x80);
    iVar2 = FUN_005f3390(this_00,param_1);
    if (iVar2 != 0) {
      FUN_005f2f90(local_18,(int)this_00);
      local_4 = 0;
      FUN_005f3090(this_00,param_1);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_0052b650(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_0052b650(pvVar3,pvVar4);
      (**(code **)(*(int *)this + 0x22c))(local_18,this_00,param_2);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
    }
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar1);
}

