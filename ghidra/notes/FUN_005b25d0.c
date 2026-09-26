
void __thiscall FUN_005b25d0(void *this,uint param_1)

{
  int iVar1;
  void *this_00;
  uint uVar2;
  
  FUN_005c60d0((int *)((int)this + 0x360),param_1);
  iVar1 = (**(code **)(*(int *)((int)this + 0x360) + 0x10))();
  if (iVar1 == 0) {
    iVar1 = 1;
    uVar2 = 7;
    this_00 = (void *)FUN_005ba8b0();
    FUN_005ba980(this_00,uVar2,iVar1);
    if (*(int *)((int)this + 0x150) == 0) {
      FUN_005c1360(*(uint *)((int)this + 0xec),0);
    }
  }
  return;
}

