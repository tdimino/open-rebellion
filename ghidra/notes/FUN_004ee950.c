
int __thiscall FUN_004ee950(void *this,int param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  void *pvVar3;
  void *pvVar4;
  
  bVar1 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar2 = FUN_0053a640(0x10,param_1,(uint *)((int)this + 0xac));
    if (iVar2 != 0) {
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_004f0d90(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_004f0d90(pvVar3,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x358))(param_1,param_2);
    }
  }
  return CONCAT31(extraout_var,bVar1);
}

