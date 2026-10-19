
void __cdecl FUN_005b26d0(int param_1)

{
  void *pvVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  int *piVar5;
  
  uVar4 = **(uint **)(param_1 + 4);
  pvVar1 = (void *)FUN_00596bd0();
  piVar2 = (int *)FUN_005a6340(pvVar1,uVar4);
  if (piVar2 != (int *)0x0) {
    piVar5 = piVar2;
    FUN_005aaf90();
    iVar3 = FUN_005ad870(piVar5);
    if (((iVar3 != 0) || (piVar2[0x5c] == 7)) || (piVar2[0x5c] == 8)) {
      pvVar1 = (void *)FUN_005aaf90();
      FUN_005a90c0(pvVar1,piVar2);
    }
  }
  return;
}

