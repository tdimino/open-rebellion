
void __thiscall FUN_005b2570(void *this,int param_1)

{
  int iVar1;
  void *this_00;
  uint uVar2;
  
  iVar1 = (**(code **)(*(int *)((int)this + 0x360) + 0x10))();
  if (iVar1 == 0) {
    iVar1 = 1;
    uVar2 = 6;
    this_00 = (void *)FUN_005ba8b0();
    FUN_005ba980(this_00,uVar2,iVar1);
    if (*(int *)((int)this + 0x150) == 0) {
      FUN_005c1360(*(uint *)((int)this + 0xec),1);
    }
  }
  FUN_005c5f20((void *)((int)this + 0x360),param_1);
  return;
}

