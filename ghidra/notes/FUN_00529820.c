
int __thiscall FUN_00529820(void *this,int *param_1,void *param_2)

{
  undefined4 *this_00;
  int *piVar1;
  void *pvVar2;
  bool bVar3;
  undefined3 extraout_var;
  void *pvVar4;
  void *pvVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006447d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar3 = FUN_0053a000((int)this);
  piVar1 = param_1;
  if (CONCAT31(extraout_var,bVar3) != 0) {
    this_00 = (undefined4 *)((int)this + 0x78);
    if (*(int *)((int)this + 0x78) != *param_1) {
      FUN_004f26d0(&param_1,this_00);
      local_4 = 0;
      FUN_004f26d0(this_00,piVar1);
      pvVar2 = param_2;
      pvVar5 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,1);
      FUN_0052b410(pvVar4,pvVar5);
      pvVar5 = pvVar2;
      pvVar4 = (void *)FUN_00539fd0(this,2);
      FUN_0052b410(pvVar4,pvVar5);
      (**(code **)(*(int *)this + 0x224))(&param_1,this_00,pvVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar3);
}

