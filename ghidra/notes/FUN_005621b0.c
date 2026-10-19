
uint __thiscall FUN_005621b0(void *this,void *param_1)

{
  uint uVar1;
  
  uVar1 = FUN_004f9340(this,param_1);
  if (uVar1 != 0) {
    FUN_005f4d90(param_1,(int)this + 0x58);
    if ((uVar1 != 0) && ((*(uint *)((int)this + 0x24) & 0x30) == 0)) {
      FUN_005f4d90(param_1,(int)this + 0x5c);
    }
  }
  return uVar1;
}

