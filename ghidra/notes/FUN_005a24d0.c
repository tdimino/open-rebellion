
void * __thiscall FUN_005a24d0(void *this,uint param_1,int param_2)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  void *pvVar5;
  int iVar6;
  uint uVar7;
  undefined4 uVar8;
  int *piVar9;
  uint uVar10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006521f3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar1);
    }
  }
  local_4 = 0xffffffff;
  iVar2 = FUN_005a6340(DAT_006bc470,param_1);
  iVar6 = *(int *)(iVar2 + 0x168);
  iVar3 = FUN_005aaf90();
  if (iVar6 == *(int *)(iVar3 + 0x9ec)) {
    piVar4 = (int *)FUN_0059ed70();
    if (*piVar4 == 0) {
      pvVar5 = (void *)FUN_00618b70(0x54);
      local_4 = 1;
      if (pvVar5 != (void *)0x0) {
        pvVar5 = FUN_005a1550(pvVar5,param_1);
        ExceptionList = local_c;
        return pvVar5;
      }
    }
    else if (param_2 == 0) {
      iVar6 = iVar2;
      FUN_005aaf90();
      iVar6 = FUN_005ae260(iVar6);
      if (iVar6 != 0) {
        pvVar5 = (void *)FUN_0059ed70();
        uVar7 = FUN_0059f610(pvVar5);
        if (uVar7 != 0xffffffff) {
          piVar4 = (int *)((int)this + 4);
          iVar6 = FUN_005ca4b0(piVar4,(uint)(*(int *)(iVar2 + 0xec) != 0));
          if (iVar6 != 0) {
            FUN_005c60d0(piVar4,*(uint *)(iVar2 + 0xec));
          }
          FUN_005c5f20((int *)((int)this + 0x1c),iVar2);
          iVar6 = (**(code **)(*piVar4 + 0x10))();
          if (iVar6 != 0) {
            pvVar5 = (void *)FUN_00618b70(0x60);
            local_4 = 3;
            if (pvVar5 == (void *)0x0) {
              ExceptionList = local_c;
              return (void *)0x0;
            }
            pvVar5 = FUN_005a34e0(pvVar5,piVar4,(int *)((int)this + 0x34),(int *)((int)this + 0x1c),
                                  uVar7);
            ExceptionList = local_c;
            return pvVar5;
          }
        }
        pvVar5 = (void *)FUN_00618b70(0x54);
        local_4 = 4;
        if (pvVar5 != (void *)0x0) {
          pvVar5 = FUN_005a1550(pvVar5,param_1);
          ExceptionList = local_c;
          return pvVar5;
        }
      }
    }
    else {
      piVar4 = (int *)((int)this + 4);
      iVar6 = FUN_005ca4b0(piVar4,param_1);
      if (iVar6 == 0) {
        FUN_005c5f20(piVar4,iVar2);
        pvVar5 = (void *)FUN_0059ed70();
        FUN_0051c9b0(pvVar5,iVar2);
      }
      else {
        FUN_005c60d0(piVar4,param_1);
        iVar6 = FUN_005c5eb0(piVar4);
        if (iVar6 == 1) {
          pvVar5 = (void *)FUN_00618b70(0x54);
          local_4 = 2;
          if (pvVar5 != (void *)0x0) {
            piVar4 = (int *)(**(code **)(*piVar4 + 0x10))();
            uVar7 = (**(code **)(*piVar4 + 4))();
            pvVar5 = FUN_005a1550(pvVar5,uVar7);
            ExceptionList = local_c;
            return pvVar5;
          }
        }
        else {
          piVar4 = (int *)(**(code **)(*piVar4 + 0x10))();
          uVar8 = (**(code **)(*piVar4 + 0x10))();
          pvVar5 = (void *)FUN_0059ed70();
          FUN_0051c9b0(pvVar5,uVar8);
        }
      }
    }
  }
  else {
    piVar4 = (int *)FUN_0059ed70();
    if (*piVar4 == 0) {
      pvVar5 = (void *)FUN_00618b70(0x54);
      local_4 = 5;
      if (pvVar5 != (void *)0x0) {
        pvVar5 = FUN_005a1110(pvVar5,param_1);
        ExceptionList = local_c;
        return pvVar5;
      }
    }
    else {
      iVar6 = iVar2;
      FUN_005aaf90();
      iVar6 = FUN_005ae2a0(iVar6);
      if (iVar6 == 0) {
        pvVar5 = (void *)FUN_00618b70(0x54);
        local_4 = 6;
        if (pvVar5 != (void *)0x0) {
          pvVar5 = FUN_005a1110(pvVar5,*(uint *)(iVar2 + 0xec));
          ExceptionList = local_c;
          return pvVar5;
        }
      }
      else {
        pvVar5 = (void *)FUN_0059ed70();
        uVar7 = FUN_0059f610(pvVar5);
        if (uVar7 == 0xffffffff) {
          iVar6 = FUN_005c6070((int *)((int)this + 4));
          if (iVar6 != 0) {
            iVar3 = 1;
            if (*(int *)(iVar6 + 0x168) == 0) {
              uVar7 = 0x84;
            }
            else {
              uVar7 = 0x102;
            }
            pvVar5 = (void *)FUN_005ba8b0();
            FUN_005ba980(pvVar5,uVar7,iVar3);
            for (piVar4 = (int *)(**(code **)(*(int *)((int)this + 4) + 0x10))();
                piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
              iVar3 = (**(code **)(*piVar4 + 0x10))();
              pvVar5 = (void *)FUN_005aaf90();
              piVar9 = (int *)FUN_005aded0(pvVar5,iVar3);
              iVar3 = iVar2;
              pvVar5 = (void *)FUN_005aaf90();
              FUN_005a8e30(pvVar5,iVar3,piVar9);
            }
            if (iVar6 != 0) {
              pvVar5 = (void *)FUN_00618b70(0x54);
              local_4 = 8;
              if (pvVar5 != (void *)0x0) {
                pvVar5 = FUN_005a1550(pvVar5,*(uint *)(iVar6 + 0xec));
                ExceptionList = local_c;
                return pvVar5;
              }
            }
          }
        }
        else {
          iVar6 = FUN_005aaf90();
          iVar3 = 1;
          if (*(int *)(iVar6 + 0x9ec) == 0) {
            uVar10 = uVar7 + 0x3c;
          }
          else {
            uVar10 = uVar7 + 0xb6;
          }
          pvVar5 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar5,uVar10,iVar3);
          FUN_005c5f20((int *)((int)this + 0x1c),iVar2);
          pvVar5 = (void *)FUN_00618b70(0x60);
          local_4 = 7;
          if (pvVar5 != (void *)0x0) {
            pvVar5 = FUN_005a34e0(pvVar5,(int *)((int)this + 4),(int *)((int)this + 0x34),
                                  (int *)((int)this + 0x1c),uVar7);
            ExceptionList = local_c;
            return pvVar5;
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

