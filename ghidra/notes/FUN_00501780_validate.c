
int __thiscall FUN_00501780(void *this,uint param_1,void *param_2)

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
    bVar1 = FUN_00500680((int *)this);
    iVar2 = FUN_0053fc90(param_1,0,CONCAT31(extraout_var_00,bVar1));
  }
  if (iVar2 != 0) {
    uVar4 = *(uint *)((int)this + 100) >> 0x10 & 0xf;
    if (uVar4 != param_1) {
      *(uint *)((int)this + 100) = (param_1 & 0xf) << 0x10 | *(uint *)((int)this + 100) & 0xfff0ffff
      ;
      pvVar5 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_005026a0(pvVar3,pvVar5);
      pvVar5 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_005026a0(pvVar3,pvVar5);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x2b0))(uVar4,*(uint *)((int)this + 100) >> 0x10 & 0xf,param_2);
    }
  }
  return iVar2;
}

