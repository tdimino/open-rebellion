
int __thiscall FUN_004ee3e0(void *this,int param_1,void *param_2)

{
  short sVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *pvVar4;
  void *pvVar5;
  
  bVar2 = FUN_0053a000((int)this);
  iVar3 = CONCAT31(extraout_var,bVar2);
  if (iVar3 != 0) {
    iVar3 = FUN_0053fc90(param_1,0,4);
  }
  if ((iVar3 != 0) && (sVar1 = *(short *)((int)this + 0x98), sVar1 != param_1)) {
    *(short *)((int)this + 0x98) = (short)param_1;
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,1);
    FUN_004f0580(pvVar4,pvVar5);
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,2);
    FUN_004f0580(pvVar4,pvVar5);
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 0x334))((int)sVar1,(int)*(short *)((int)this + 0x98),param_2);
  }
  return iVar3;
}

