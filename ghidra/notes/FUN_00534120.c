
int __thiscall FUN_00534120(void *this,int param_1,void *param_2)

{
  short sVar1;
  bool bVar2;
  undefined3 extraout_var;
  void *pvVar3;
  void *pvVar4;
  
  bVar2 = FUN_0053a000((int)this);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    FUN_0053e0f0(param_1,0,0x7fff);
    sVar1 = *(short *)((int)this + 100);
    if (sVar1 != param_1) {
      *(short *)((int)this + 100) = (short)param_1;
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,1);
      FUN_00535300(pvVar3,pvVar4);
      pvVar4 = param_2;
      pvVar3 = (void *)FUN_00539fd0(this,2);
      FUN_00535300(pvVar3,pvVar4);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x234))((int)sVar1,(int)*(short *)((int)this + 100),param_2);
    }
  }
  return CONCAT31(extraout_var,bVar2);
}

