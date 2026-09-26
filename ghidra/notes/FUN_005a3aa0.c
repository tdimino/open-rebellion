
undefined4 * __thiscall FUN_005a3aa0(void *this,uint param_1,int param_2)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  void *pvVar6;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar7;
  undefined3 extraout_var_01;
  uint uVar8;
  void *pvVar9;
  undefined3 extraout_var_02;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065243d;
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
        puVar2 = FUN_005a1550(pvVar6,param_1);
        ExceptionList = local_c;
        return puVar2;
      }
    }
    else if (param_2 == 0) {
      bVar1 = FUN_005ca2f0(*(void **)((int)this + 0x5c),param_1);
      if (CONCAT31(extraout_var_01,bVar1) == 1) {
        pvVar6 = (void *)FUN_0059ed70();
        uVar8 = FUN_0059f610(pvVar6);
        if (uVar8 != 0xffffffff) {
          (**(code **)(*(int *)((int)this + 0x34) + 4))();
          (**(code **)(*(int *)((int)this + 0x1c) + 4))();
          FUN_005c5f20((int *)((int)this + 0x1c),iVar3);
          piVar5 = (int *)((int)this + 4);
          FUN_005c6090(piVar5,(int *)(*(int *)((int)this + 0x5c) + 0x44));
          FUN_005c60d0(piVar5,param_1);
          iVar7 = (**(code **)(*piVar5 + 0x10))();
          if (iVar7 != 0) {
            pvVar6 = (void *)FUN_00618b70(0x60);
            local_4 = 3;
            if (pvVar6 == (void *)0x0) {
              ExceptionList = local_c;
              return (undefined4 *)0x0;
            }
            puVar2 = FUN_005a34e0(pvVar6,piVar5,(int *)((int)this + 0x34),(int *)((int)this + 0x1c),
                                  uVar8);
            ExceptionList = local_c;
            return puVar2;
          }
        }
        pvVar6 = (void *)FUN_00618b70(0x54);
        local_4 = 4;
        if (pvVar6 != (void *)0x0) {
          puVar2 = FUN_005a1550(pvVar6,param_1);
          ExceptionList = local_c;
          return puVar2;
        }
      }
      else {
        iVar7 = iVar3;
        FUN_005aaf90();
        iVar7 = FUN_005ae260(iVar7);
        if (iVar7 != 0) {
          pvVar6 = *(void **)((int)this + 0x5c);
          pvVar9 = (void *)FUN_005aaf90();
          FUN_005a8fc0(pvVar9,iVar3,pvVar6);
          puVar2 = (undefined4 *)(**(code **)(*(int *)this + 0x20))(1);
          ExceptionList = local_c;
          return puVar2;
        }
      }
    }
    else {
      bVar1 = FUN_005ca310(*(void **)((int)this + 0x5c),*(int *)(iVar3 + 0xec));
      if (CONCAT31(extraout_var,bVar1) == 1) {
        piVar5 = *(int **)((int)this + 0x5c);
        pvVar6 = (void *)FUN_005aaf90();
        FUN_005a8e80(pvVar6,iVar3,piVar5);
        (**(code **)(**(int **)((int)this + 0x5c) + 100))(0);
      }
      else {
        bVar1 = FUN_005ca2f0(*(void **)((int)this + 0x5c),*(uint *)(iVar3 + 0xec));
        if (CONCAT31(extraout_var_00,bVar1) == 0) {
          piVar5 = *(int **)((int)this + 0x5c);
          pvVar6 = (void *)FUN_005aaf90();
          FUN_005ade50(pvVar6,iVar3,piVar5);
        }
        else {
          piVar5 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x44) + 0x10))();
          iVar7 = (**(code **)(*piVar5 + 8))();
          if (iVar7 == 0) {
            piVar5 = *(int **)((int)this + 0x5c);
            pvVar6 = (void *)FUN_005aaf90();
            FUN_005a8d30(pvVar6,iVar3,piVar5);
            iVar7 = *(int *)((int)this + 0x58);
            *(undefined4 *)((int)this + 0x5c) = 0;
            pvVar6 = (void *)FUN_0059ed70();
            FUN_005a0410(pvVar6,iVar7);
            puVar2 = (undefined4 *)FUN_00618b70(0x54);
            local_4 = 2;
            if (puVar2 != (undefined4 *)0x0) {
              puVar2 = FUN_005a0c80(puVar2);
              ExceptionList = local_c;
              return puVar2;
            }
          }
          else {
            piVar5 = *(int **)((int)this + 0x5c);
            pvVar6 = (void *)FUN_005aaf90();
            FUN_005a8d30(pvVar6,iVar3,piVar5);
          }
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
        puVar2 = FUN_005a1110(pvVar6,param_1);
        ExceptionList = local_c;
        return puVar2;
      }
    }
    else {
      if (param_2 == 0) {
        iVar7 = *(int *)(*(int *)((int)this + 0x5c) + 0x18);
        pvVar6 = (void *)FUN_0059ed70();
        uVar8 = FUN_005a0300(pvVar6,iVar7);
        if ((uVar8 != 0xffffffff) &&
           (iVar7 = (**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x80) + 0xc))(), iVar7 == 0))
        {
          iVar7 = FUN_005aaf90();
          iVar4 = 1;
          if (*(int *)(iVar7 + 0x9ec) == 0) {
            uVar8 = uVar8 + 0x3c;
          }
          else {
            uVar8 = uVar8 + 0xb6;
          }
          pvVar6 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar6,uVar8,iVar4);
        }
        piVar5 = *(int **)((int)this + 0x5c);
      }
      else {
        bVar1 = FUN_005ca310(*(void **)((int)this + 0x5c),*(int *)(iVar3 + 0xec));
        if (CONCAT31(extraout_var_02,bVar1) != 0) {
          piVar5 = *(int **)((int)this + 0x5c);
          pvVar6 = (void *)FUN_005aaf90();
          FUN_005a8e80(pvVar6,iVar3,piVar5);
          ExceptionList = local_c;
          return (undefined4 *)0x0;
        }
        iVar7 = iVar3;
        FUN_005aaf90();
        iVar7 = FUN_005ae2a0(iVar7);
        if (iVar7 == 0) {
          pvVar6 = (void *)FUN_00618b70(0x54);
          local_4 = 6;
          if (pvVar6 == (void *)0x0) {
            ExceptionList = local_c;
            return (undefined4 *)0x0;
          }
          puVar2 = FUN_005a1110(pvVar6,*(uint *)(iVar3 + 0xec));
          ExceptionList = local_c;
          return puVar2;
        }
        iVar7 = *(int *)(*(int *)((int)this + 0x5c) + 0x18);
        pvVar6 = (void *)FUN_0059ed70();
        uVar8 = FUN_005a0300(pvVar6,iVar7);
        if (((uVar8 != 0xffffffff) &&
            (iVar7 = (**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x80) + 0xc))(), iVar7 == 0)
            ) && (iVar7 = (**(code **)(*(int *)(*(int *)((int)this + 0x5c) + 0x98) + 0x10))(),
                 iVar7 == 0)) {
          iVar7 = FUN_005aaf90();
          iVar4 = 1;
          if (*(int *)(iVar7 + 0x9ec) == 0) {
            uVar8 = uVar8 + 0x3c;
          }
          else {
            uVar8 = uVar8 + 0xb6;
          }
          pvVar6 = (void *)FUN_005ba8b0();
          FUN_005ba980(pvVar6,uVar8,iVar4);
        }
        piVar5 = *(int **)((int)this + 0x5c);
        if (piVar5[0x1a] != 1) {
          pvVar6 = (void *)FUN_005aaf90();
          FUN_005a8e30(pvVar6,iVar3,piVar5);
          ExceptionList = local_c;
          return (undefined4 *)0x0;
        }
      }
      (**(code **)(*piVar5 + 100))(0);
      pvVar6 = *(void **)((int)this + 0x5c);
      pvVar9 = (void *)FUN_005aaf90();
      FUN_005a8fc0(pvVar9,iVar3,pvVar6);
    }
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

