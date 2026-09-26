
void * __thiscall FUN_005a1af0(void *this,uint param_1,int param_2)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  void *pvVar6;
  int iVar7;
  uint uVar8;
  undefined3 extraout_var;
  int *piVar9;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652088;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar2 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 0;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar2);
    }
  }
  local_4 = 0xffffffff;
  iVar3 = FUN_005a6340(DAT_006bc470,param_1);
  iVar7 = *(int *)(iVar3 + 0x168);
  iVar4 = FUN_005aaf90();
  if (iVar7 == *(int *)(iVar4 + 0x9ec)) {
    piVar5 = (int *)FUN_0059ed70();
    if (*piVar5 == 0) {
      pvVar6 = (void *)FUN_00618b70(0x54);
      local_4 = 1;
      if (pvVar6 != (void *)0x0) {
        pvVar6 = FUN_005a1550(pvVar6,param_1);
        ExceptionList = local_c;
        return pvVar6;
      }
    }
    else if (param_2 == 0) {
      piVar5 = (int *)FUN_0059ed70();
      if (*piVar5 == 0) {
        pvVar6 = (void *)FUN_00618b70(0x54);
        local_4 = 3;
        if (pvVar6 != (void *)0x0) {
          pvVar6 = FUN_005a1110(pvVar6,param_1);
          ExceptionList = local_c;
          return pvVar6;
        }
      }
      else {
        iVar7 = FUN_005c6070((int *)((int)this + 4));
        if (iVar3 != iVar7) {
          iVar7 = iVar3;
          FUN_005aaf90();
          iVar7 = FUN_005ae260(iVar7);
          if (iVar7 != 0) {
            iVar4 = FUN_005c6070((int *)((int)this + 4));
            iVar7 = iVar4;
            pvVar6 = (void *)FUN_0059ed70();
            uVar8 = FUN_005a0c50(pvVar6,iVar7);
            if (uVar8 == 0) {
              pvVar6 = (void *)FUN_005aaf90();
              piVar5 = FUN_005aded0(pvVar6,iVar4);
            }
            else {
              if (DAT_006bc470 == (undefined4 *)0x0) {
                puVar2 = (undefined4 *)FUN_00618b70(0x48);
                local_4 = 4;
                if (puVar2 == (undefined4 *)0x0) {
                  DAT_006bc470 = (undefined4 *)0x0;
                }
                else {
                  DAT_006bc470 = FUN_005a6290(puVar2);
                }
                local_4 = 0xffffffff;
              }
              piVar5 = (int *)FUN_005a6460(DAT_006bc470,uVar8);
            }
            iVar7 = FUN_005aaf90();
            iVar4 = 1;
            if (*(int *)(iVar7 + 0x9ec) == 0) {
              uVar8 = 0x77;
            }
            else {
              uVar8 = 0xf5;
            }
            pvVar6 = (void *)FUN_005ba8b0();
            FUN_005ba980(pvVar6,uVar8,iVar4);
            piVar9 = piVar5;
            pvVar6 = (void *)FUN_005aaf90();
            FUN_005a8fc0(pvVar6,iVar3,piVar9);
            (**(code **)(*piVar5 + 100))(1);
          }
        }
      }
    }
    else {
      iVar7 = FUN_005c6070((int *)((int)this + 4));
      if (iVar3 != iVar7) {
        pvVar6 = (void *)FUN_00618b70(0x54);
        local_4 = 2;
        if (pvVar6 != (void *)0x0) {
          pvVar6 = FUN_005a2280(pvVar6,(int *)((int)this + 4),param_1);
          ExceptionList = local_c;
          return pvVar6;
        }
      }
    }
  }
  else {
    piVar5 = (int *)FUN_0059ed70();
    if (*piVar5 == 0) {
      pvVar6 = (void *)FUN_00618b70(0x54);
      local_4 = 5;
      if (pvVar6 != (void *)0x0) {
        pvVar6 = FUN_005a1110(pvVar6,*(uint *)(iVar3 + 0xec));
        ExceptionList = local_c;
        return pvVar6;
      }
    }
    else {
      iVar7 = iVar3;
      FUN_005aaf90();
      iVar7 = FUN_005ae2a0(iVar7);
      if (iVar7 == 0) {
        pvVar6 = (void *)FUN_00618b70(0x54);
        local_4 = 6;
        if (pvVar6 != (void *)0x0) {
          pvVar6 = FUN_005a1110(pvVar6,*(uint *)(iVar3 + 0xec));
          ExceptionList = local_c;
          return pvVar6;
        }
      }
      else {
        iVar4 = FUN_005c6070((int *)((int)this + 4));
        iVar7 = iVar4;
        pvVar6 = (void *)FUN_0059ed70();
        uVar8 = FUN_005a0c50(pvVar6,iVar7);
        if (uVar8 == 0) {
          pvVar6 = (void *)FUN_005aaf90();
          piVar5 = FUN_005aded0(pvVar6,iVar4);
        }
        else {
          if (DAT_006bc470 == (undefined4 *)0x0) {
            puVar2 = (undefined4 *)FUN_00618b70(0x48);
            local_4 = 7;
            if (puVar2 == (undefined4 *)0x0) {
              DAT_006bc470 = (undefined4 *)0x0;
            }
            else {
              DAT_006bc470 = FUN_005a6290(puVar2);
            }
            local_4 = 0xffffffff;
          }
          piVar5 = (int *)FUN_005a6460(DAT_006bc470,uVar8);
        }
        if (param_2 == 0) {
          (**(code **)(*piVar5 + 100))(0);
          piVar9 = piVar5;
          pvVar6 = (void *)FUN_005aaf90();
          FUN_005a8fc0(pvVar6,iVar3,piVar9);
          iVar7 = (**(code **)(piVar5[0x20] + 0xc))();
          if (iVar7 == 0) {
            iVar7 = FUN_005aaf90();
            iVar3 = 1;
            if (*(int *)(iVar7 + 0x9ec) == 0) {
              uVar8 = 0x3b;
            }
            else {
              uVar8 = 0xb5;
            }
            pvVar6 = (void *)FUN_005ba8b0();
            FUN_005ba980(pvVar6,uVar8,iVar3);
          }
        }
        else {
          bVar1 = FUN_005ca310(piVar5,param_1);
          if (CONCAT31(extraout_var,bVar1) == 0) {
            iVar7 = (**(code **)(piVar5[0x26] + 0x10))();
            if ((iVar7 == 0) && (iVar7 = (**(code **)(piVar5[0x20] + 0xc))(), iVar7 == 0)) {
              iVar7 = FUN_005aaf90();
              iVar4 = 1;
              if (*(int *)(iVar7 + 0x9ec) == 0) {
                uVar8 = 0x3b;
              }
              else {
                uVar8 = 0xb5;
              }
              pvVar6 = (void *)FUN_005ba8b0();
              FUN_005ba980(pvVar6,uVar8,iVar4);
            }
            if (piVar5[0x1a] == 1) {
              (**(code **)(*piVar5 + 100))(0);
              iVar7 = iVar3;
              pvVar6 = (void *)FUN_005aaf90();
              FUN_005a8fc0(pvVar6,iVar7,piVar5);
              FUN_005c5f20((void *)((int)this + 0x1c),iVar3);
            }
            else {
              iVar7 = iVar3;
              pvVar6 = (void *)FUN_005aaf90();
              FUN_005a8e30(pvVar6,iVar7,piVar5);
              FUN_005c5f20((void *)((int)this + 0x1c),iVar3);
            }
          }
          else {
            iVar7 = iVar3;
            pvVar6 = (void *)FUN_005aaf90();
            FUN_005a8e80(pvVar6,iVar7,piVar5);
            FUN_005c5fb0((void *)((int)this + 0x1c),iVar3);
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

