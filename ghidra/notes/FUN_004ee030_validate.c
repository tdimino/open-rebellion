
int __thiscall FUN_004ee030(void *this,int param_1,void *param_2)

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
    iVar3 = FUN_0053fc90(param_1,0,100);
    FUN_0053e0f0(param_1,0,0x7fff);
  }
  if ((iVar3 != 0) && (sVar1 = *(short *)((int)this + 0x8a), sVar1 != param_1)) {
    *(short *)((int)this + 0x8a) = (short)param_1;
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,1);
    FUN_004f0190(pvVar4,pvVar5);
    pvVar5 = param_2;
    pvVar4 = (void *)FUN_00539fd0(this,2);
    FUN_004f0190(pvVar4,pvVar5);
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 0x318))((int)sVar1,(int)*(short *)((int)this + 0x8a),param_2);
  }
  return iVar3;
}

