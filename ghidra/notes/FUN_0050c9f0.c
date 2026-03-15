
bool __thiscall FUN_0050c9f0(void *this,int param_1,int param_2,void *param_3)

{
  int iVar1;
  int iVar2;
  int iVar3;
  bool bVar4;
  
  bVar4 = true;
  if (((0 < param_2) && (param_2 < 4)) && ((*(uint *)((int)this + 0x88) & 1) != 0)) {
    if (param_2 == 1) {
      iVar3 = 1;
    }
    else if (param_2 == 2) {
      iVar3 = 2;
    }
    else {
      iVar3 = 0;
    }
    bVar4 = iVar3 != 0;
    if (bVar4) {
      iVar1 = FUN_00559be0(param_1,iVar3,*(uint *)((int)this + 0x88) >> 0xb & 1);
      iVar2 = FUN_00507270(this,iVar3);
      iVar1 = FUN_0053e0d0(iVar2 + iVar1);
      iVar3 = FUN_00509c00(this,iVar1,iVar3,param_3);
      bVar4 = iVar3 != 0;
    }
  }
  return bVar4;
}

