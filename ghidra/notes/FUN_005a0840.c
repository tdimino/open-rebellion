
void __thiscall FUN_005a0840(void *this,int param_1)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  
  (**(code **)(*(int *)((int)this + 0x34) + 4))();
  bVar1 = true;
  for (piVar2 = (int *)(**(code **)(*(int *)(param_1 + 0x80) + 0xc))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
    iVar3 = (**(code **)(*piVar2 + 8))();
    FUN_005c5ca0((void *)((int)this + 0x34),iVar3);
    if (bVar1) {
      bVar1 = false;
      iVar3 = (**(code **)(*piVar2 + 8))();
      iVar3 = iVar3 + 0x1c;
    }
    else {
      iVar3 = (**(code **)(*piVar2 + 8))();
      iVar3 = iVar3 + 0x28;
    }
    *(undefined4 *)(iVar3 + 4) = 1;
    iVar3 = (**(code **)(*piVar2 + 8))();
    FUN_0059ed70();
    FUN_0059f560(iVar3);
  }
  return;
}

