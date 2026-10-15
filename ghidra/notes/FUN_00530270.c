
int __thiscall FUN_00530270(void *this,int param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = 1;
  uVar2 = *(uint *)(param_1 + 0x40) & 0xff;
  if (((0x13 < uVar2) && (uVar2 < 0x17)) && ((*(byte *)(param_1 + 0x50) & 4) != 0)) {
    iVar1 = FUN_004f2980(param_1);
    iVar1 = FUN_0052eff0(this,*(int *)((int)this + 0x7c) + iVar1 / 2,param_2);
  }
  return iVar1;
}

