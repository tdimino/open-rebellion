
uint __thiscall FUN_0050d510(void *this,int param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  uint uVar3;
  int iVar4;
  undefined3 extraout_var_00;
  void *local_4;
  
  local_4 = this;
  bVar1 = FUN_0053a000((int)this);
  if ((CONCAT31(extraout_var,bVar1) != 0) &&
     (((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) != 0xc0)) {
    local_4 = (void *)0x0;
    iVar2 = FUN_00559db0(&local_4);
    uVar3 = *(uint *)((int)this + 0x24) >> 6 & 3;
    if (uVar3 == 3) {
      iVar4 = 3;
    }
    else {
      iVar4 = 2 - (uint)(uVar3 != 1);
    }
    bVar1 = FUN_0050d5a0(this,local_4,iVar4,param_1);
    if ((CONCAT31(extraout_var_00,bVar1) != 0) && (iVar2 != 0)) {
      return 1;
    }
  }
  return 0;
}

