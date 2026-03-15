
uint __thiscall FUN_00503370(void *this,int param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  
  uVar2 = (uint)(-1 < param_1);
  if (uVar2 != 0) {
                    /* WARNING: Load size is inaccurate */
    iVar3 = *(int *)((int)this + 0x60) + param_1;
    iVar1 = (**(code **)(*this + 0x244))();
    if (iVar1 < iVar3) {
                    /* WARNING: Load size is inaccurate */
      iVar3 = (**(code **)(*this + 0x244))();
    }
    uVar2 = FUN_005032c0(this,iVar3,param_3);
  }
  if (uVar2 != 0) {
                    /* WARNING: Load size is inaccurate */
    iVar1 = (**(code **)(*this + 0x254))();
                    /* WARNING: Load size is inaccurate */
    iVar3 = (**(code **)(*this + 0x244))();
    if (iVar3 == iVar1) {
                    /* WARNING: Load size is inaccurate */
      uVar2 = (**(code **)(*this + 0xac))(param_2,param_3);
    }
  }
  return uVar2;
}

