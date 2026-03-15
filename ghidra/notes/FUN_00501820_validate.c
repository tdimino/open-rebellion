
int __thiscall FUN_00501820(void *this,uint param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar2;
  void *pvVar3;
  uint uVar4;
  void *pvVar5;
  
  bVar1 = FUN_0053a000((int)this);
  iVar2 = CONCAT31(extraout_var,bVar1);
  if (iVar2 != 0) {
    bVar1 = FUN_00500690((int *)this);
    iVar2 = FUN_0053fc90(param_1,0,CONCAT31(extraout_var_00,bVar1));
  }
  if (iVar2 != 0) {
    uVar4 = *(uint *)((int)this + 100) >> 0x14 & 0xf;
    if (uVar4 != param_1) {
      *(uint *)((int)this + 100) = (param_1 & 0xf) << 0x14 | *(uint *)((int)this + 100) & 0xff0fffff
      ;
      pvVar5 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_00502750(pvVar3,pvVar5);
      pvVar5 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_00502750(pvVar3,pvVar5);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x2b4))(uVar4,*(uint *)((int)this + 100) >> 0x14 & 0xf,param_2);
    }
  }
  return iVar2;
}

