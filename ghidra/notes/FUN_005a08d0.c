
void __thiscall FUN_005a08d0(void *this,int param_1)

{
  bool bVar1;
  int *piVar2;
  undefined4 uVar3;
  int iVar4;
  int *piVar5;
  
  (**(code **)(*(int *)((int)this + 0x1c) + 4))();
  bVar1 = true;
  piVar2 = (int *)(**(code **)(*(int *)(param_1 + 0x98) + 0x10))();
  do {
    if (piVar2 == (int *)0x0) {
      return;
    }
    uVar3 = (**(code **)(*piVar2 + 0x10))();
    FUN_005aaf90();
    iVar4 = FUN_005ad870(uVar3);
    if (iVar4 != 0) {
      iVar4 = (**(code **)(*piVar2 + 0x10))();
      FUN_005c5f20((void *)((int)this + 0x1c),iVar4);
      if (bVar1) {
        bVar1 = false;
        piVar5 = (int *)(**(code **)(*piVar2 + 0x10))();
        iVar4 = (**(code **)(*piVar5 + 0x20))();
        if (iVar4 == 0) {
          iVar4 = (**(code **)(*piVar2 + 0x10))();
          iVar4 = iVar4 + 0x10c;
          goto LAB_005a0958;
        }
      }
      else {
        iVar4 = (**(code **)(*piVar2 + 0x10))();
        iVar4 = iVar4 + 0xfc;
LAB_005a0958:
        *(undefined4 *)(iVar4 + 4) = 1;
      }
      piVar5 = (int *)(**(code **)(*piVar2 + 0x10))();
      iVar4 = (**(code **)(*piVar5 + 0x20))();
      if (iVar4 == 0) {
        iVar4 = (**(code **)(*piVar2 + 0x10))();
        FUN_0059ed70();
        FUN_0059f4d0(iVar4);
      }
    }
    piVar2 = (int *)(**(code **)(*piVar2 + 8))();
  } while( true );
}

