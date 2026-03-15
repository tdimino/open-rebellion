
int __thiscall FUN_00535380(void *this,void *param_1)

{
  short sVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  int iVar4;
  int iVar5;
  
  bVar2 = FUN_0053a010((int)this);
  iVar5 = CONCAT31(extraout_var,bVar2);
  if (iVar5 != 0) {
    iVar3 = FUN_00539fd0(this,0);
    iVar4 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar4 != 0) {
      sVar1 = *(short *)((int)this + 0x66);
      if (sVar1 != *(short *)(iVar3 + 0x66)) {
        *(short *)((int)this + 0x66) = *(short *)(iVar3 + 0x66);
        iVar3 = FUN_004fd3d0();
        if (iVar3 != 0) {
          iVar3 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar3 != 0) {
                    /* WARNING: Load size is inaccurate */
            iVar5 = (**(code **)(*this + 0x290))
                              ((int)sVar1,(int)*(short *)((int)this + 0x66),param_1);
          }
        }
      }
    }
  }
  return iVar5;
}

