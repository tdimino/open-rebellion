
uint __thiscall FUN_0042f640(void *this,undefined4 param_1,uint param_2)

{
  void *pvVar1;
  uint uVar2;
  
  pvVar1 = FUN_0042f830(this,param_1,param_2);
  if (pvVar1 != (void *)0x0) {
    uVar2 = FUN_005f5440(this,pvVar1);
    *(undefined4 *)((int)pvVar1 + 100) = *(undefined4 *)((int)this + 0x10);
    *(undefined4 *)((int)pvVar1 + 0x68) = *(undefined4 *)((int)this + 0x14);
    FUN_004184f0(*(int *)((int)this + 0x14));
    return uVar2;
  }
  return param_2;
}

