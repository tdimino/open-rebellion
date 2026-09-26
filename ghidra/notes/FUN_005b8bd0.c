
void __cdecl FUN_005b8bd0(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int iVar3;
  int *this;
  int iVar4;
  uint uVar5;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar5 = puVar1[1];
  pvVar2 = (void *)FUN_00596bd0();
  iVar3 = FUN_005a6340(pvVar2,uVar5);
  uVar5 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  this = FUN_005a6360(pvVar2,uVar5);
  if (this != (int *)0x0) {
    FUN_005aaf90();
    iVar4 = FUN_005a9330();
    if ((iVar4 == 0) || (*puVar1 != *(uint *)(iVar4 + 0x18))) {
      uVar5 = *puVar1;
      pvVar2 = (void *)FUN_00596bd0();
      iVar4 = FUN_005a6340(pvVar2,uVar5);
      if (iVar4 != 0) {
        FUN_005aaf90();
        iVar4 = FUN_005ad870(iVar4);
        if (iVar4 != 0) {
          FUN_005b8630(this,iVar3);
        }
      }
    }
    else {
      iVar4 = FUN_005ba420(iVar4);
      if (iVar4 != 0) {
        FUN_005b8630(this,iVar3);
        return;
      }
    }
  }
  return;
}

