
int __thiscall FUN_00501640(void *this,uint param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  void *pvVar3;
  uint uVar4;
  void *pvVar5;
  
  bVar1 = FUN_0053a000((int)this);
  iVar2 = CONCAT31(extraout_var,bVar1);
  if (iVar2 != 0) {
    iVar2 = FUN_00500670();
    iVar2 = FUN_0053fc90(param_1,0,iVar2);
  }
  if (iVar2 != 0) {
    uVar4 = *(uint *)((int)this + 100) >> 8 & 0xf;
    if (uVar4 != param_1) {
      *(uint *)((int)this + 100) = (param_1 & 0xf) << 8 | *(uint *)((int)this + 100) & 0xfffff0ff;
      pvVar5 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_00502560(pvVar3,pvVar5);
      pvVar5 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_00502560(pvVar3,pvVar5);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x2a8))(uVar4,*(uint *)((int)this + 100) >> 8 & 0xf,param_2);
    }
  }
  return iVar2;
}

