
void __cdecl FUN_005ba520(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar4 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  iVar3 = FUN_005a6340(pvVar2,uVar4);
  if (iVar3 != 0) {
    FUN_005aaf90();
    iVar3 = FUN_005ad870(iVar3);
    if (iVar3 != 0) {
      iVar3 = FUN_005aaf90();
      iVar5 = 1;
      uVar4 = 0x14;
      *(undefined4 *)(iVar3 + 0x894) = 1;
      pvVar2 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar2,uVar4,iVar5);
      iVar3 = FUN_005aaf90();
      if (*(int *)(iVar3 + 0x9ec) == 0) {
        iVar3 = 1;
        uVar4 = 0x91;
        pvVar2 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar2,uVar4,iVar3);
      }
    }
  }
  FUN_005c12d0(puVar1[1],*puVar1,0,0x3f800000,1);
  return;
}

