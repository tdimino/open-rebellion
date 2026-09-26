
void __thiscall FUN_005a5cf0(void *this,uint param_1,uint param_2)

{
  void *pvVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  
  uVar6 = param_1;
  pvVar1 = (void *)FUN_00596bd0();
  piVar2 = (int *)FUN_005a6340(pvVar1,uVar6);
  pvVar1 = (void *)FUN_00596bd0();
  piVar3 = (int *)FUN_005a6340(pvVar1,param_2);
  if ((piVar2 != (int *)0x0) && (piVar3 != (int *)0x0)) {
    iVar5 = piVar3[0x5a];
    iVar4 = FUN_005aaf90();
    if (iVar5 == *(int *)(iVar4 + 0x9ec)) {
      uVar6 = piVar3[0x5d];
      if (uVar6 != 0) {
        iVar5 = (**(code **)(*piVar3 + 0x20))();
        if (iVar5 != 0) {
          pvVar1 = (void *)FUN_0059ed70();
          uVar6 = FUN_005a02c0(pvVar1,uVar6);
          if (uVar6 == 0xffffffff) {
            return;
          }
          iVar5 = FUN_005aaf90();
          iVar4 = 1;
          if (*(int *)(iVar5 + 0x9ec) == 0) {
            uVar6 = uVar6 + 0x5e;
            pvVar1 = (void *)FUN_005ba8b0();
            FUN_005ba980(pvVar1,uVar6,iVar4);
            return;
          }
          uVar6 = uVar6 + 0xd8;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar4);
          return;
        }
        uVar6 = FUN_005a0300(this,uVar6);
        if (uVar6 != 0xffffffff) {
          iVar5 = FUN_005aaf90();
          iVar4 = 1;
          if (*(int *)(iVar5 + 0x9ec) == 0) {
            uVar6 = uVar6 + 0x56;
            pvVar1 = (void *)FUN_005ba8b0();
            FUN_005ba980(pvVar1,uVar6,iVar4);
            return;
          }
          uVar6 = uVar6 + 0xd0;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar4);
          return;
        }
      }
      iVar5 = FUN_005aaf90();
      iVar4 = 1;
      if (*(int *)(iVar5 + 0x9ec) == 0) {
        uVar6 = 0x55;
        pvVar1 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar1,uVar6,iVar4);
        return;
      }
      uVar6 = 0xcf;
      pvVar1 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar1,uVar6,iVar4);
      return;
    }
    uVar6 = piVar2[0x5d];
    if (uVar6 != 0) {
      iVar5 = (**(code **)(*piVar2 + 0x20))();
      if (iVar5 != 0) {
        pvVar1 = (void *)FUN_0059ed70();
        uVar6 = FUN_005a02c0(pvVar1,uVar6);
        if (uVar6 == 0xffffffff) {
          return;
        }
        iVar5 = FUN_005aaf90();
        iVar4 = 1;
        if (*(int *)(iVar5 + 0x9ec) == 0) {
          uVar6 = uVar6 + 0x6b;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar4);
          return;
        }
        uVar6 = uVar6 + 0xe5;
        pvVar1 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar1,uVar6,iVar4);
        return;
      }
      iVar5 = FUN_005aaf90();
      for (iVar5 = (**(code **)(*(int *)(iVar5 + 0xe50) + 0xc))(); iVar5 != 0;
          iVar5 = *(int *)(iVar5 + 0x10)) {
        if (*(uint *)(iVar5 + 0x5c) == param_1) {
          iVar4 = FUN_005aaf90();
          iVar8 = 1;
          if (*(int *)(iVar4 + 0x9ec) == 0) {
            uVar7 = 0x86;
          }
          else {
            uVar7 = 0x104;
          }
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar7,iVar8);
        }
      }
      pvVar1 = (void *)FUN_0059ed70();
      uVar6 = FUN_005a0300(pvVar1,uVar6);
      if (uVar6 != 0xffffffff) {
        iVar5 = FUN_005aaf90();
        iVar4 = 1;
        if (*(int *)(iVar5 + 0x9ec) == 0) {
          uVar6 = uVar6 + 99;
          pvVar1 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar1,uVar6,iVar4);
          return;
        }
        uVar6 = uVar6 + 0xdd;
        pvVar1 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar1,uVar6,iVar4);
        return;
      }
    }
    iVar5 = FUN_005aaf90();
    iVar4 = 1;
    if (*(int *)(iVar5 + 0x9ec) == 0) {
      uVar6 = 0x62;
      pvVar1 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar1,uVar6,iVar4);
      return;
    }
    uVar6 = 0xdc;
    pvVar1 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar1,uVar6,iVar4);
  }
  return;
}

