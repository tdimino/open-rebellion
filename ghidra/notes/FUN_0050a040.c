
int __thiscall FUN_0050a040(void *this,uint param_1,void *param_2)

{
  uint uVar1;
  bool bVar2;
  undefined3 extraout_var;
  void *pvVar3;
  void *pvVar4;
  
  bVar2 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    uVar1 = *(uint *)((int)this + 0x78);
    if ((uVar1 & 3) != param_1) {
      *(uint *)((int)this + 0x78) = (param_1 ^ uVar1) & 3 ^ uVar1;
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_0050f710(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_0050f710(pvVar3,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x1f0))(uVar1 & 3,*(uint *)((int)this + 0x78) & 3,param_2);
    }
  }
  return CONCAT31(extraout_var,bVar2);
}

