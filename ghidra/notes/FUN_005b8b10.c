
void __cdecl FUN_005b8b10(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int *piVar3;
  int *this;
  int iVar4;
  uint uVar5;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar5 = puVar1[1];
  pvVar2 = (void *)FUN_00596bd0();
  piVar3 = (int *)FUN_005a6340(pvVar2,uVar5);
  uVar5 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  this = FUN_005a6360(pvVar2,uVar5);
  if ((piVar3 != (int *)0x0) && (this != (int *)0x0)) {
    uVar5 = piVar3[0x5d];
    pvVar2 = (void *)FUN_00596bd0();
    iVar4 = FUN_005a6480(pvVar2,uVar5);
    if ((iVar4 == 0) || (*(int *)(iVar4 + 100) != 2)) {
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
            FUN_005b8c70(this,piVar3);
          }
        }
      }
      else {
        iVar4 = FUN_005ba420(iVar4);
        if (iVar4 != 0) {
          FUN_005b8c70(this,piVar3);
          return;
        }
      }
    }
  }
  return;
}

