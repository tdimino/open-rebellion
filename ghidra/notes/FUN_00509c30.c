
uint __thiscall FUN_00509c30(void *this,int param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  undefined3 extraout_var;
  void *pvVar3;
  uint uVar4;
  void *pvVar5;
  
  bVar2 = FUN_0053a000((int)this);
  uVar4 = CONCAT31(extraout_var,bVar2);
  bVar2 = uVar4 == 0;
  if (!bVar2) {
    uVar4 = *(uint *)((int)this + 0x88) & 1;
    bVar2 = uVar4 == 0;
  }
  if (!bVar2) {
    if ((param_1 < 0) || (100 < param_1)) {
      uVar4 = 0;
    }
    else {
      uVar4 = 1;
    }
  }
  if ((uVar4 != 0) && (iVar1 = *(int *)((int)this + 0x58), iVar1 != param_1)) {
    *(int *)((int)this + 0x58) = param_1;
    pvVar5 = param_2;
    pvVar3 = (void *)FUN_00539fd0(this,1);
    FUN_0050f390(pvVar3,pvVar5);
    pvVar5 = param_2;
    pvVar3 = (void *)FUN_00539fd0(this,2);
    FUN_0050f390(pvVar3,pvVar5);
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 0x1d0))(iVar1,*(undefined4 *)((int)this + 0x58),param_2);
  }
  return uVar4;
}

