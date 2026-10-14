
uint __thiscall FUN_0042ef50(void *this,uint param_1)

{
  void *pvVar1;
  uint uVar2;
  
  pvVar1 = FUN_0042efe0(this,param_1);
  if (pvVar1 != (void *)0x0) {
    uVar2 = FUN_005f5440(this,pvVar1);
    *(undefined4 *)((int)pvVar1 + 0x40) = *(undefined4 *)((int)this + 0x10);
    *(undefined4 *)((int)pvVar1 + 0x44) = *(undefined4 *)((int)this + 0x14);
    return uVar2;
  }
  return param_1;
}

