
void __fastcall FUN_005d0340(int param_1)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  int iVar4;
  
  *(undefined4 *)(param_1 + 0x2c) = 2;
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 0x80) = 0;
  iVar1 = FUN_005aaf90();
  if (*(int *)(iVar1 + 0x9ec) == 0) {
    iVar1 = (**(code **)(**(int **)(param_1 + 8) + 4))();
    if (iVar1 == 4) {
      iVar1 = FUN_005c7150(*(int *)(param_1 + 8));
      if (iVar1 == 0) {
        iVar1 = 1;
        uVar3 = 0x139;
        pvVar2 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar2,uVar3,iVar1);
        return;
      }
      if (iVar1 == 1) {
        iVar1 = 1;
        uVar3 = 0x12d;
        pvVar2 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar2,uVar3,iVar1);
        return;
      }
      iVar4 = 1;
      uVar3 = (-(uint)(iVar1 != 2) & 4) + 0x131;
      pvVar2 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar2,uVar3,iVar4);
      return;
    }
  }
  else {
    iVar1 = 1;
    uVar3 = 0x115;
    pvVar2 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar2,uVar3,iVar1);
  }
  return;
}

