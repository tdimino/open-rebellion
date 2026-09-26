
undefined4 __thiscall FUN_0053b1d0(void *this,int param_1,int param_2,void *param_3)

{
  int *piVar1;
  bool bVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  
  if ((param_2 == 1) || (param_1 == 1)) {
    piVar1 = (int *)(*(int *)((int)this + 0x54) + 0x24);
    *piVar1 = *piVar1 + 1;
  }
  uVar3 = *(uint *)((int)this + 0x60) >> 4 & 1;
  if (param_2 == 1) {
    uVar3 = (uint)(param_1 == 4);
  }
  else if (param_2 == 4) {
    uVar3 = 0;
  }
  iVar4 = FUN_0053a9b0(this,uVar3,param_3);
  iVar5 = FUN_0053a940(this,(uint)(param_2 == 2),param_3);
  if ((iVar5 == 0) || (iVar4 == 0)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  iVar4 = FUN_0053aa40(this,param_3);
  if ((iVar4 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  uVar3 = FUN_0053f950(0x315,this,param_1,param_2,param_3);
  if ((uVar3 != 0) && (bVar2)) {
    return 1;
  }
  return 0;
}

