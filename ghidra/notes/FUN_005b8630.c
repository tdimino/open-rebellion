
void __thiscall FUN_005b8630(void *this,int param_1)

{
  void *pvVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  
  FUN_005c5f20(*(void **)((int)this + 0x20),param_1);
  FUN_005c60d0(*(void **)((int)this + 0x28),*(uint *)(param_1 + 0xec));
  *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
  uVar6 = *(uint *)(param_1 + 0x174);
  iVar5 = 4;
  *(undefined4 *)(param_1 + 0x35c) = 4;
  pvVar1 = (void *)FUN_00596bd0();
  piVar2 = (int *)FUN_005a6480(pvVar1,uVar6);
  uVar6 = *(uint *)(param_1 + 0x174);
  if (piVar2 != (int *)0x0) {
    iVar5 = FUN_005c7150((int)piVar2);
    (**(code **)(*piVar2 + 0x54))(*(undefined4 *)(param_1 + 0xec));
    FUN_005caef0(piVar2,*(undefined4 *)(param_1 + 0xec));
  }
  iVar4 = param_1;
  pvVar1 = (void *)FUN_005aaf90();
  FUN_005a9070(pvVar1,iVar4);
  iVar4 = *(int *)(param_1 + 0x168);
  *(undefined4 *)(param_1 + 0x170) = 5;
  iVar3 = FUN_005aaf90();
  if ((iVar4 == *(int *)(iVar3 + 0x9ec)) && (uVar6 != 0)) {
    pvVar1 = (void *)FUN_00596bd0();
    iVar4 = FUN_005a6480(pvVar1,uVar6);
    if (iVar4 == 0) {
      if (*(int *)(param_1 + 0x168) == 0) {
        if (iVar5 == 0) {
          iVar5 = 1;
          uVar6 = 0x87;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
          return;
        }
        if (iVar5 == 1) {
          uVar6 = 0x88;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
          return;
        }
        if (iVar5 == 2) {
          iVar5 = 1;
          uVar6 = 0x89;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
          return;
        }
        if (iVar5 == 3) {
          iVar5 = 1;
          uVar6 = 0x8a;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
          return;
        }
      }
      else {
        if (iVar5 == 0) {
          iVar5 = 1;
          uVar6 = 0x105;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
          return;
        }
        if (iVar5 == 1) {
          uVar6 = 0x106;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
          return;
        }
        if (iVar5 == 2) {
          iVar5 = 1;
          uVar6 = 0x107;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
          return;
        }
        if (iVar5 == 3) {
          iVar5 = 1;
          uVar6 = 0x108;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar5);
        }
      }
    }
  }
  return;
}

