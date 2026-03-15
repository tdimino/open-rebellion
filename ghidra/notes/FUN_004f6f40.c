
int __thiscall FUN_004f6f40(void *this,uint param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  void *pvVar2;
  int iVar3;
  uint uVar4;
  void *pvVar5;
  
  bVar1 = FUN_0053a000((int)this);
  iVar3 = CONCAT31(extraout_var,bVar1);
  if (iVar3 != 0) {
    if (((int)param_1 < 1) || (3 < (int)param_1)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  if ((iVar3 != 0) && (uVar4 = *(uint *)((int)this + 0x24) >> 6 & 3, uVar4 != param_1)) {
    FUN_00540670(this,param_1);
    pvVar5 = param_2;
    pvVar2 = (void *)FUN_00539fd0(this,1);
    FUN_004f9ca0(pvVar2,pvVar5);
    pvVar5 = param_2;
    pvVar2 = (void *)FUN_00539fd0(this,2);
    FUN_004f9ca0(pvVar2,pvVar5);
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 0xf0))(uVar4,*(uint *)((int)this + 0x24) >> 6 & 3,param_2);
  }
  return iVar3;
}

