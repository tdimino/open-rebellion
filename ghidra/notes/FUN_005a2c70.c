
void * __thiscall FUN_005a2c70(int param_1,undefined4 param_2)

{
  int *piVar1;
  void *pvVar2;
  uint uVar3;
  int iVar4;
  int *piVar5;
  uint uVar6;
  int iVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00652296;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  piVar1 = (int *)FUN_0059ed70();
  if (*piVar1 != 0) {
    piVar1 = (int *)(param_1 + 4);
    (**(code **)(*(int *)(param_1 + 4) + 0x10))();
    pvVar2 = (void *)FUN_0059ed70();
    uVar3 = FUN_0059f610(pvVar2);
    if (uVar3 == 0xffffffff) {
      iVar4 = FUN_005c6070(piVar1);
      if (iVar4 != 0) {
        iVar7 = 1;
        if (*(int *)(iVar4 + 0x168) == 0) {
          uVar3 = 0x84;
        }
        else {
          uVar3 = 0x102;
        }
        pvVar2 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar2,uVar3,iVar7);
        for (piVar1 = (int *)(**(code **)(*piVar1 + 0x10))(); piVar1 != (int *)0x0;
            piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
          iVar7 = (**(code **)(*piVar1 + 0x10))();
          pvVar2 = (void *)FUN_005aaf90();
          piVar5 = FUN_005aded0(pvVar2,iVar7);
          (**(code **)(*piVar5 + 100))(param_2);
        }
        pvVar2 = (void *)FUN_00618b70(0x54);
        uStack_4 = 0;
        if (pvVar2 != (void *)0x0) {
          pvVar2 = FUN_005a1550(pvVar2,*(uint *)(iVar4 + 0xec));
          ExceptionList = pvStack_c;
          return pvVar2;
        }
      }
    }
    else {
      pvVar2 = (void *)FUN_005aaf90();
      piVar1 = FUN_005ade70(pvVar2,piVar1);
      if (piVar1 != (int *)0x0) {
        iVar4 = piVar1[6];
        uVar6 = uVar3;
        pvVar2 = (void *)FUN_0059ed70();
        FUN_005a03f0(pvVar2,uVar6,iVar4);
        iVar4 = piVar1[6];
        pvVar2 = (void *)FUN_0059ed70();
        uVar6 = FUN_005a0300(pvVar2,iVar4);
        if (uVar6 != 0xffffffff) {
          iVar4 = FUN_005aaf90();
          iVar7 = 1;
          if (*(int *)(iVar4 + 0x9ec) == 0) {
            uVar6 = uVar6 + 0x78;
          }
          else {
            uVar6 = uVar6 + 0xf6;
          }
          pvVar2 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar2,uVar6,iVar7);
        }
        (**(code **)(*piVar1 + 100))(param_2);
        pvVar2 = (void *)FUN_00618b70(0x60);
        uStack_4 = 1;
        if (pvVar2 != (void *)0x0) {
          pvVar2 = FUN_005a33d0(pvVar2,uVar3);
          ExceptionList = pvStack_c;
          return pvVar2;
        }
      }
    }
  }
  ExceptionList = pvStack_c;
  return (void *)0x0;
}

