
undefined4 __thiscall FUN_005a1a60(int param_1,undefined4 param_2)

{
  int iVar1;
  void *pvVar2;
  int *piVar3;
  uint uVar4;
  int iVar5;
  
  iVar1 = FUN_005c6070((int *)(param_1 + 4));
  pvVar2 = (void *)FUN_005aaf90();
  piVar3 = FUN_005aded0(pvVar2,iVar1);
  if (piVar3 != (int *)0x0) {
    iVar1 = FUN_005aaf90();
    iVar5 = 1;
    if (*(int *)(iVar1 + 0x9ec) == 0) {
      uVar4 = 0x77;
    }
    else {
      uVar4 = 0xf5;
    }
    pvVar2 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar2,uVar4,iVar5);
    (**(code **)(*piVar3 + 100))(param_2);
  }
  return 0;
}

