
int __thiscall FUN_00504470(void *this,int param_1,void *param_2)

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
    iVar3 = FUN_0053fc90(param_1,0,100);
  }
  if ((iVar3 != 0) && (iVar1 = *(int *)((int)this + 0x60), iVar1 != param_1)) {
    *(int *)((int)this + 0x60) = param_1;
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,1);
    FUN_005048e0(pvVar4,pvVar5);
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,2);
    FUN_005048e0(pvVar4,pvVar5);
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 500))(iVar1,*(undefined4 *)((int)this + 0x60),param_2);
  }
  return iVar3;
}

