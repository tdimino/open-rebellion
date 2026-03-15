
int __thiscall FUN_004f71d0(void *this,uint param_1,void *param_2)

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
    iVar3 = FUN_0053fc90(param_1,0,param_1);
  }
  if (iVar3 != 0) {
    uVar1 = *(uint *)((int)this + 0x40);
    if ((uVar1 & 0xff) != param_1) {
      *(uint *)((int)this + 0x40) = (param_1 ^ uVar1) & 0xff ^ uVar1;
      pvVar5 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,1);
      FUN_004fa170(pvVar4,pvVar5);
      pvVar5 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,2);
      FUN_004fa170(pvVar4,pvVar5);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x100))(uVar1 & 0xff,*(uint *)((int)this + 0x40) & 0xff,param_2);
    }
  }
  return iVar3;
}

