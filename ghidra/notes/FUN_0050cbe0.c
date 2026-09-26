
uint __thiscall FUN_0050cbe0(void *this,void *param_1)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  uint *puVar4;
  uint local_4;
  
  uVar2 = 0;
  if (((byte)*(uint *)((int)this + 0x24) & 0xc0) != 0xc0) {
    puVar4 = &local_4;
    local_4 = 0;
    iVar1 = FUN_00507270(this,*(uint *)((int)this + 0x24) >> 6 & 3);
    uVar2 = FUN_00559f90(iVar1,puVar4);
    if ((uVar2 != 0) && (local_4 != 0)) {
      iVar1 = FUN_0050aac0(this,1,param_1);
      iVar3 = FUN_0050aac0(this,0,param_1);
      if ((iVar3 != 0) && (iVar1 != 0)) {
        return 1;
      }
      uVar2 = 0;
    }
  }
  return uVar2;
}

