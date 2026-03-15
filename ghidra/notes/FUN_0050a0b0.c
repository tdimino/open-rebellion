
int __thiscall FUN_0050a0b0(void *this,uint param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  void *pvVar2;
  uint uVar3;
  void *pvVar4;
  
  bVar1 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    uVar3 = *(uint *)((int)this + 0x78) >> 2 & 3;
    if (uVar3 != param_1) {
      *(uint *)((int)this + 0x78) = (param_1 & 3) << 2 | *(uint *)((int)this + 0x78) & 0xfffffff3;
      pvVar4 = param_2;
      pvVar2 = (void *)FUN_00539fd0(this,1);
      FUN_0050f7a0(pvVar2,pvVar4);
      pvVar4 = param_2;
      pvVar2 = (void *)FUN_00539fd0(this,2);
      FUN_0050f7a0(pvVar2,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 500))(uVar3,*(uint *)((int)this + 0x78) >> 2 & 3,param_2);
    }
  }
  return CONCAT31(extraout_var,bVar1);
}

