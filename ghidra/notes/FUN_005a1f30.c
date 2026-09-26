
void * __thiscall FUN_005a1f30(void *this,uint param_1,int param_2)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  int *piVar4;
  void *pvVar5;
  int iVar6;
  uint uVar7;
  undefined3 extraout_var;
  int iVar8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006520c1;
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
  iVar3 = FUN_005a6450(DAT_006bc470,param_1);
  piVar4 = (int *)FUN_0059ed70();
  if (*piVar4 == 0) {
    pvVar5 = (void *)FUN_00618b70(0x54);
    local_4 = 1;
    if (pvVar5 != (void *)0x0) {
      pvVar5 = FUN_005a0e70(pvVar5,param_1);
      ExceptionList = local_c;
      return pvVar5;
    }
  }
  else {
    iVar6 = FUN_005c6070((int *)((int)this + 4));
    iVar8 = iVar6;
    pvVar5 = (void *)FUN_0059ed70();
    uVar7 = FUN_005a0c50(pvVar5,iVar8);
    if (uVar7 == 0) {
      pvVar5 = (void *)FUN_005aaf90();
      piVar4 = FUN_005aded0(pvVar5,iVar6);
    }
    else {
      if (DAT_006bc470 == (undefined4 *)0x0) {
        puVar2 = (undefined4 *)FUN_00618b70(0x48);
        local_4 = 2;
        if (puVar2 == (undefined4 *)0x0) {
          DAT_006bc470 = (undefined4 *)0x0;
        }
        else {
          DAT_006bc470 = FUN_005a6290(puVar2);
        }
        local_4 = 0xffffffff;
      }
      piVar4 = (int *)FUN_005a6460(DAT_006bc470,uVar7);
    }
    if (param_2 == 0) {
      (**(code **)(*piVar4 + 100))(0);
      pvVar5 = (void *)FUN_005aaf90();
      FUN_005a8f70(pvVar5,iVar3,piVar4);
      iVar3 = FUN_005aaf90();
      iVar8 = 1;
      if (*(int *)(iVar3 + 0x9ec) == 0) {
        uVar7 = 0x21;
      }
      else {
        uVar7 = 0x9b;
      }
      pvVar5 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar5,uVar7,iVar8);
    }
    else {
      bVar1 = FUN_005ca330(piVar4,param_1);
      if (CONCAT31(extraout_var,bVar1) == 0) {
        iVar8 = (**(code **)(piVar4[0x20] + 0xc))();
        if (iVar8 == 0) {
          iVar8 = FUN_005aaf90();
          iVar6 = 1;
          if (*(int *)(iVar8 + 0x9ec) == 0) {
            uVar7 = 0x21;
          }
          else {
            uVar7 = 0x9b;
          }
          pvVar5 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar5,uVar7,iVar6);
        }
        pvVar5 = (void *)FUN_005aaf90();
        FUN_005a8f20(pvVar5,iVar3,piVar4);
      }
      else {
        pvVar5 = (void *)FUN_005aaf90();
        FUN_005a8ed0(pvVar5,iVar3,piVar4);
      }
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

