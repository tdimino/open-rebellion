
bool __thiscall FUN_004fc600(void *this,int *param_1,void *param_2)

{
  void *this_00;
  uint uVar1;
  int iVar2;
  bool bVar3;
  
  this_00 = (void *)param_1[7];
  uVar1 = FUN_0053a4f0(this,param_1,(int *)&param_1);
  bVar3 = uVar1 != 0;
  if ((*(byte *)((int)this + 0x50) & 8) != 0) {
    iVar2 = FUN_004f82d0(this,param_2);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  if ((*(byte *)((int)this_00 + 0x50) & 8) != 0) {
    iVar2 = FUN_004f82d0(this_00,param_2);
    if ((iVar2 != 0) && (bVar3)) {
      return true;
    }
    bVar3 = false;
  }
  return bVar3;
}

