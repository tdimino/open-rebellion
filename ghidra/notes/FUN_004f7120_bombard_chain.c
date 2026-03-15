
int __thiscall FUN_004f7120(void *this,int *param_1,void *param_2)

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
  puStack_8 = &LAB_0063fd78;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar3 = FUN_0053a000((int)this);
  piVar1 = param_1;
  if (CONCAT31(extraout_var,bVar3) != 0) {
    this_00 = (undefined4 *)((int)this + 0x3c);
    if (*(int *)((int)this + 0x3c) != *param_1) {
      FUN_004f26d0(&param_1,this_00);
      local_4 = 0;
      FUN_004f26d0(this_00,piVar1);
      pvVar2 = param_2;
      pvVar5 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,1);
      FUN_004fa030(pvVar4,pvVar5);
      pvVar5 = pvVar2;
      pvVar4 = (void *)FUN_00539fd0(this,2);
      FUN_004fa030(pvVar4,pvVar5);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0xfc))(&param_1,this_00,pvVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar3);
}

