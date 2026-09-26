
int __thiscall FUN_0050f7a0(void *this,void *param_1)

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
      uVar1 = *(uint *)((int)this + 0x78);
      if (((uVar1 ^ *(uint *)(iVar3 + 0x78)) & 0xc) != 0) {
        *(uint *)((int)this + 0x78) = uVar1 & 0xfffffff3 | (*(uint *)(iVar3 + 0x78) >> 2 & 3) << 2;
        iVar3 = FUN_004fd3d0();
        if (iVar3 != 0) {
          iVar3 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar3 != 0) {
            iVar5 = (**(code **)(*(int *)this + 0x280))
                              (uVar1 >> 2 & 3,*(uint *)((int)this + 0x78) >> 2 & 3,param_1);
          }
        }
      }
    }
  }
  return iVar5;
}

