
int __thiscall FUN_00501510(void *this,uint param_1,void *param_2)

{
  uint uVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *pvVar4;
  void *pvVar5;
  
  bVar2 = FUN_0053a000((int)this);
  iVar3 = CONCAT31(extraout_var,bVar2);
  if (iVar3 != 0) {
    iVar3 = FUN_00500670();
    iVar3 = FUN_0053fc90(param_1,0,iVar3);
  }
  if (iVar3 != 0) {
    uVar1 = *(uint *)((int)this + 100);
    if ((uVar1 & 0xf) != param_1) {
      *(uint *)((int)this + 100) = (param_1 ^ uVar1) & 0xf ^ uVar1;
      pvVar5 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,1);
      FUN_00502430(pvVar4,pvVar5);
      pvVar5 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,2);
      FUN_00502430(pvVar4,pvVar5);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x2a0))(uVar1 & 0xf,*(uint *)((int)this + 100) & 0xf,param_2);
    }
  }
  return iVar3;
}

