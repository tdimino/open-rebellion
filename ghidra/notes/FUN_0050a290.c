
int __thiscall FUN_0050a290(void *this,int param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  undefined3 extraout_var;
  void *pvVar3;
  void *pvVar4;
  
  bVar2 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    iVar1 = *(int *)((int)this + 0x80);
    if (iVar1 != param_1) {
      *(int *)((int)this + 0x80) = param_1;
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_0050fa00(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_0050fa00(pvVar3,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x204))(iVar1,*(undefined4 *)((int)this + 0x80),param_2);
    }
  }
  return CONCAT31(extraout_var,bVar2);
}

