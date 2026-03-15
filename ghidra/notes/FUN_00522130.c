
int __thiscall FUN_00522130(void *this,int param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  void *pvVar3;
  void *pvVar4;
  
  bVar1 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar2 = FUN_0053a640(1,param_1,(uint *)((int)this + 0xa4));
    if (iVar2 != 0) {
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_005248b0(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_005248b0(pvVar3,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x220))(param_1,param_2);
    }
  }
  return CONCAT31(extraout_var,bVar1);
}

