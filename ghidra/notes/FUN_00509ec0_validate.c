
int __thiscall FUN_00509ec0(void *this,int param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *pvVar4;
  void *pvVar5;
  
  bVar2 = FUN_0053a000((int)this);
  iVar3 = CONCAT31(extraout_var,bVar2);
  if (iVar3 != 0) {
    iVar3 = FUN_0053fc90(param_1,-100,100);
  }
  if ((iVar3 != 0) && (iVar1 = *(int *)((int)this + 0x6c), iVar1 != param_1)) {
    *(int *)((int)this + 0x6c) = param_1;
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,1);
    FUN_0050f590(pvVar4,pvVar5);
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,2);
    FUN_0050f590(pvVar4,pvVar5);
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 0x1e4))(iVar1,*(undefined4 *)((int)this + 0x6c),param_2);
  }
  return iVar3;
}

