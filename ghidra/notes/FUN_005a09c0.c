
void __thiscall FUN_005a09c0(void *this,int param_1)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  
  (**(code **)(*(int *)((int)this + 0x10) + 4))();
  for (piVar1 = (int *)(**(code **)(*(int *)(param_1 + 0x44) + 0x10))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    uVar2 = (**(code **)(*piVar1 + 0x10))();
    FUN_005aaf90();
    iVar3 = FUN_005ad870(uVar2);
    if (iVar3 != 0) {
      iVar3 = (**(code **)(*piVar1 + 0x10))();
      FUN_005c5f20((void *)((int)this + 0x10),iVar3);
      iVar3 = (**(code **)(*piVar1 + 0x10))();
      *(undefined4 *)(iVar3 + 0x100) = 1;
      iVar3 = (**(code **)(*piVar1 + 0x10))();
      FUN_0059ed70();
      FUN_0059f4d0(iVar3);
    }
  }
  return;
}

