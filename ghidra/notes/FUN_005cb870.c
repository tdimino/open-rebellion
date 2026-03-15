
void __thiscall FUN_005cb870(void *this,uint param_1)

{
  void *this_00;
  int iVar1;
  int *piVar2;
  undefined4 uVar3;
  uint uVar4;
  
  uVar4 = param_1;
  this_00 = (void *)FUN_00596bd0();
  iVar1 = FUN_005a6340(this_00,uVar4);
  if (iVar1 != 0) {
    if (*(int *)(iVar1 + 0x178) == *(int *)((int)this + 0x18)) {
      *(undefined4 *)(iVar1 + 0x178) = 0;
    }
    FUN_005c5fb0((int *)((int)this + 0x44),iVar1);
    if (param_1 == *(uint *)((int)this + 0x70)) {
      piVar2 = (int *)(**(code **)(*(int *)((int)this + 0x44) + 0x10))();
      if (piVar2 != (int *)0x0) {
        uVar3 = (**(code **)(*piVar2 + 4))();
        *(undefined4 *)((int)this + 0x70) = uVar3;
        return;
      }
      *(undefined4 *)((int)this + 0x70) = 0;
    }
  }
  return;
}

