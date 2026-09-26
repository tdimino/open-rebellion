
void __thiscall FUN_004a6be0(void *this,int *param_1)

{
  bool bVar1;
  uint *puVar2;
  int *piVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar3 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637e99;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = FUN_004025b0(param_1,(uint *)&param_1);
  bVar1 = false;
  local_4 = 0;
  if ((7 < *puVar2 >> 0x18) && (*puVar2 >> 0x18 < 0x10)) {
    bVar1 = true;
  }
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    FUN_00502db0(local_2c,piVar3,3);
    local_4 = 1;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      piVar3 = (int *)FUN_0052bed0((int)local_2c);
      FUN_004a6be0(this,piVar3);
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_00502eb0(local_2c);
  }
  else {
    puVar2 = FUN_004025b0(piVar3,(uint *)&param_1);
    local_4 = 2;
    if ((*puVar2 >> 0x18 < 0x14) || (0x1b < *puVar2 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      iVar6 = *(int *)((int)this + 0x194);
      if (iVar6 == 0x66) {
        puVar5 = (undefined4 *)0x0;
        FUN_004a6e70(this,piVar3);
      }
      else if (iVar6 == 0x67) {
        pvVar4 = (void *)FUN_00618b70(0x20);
        local_4 = 3;
        if (pvVar4 == (void *)0x0) {
          puVar5 = (undefined4 *)0x0;
        }
        else {
          puVar5 = FUN_005039d0(pvVar4,piVar3,3);
        }
        local_4 = 0xffffffff;
        iVar6 = (**(code **)(*piVar3 + 0x23c))();
        *(int *)((int)this + 0x198) = *(int *)((int)this + 0x198) + iVar6;
        iVar6 = (**(code **)(*piVar3 + 0x26c))();
        *(int *)((int)this + 0x19c) = *(int *)((int)this + 0x19c) + iVar6;
      }
      else if (iVar6 == 0x68) {
        pvVar4 = (void *)FUN_00618b70(0x20);
        local_4 = 4;
        if (pvVar4 == (void *)0x0) {
          puVar5 = (undefined4 *)0x0;
        }
        else {
          puVar5 = FUN_00504c40(pvVar4,piVar3,3);
        }
        local_4 = 0xffffffff;
        iVar6 = (**(code **)(*piVar3 + 0x240))();
        *(int *)((int)this + 0x198) = *(int *)((int)this + 0x198) + iVar6;
        iVar6 = (**(code **)(*piVar3 + 0x270))();
        *(int *)((int)this + 0x19c) = *(int *)((int)this + 0x19c) + iVar6;
      }
      else {
        pvVar4 = (void *)FUN_00618b70(0x20);
        local_4 = 5;
        if (pvVar4 == (void *)0x0) {
          local_4 = 0xffffffff;
          puVar5 = (undefined4 *)0x0;
        }
        else {
          puVar5 = FUN_00536da0(pvVar4,piVar3,3);
          local_4 = 0xffffffff;
        }
      }
      if (puVar5 != (undefined4 *)0x0) {
        FUN_00513120((int)puVar5);
        iVar6 = puVar5[7];
        while (iVar6 != 0) {
          piVar3 = (int *)FUN_0052bed0((int)puVar5);
          FUN_004a6e70(this,piVar3);
          FUN_005130d0((int)puVar5);
          iVar6 = puVar5[7];
        }
        (**(code **)*puVar5)(1);
      }
    }
  }
  ExceptionList = local_c;
  return;
}

