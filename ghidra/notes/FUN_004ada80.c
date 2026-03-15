
void __thiscall FUN_004ada80(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  
  FUN_005f4db0(param_1,(int)this + 4);
  FUN_005f4db0(param_1,(int)this + 8);
  FUN_005f49a0((void *)((int)this + 0x14),param_1);
  (**(code **)(*(int *)((int)this + 0x18) + 0xc))(param_1);
  FUN_004ecea0((void *)((int)this + 0x28),param_1);
  FUN_004ecea0((void *)((int)this + 0x2c),param_1);
  FUN_004ecea0((void *)((int)this + 0x30),param_1);
  iVar2 = (int)this + 0x34;
  iVar1 = 10;
  do {
    FUN_005f4db0(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  return;
}

