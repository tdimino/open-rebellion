
void * __thiscall FUN_005a4bd0(void *this,uint param_1,int param_2)

{
  bool bVar1;
  int *piVar2;
  void *pvVar3;
  int iVar4;
  int iVar5;
  undefined3 extraout_var;
  int iVar6;
  void *pvVar7;
  undefined3 extraout_var_00;
  uint uVar8;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006525f6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_0059ed70();
  if (*piVar2 == 0) {
    pvVar3 = (void *)FUN_00618b70(0x54);
    local_4 = 0;
    if (pvVar3 != (void *)0x0) {
      pvVar3 = FUN_005a1550(pvVar3,param_1);
      ExceptionList = local_c;
      return pvVar3;
    }
  }
  else {
    local_10 = this;
    if (DAT_006bc470 == (undefined4 *)0x0) {
      local_10 = (undefined4 *)FUN_00618b70(0x48);
      local_4 = 1;
      if (local_10 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(local_10);
      }
      local_4 = 0xffffffff;
    }
    iVar4 = FUN_005a6340(DAT_006bc470,param_1);
    iVar6 = *(int *)(iVar4 + 0x168);
    iVar5 = FUN_005aaf90();
    if (iVar6 == *(int *)(iVar5 + 0x9ec)) {
      if ((param_2 != 0) &&
         (bVar1 = FUN_005ca310(*(void **)((int)this + 0x5c),*(int *)(iVar4 + 0xec)),
         CONCAT31(extraout_var,bVar1) == 1)) {
        piVar2 = *(int **)((int)this + 0x5c);
        pvVar3 = (void *)FUN_005aaf90();
        FUN_005a8e80(pvVar3,iVar4,piVar2);
        (**(code **)(**(int **)((int)this + 0x5c) + 100))(0);
        ExceptionList = local_c;
        return (void *)0x0;
      }
      iVar6 = iVar4;
      FUN_005aaf90();
      iVar6 = FUN_005ae260(iVar6);
      if (iVar6 != 0) {
        pvVar3 = *(void **)((int)this + 0x5c);
        pvVar7 = (void *)FUN_005aaf90();
        FUN_005a8fc0(pvVar7,iVar4,pvVar3);
        pvVar3 = (void *)(**(code **)(*(int *)this + 0x20))(1);
        ExceptionList = local_10;
        return pvVar3;
      }
    }
    else if (param_2 == 0) {
      uVar8 = *(uint *)(*(int *)((int)this + 0x5c) + 0x18);
      pvVar3 = (void *)FUN_0059ed70();
      uVar8 = FUN_005a02c0(pvVar3,uVar8);
      iVar6 = (**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x80) + 0xc))();
      if ((iVar6 == 0) && (uVar8 != 0xffffffff)) {
        iVar6 = FUN_005aaf90();
        iVar5 = 1;
        if (*(int *)(iVar6 + 0x9ec) == 0) {
          uVar8 = uVar8 + 0x44;
        }
        else {
          uVar8 = uVar8 + 0xbe;
        }
        pvVar3 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar3,uVar8,iVar5);
      }
      (**(code **)(**(int **)((int)this + 0x5c) + 100))(0);
      pvVar3 = *(void **)((int)this + 0x5c);
      pvVar7 = (void *)FUN_005aaf90();
      FUN_005a8fc0(pvVar7,iVar4,pvVar3);
    }
    else {
      bVar1 = FUN_005ca310(*(void **)((int)this + 0x5c),*(int *)(iVar4 + 0xec));
      if (CONCAT31(extraout_var_00,bVar1) == 0) {
        uVar8 = *(uint *)(*(int *)((int)this + 0x5c) + 0x18);
        pvVar3 = (void *)FUN_0059ed70();
        uVar8 = FUN_005a02c0(pvVar3,uVar8);
        iVar6 = (**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x98) + 0x10))();
        if (((iVar6 == 0) &&
            (iVar6 = (**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x80) + 0xc))(), iVar6 == 0)
            ) && (uVar8 != 0xffffffff)) {
          iVar6 = FUN_005aaf90();
          iVar5 = 1;
          if (*(int *)(iVar6 + 0x9ec) == 0) {
            uVar8 = uVar8 + 0x44;
          }
          else {
            uVar8 = uVar8 + 0xbe;
          }
          pvVar3 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar3,uVar8,iVar5);
        }
        piVar2 = *(int **)((int)this + 0x5c);
        pvVar3 = (void *)FUN_005aaf90();
        FUN_005a8e30(pvVar3,iVar4,piVar2);
      }
      else {
        piVar2 = *(int **)((int)this + 0x5c);
        pvVar3 = (void *)FUN_005aaf90();
        FUN_005a8e80(pvVar3,iVar4,piVar2);
      }
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

