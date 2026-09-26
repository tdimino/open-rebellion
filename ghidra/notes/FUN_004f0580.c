
int __thiscall FUN_004f0580(void *this,void *param_1)

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
      sVar1 = *(short *)((int)this + 0x98);
      if (sVar1 != *(short *)(iVar3 + 0x98)) {
        *(short *)((int)this + 0x98) = *(short *)(iVar3 + 0x98);
        iVar3 = FUN_004fd3d0();
        if (iVar3 != 0) {
          iVar3 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar3 != 0) {
            iVar5 = (**(code **)(*(int *)this + 0x3ac))
                              ((int)sVar1,(int)*(short *)((int)this + 0x98),param_1);
          }
        }
      }
    }
  }
  return iVar5;
}

