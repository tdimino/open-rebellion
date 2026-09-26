
void * __thiscall FUN_005a3f00(void *this,uint param_1,int param_2)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  int *piVar4;
  void *pvVar5;
  undefined3 extraout_var;
  uint uVar6;
  int iVar7;
  void *this_00;
  int iVar8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652466;
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
  else if (param_2 == 0) {
    iVar7 = *(int *)(*(int *)((int)this + 0x5c) + 0x18);
    pvVar5 = (void *)FUN_0059ed70();
    uVar6 = FUN_005a0300(pvVar5,iVar7);
    if (uVar6 != 0xffffffff) {
      iVar7 = FUN_005aaf90();
      iVar8 = 1;
      if (*(int *)(iVar7 + 0x9ec) == 0) {
        uVar6 = uVar6 + 0x22;
      }
      else {
        uVar6 = uVar6 + 0x9c;
      }
      pvVar5 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar5,uVar6,iVar8);
    }
    (**(code **)(**(int **)((int)this + 0x5c) + 100))(0);
    pvVar5 = *(void **)((int)this + 0x5c);
    this_00 = (void *)FUN_005aaf90();
    FUN_005a8f70(this_00,iVar3,pvVar5);
  }
  else {
    bVar1 = FUN_005ca330(*(void **)((int)this + 0x5c),*(int *)(iVar3 + 0x18));
    if (CONCAT31(extraout_var,bVar1) == 0) {
      iVar7 = *(int *)(*(int *)((int)this + 0x5c) + 0x18);
      pvVar5 = (void *)FUN_0059ed70();
      uVar6 = FUN_005a0300(pvVar5,iVar7);
      if ((uVar6 != 0xffffffff) &&
         (iVar7 = (**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x80) + 0xc))(), iVar7 == 0)) {
        iVar7 = FUN_005aaf90();
        iVar8 = 1;
        if (*(int *)(iVar7 + 0x9ec) == 0) {
          uVar6 = uVar6 + 0x22;
        }
        else {
          uVar6 = uVar6 + 0x9c;
        }
        pvVar5 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar5,uVar6,iVar8);
      }
      piVar4 = *(int **)((int)this + 0x5c);
      pvVar5 = (void *)FUN_005aaf90();
      FUN_005a8f20(pvVar5,iVar3,piVar4);
    }
    else {
      piVar4 = *(int **)((int)this + 0x5c);
      pvVar5 = (void *)FUN_005aaf90();
      FUN_005a8ed0(pvVar5,iVar3,piVar4);
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

