
uint __thiscall FUN_005f58d0(void *this,int param_1)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = 0;
  if (param_1 != 0) {
    uVar2 = *(uint *)(param_1 + 0xc);
    if (uVar2 == 0) {
      *(undefined4 *)(param_1 + 0xc) = *(undefined4 *)((int)this + 0xc);
    }
    iVar1 = FUN_005f59f0(this,param_1);
    if (iVar1 != 0) {
      if (uVar2 == 0) {
        uVar2 = *(uint *)((int)this + 0xc);
        *(uint *)((int)this + 0xc) = uVar2 + 1;
        return uVar2;
      }
      if (*(uint *)((int)this + 0xc) <= uVar2) {
        *(uint *)((int)this + 0xc) = uVar2 + 1;
      }
    }
  }
  return uVar2;
}

