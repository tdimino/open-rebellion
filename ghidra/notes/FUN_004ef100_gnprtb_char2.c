
void __thiscall FUN_004ef100(void *this,void *param_1)

{
  uint uVar1;
  int iVar2;
  
  if (*(short *)((int)this + 0x94) != 0) {
    iVar2 = DAT_006b9084;
    if ((*(byte *)((int)this + 0xac) & 4) == 0) {
      iVar2 = DAT_006b905c;
    }
    uVar1 = FUN_0053e120(*(short *)((int)this + 0x94) - iVar2);
    FUN_004ee2d0(this,uVar1,param_1);
  }
  return;
}

