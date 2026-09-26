
int __thiscall FUN_00502600(void *this,void *param_1)

{
  uint uVar1;
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
      uVar1 = *(uint *)((int)this + 100);
      if (((*(uint *)(iVar3 + 100) ^ uVar1) & 0xf000) != 0) {
        *(uint *)((int)this + 100) =
             uVar1 & 0xffff0fff | (*(uint *)(iVar3 + 100) >> 0xc & 0xf) << 0xc;
        iVar3 = FUN_004fd3d0();
        if (iVar3 != 0) {
          iVar3 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar3 != 0) {
            iVar5 = (**(code **)(*(int *)this + 0x2c8))
                              (uVar1 >> 0xc & 0xf,*(uint *)((int)this + 100) >> 0xc & 0xf,param_1);
          }
        }
      }
    }
  }
  return iVar5;
}

