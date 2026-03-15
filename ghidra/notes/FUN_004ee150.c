
int __thiscall FUN_004ee150(void *this,int param_1,void *param_2)

{
  short sVar1;
  bool bVar2;
  undefined3 extraout_var;
  void *pvVar3;
  void *pvVar4;
  
  bVar2 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    FUN_0053e0f0(param_1,0,0x7fff);
    sVar1 = *(short *)((int)this + 0x8e);
    if (sVar1 != param_1) {
      *(short *)((int)this + 0x8e) = (short)param_1;
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_004f02b0(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_004f02b0(pvVar3,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 800))((int)sVar1,(int)*(short *)((int)this + 0x8e),param_2);
    }
  }
  return CONCAT31(extraout_var,bVar2);
}

