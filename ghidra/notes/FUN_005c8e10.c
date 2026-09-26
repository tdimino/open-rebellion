
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int * __fastcall FUN_005c8e10(int *param_1)

{
  int *this;
  int iVar1;
  int iVar2;
  int *piVar3;
  undefined4 *puVar4;
  int iVar5;
  float10 fVar6;
  undefined **local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653ef9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f4ed0(local_14);
  local_14[0] = &PTR_FUN_0066cc38;
  this = param_1 + 0x24;
  local_4 = 0;
  (**(code **)(param_1[0x24] + 4))();
  if (param_1[10] == 0) {
    (**(code **)(*param_1 + 0x10))();
  }
  iVar1 = (**(code **)(*param_1 + 4))();
  iVar2 = FUN_005aaf90();
  iVar1 = *(int *)(iVar2 + 8 + (uint)(iVar1 == 0) * 4);
  for (piVar3 = (int *)(**(code **)(*(int *)(iVar1 + 0x58) + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar2 = (**(code **)(*piVar3 + 0x10))();
    if (iVar2 != 0) {
      fVar6 = FUN_005c9190(param_1,0,iVar2);
      puVar4 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 1;
      if (puVar4 == (undefined4 *)0x0) {
        puVar4 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar4);
        puVar4[6] = -(float)fVar6;
        puVar4[7] = iVar2;
        *puVar4 = &PTR_FUN_0066cc6c;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      iVar5 = FUN_005c91c0(param_1,iVar2);
      if (iVar5 == 1) {
        FUN_005f4f10(this,(int)puVar4);
      }
      else {
        FUN_005aaf90();
        iVar2 = FUN_005ad870(iVar2);
        if (iVar2 == 0) {
          if (puVar4 != (undefined4 *)0x0) {
            (**(code **)*puVar4)(1);
          }
        }
        else {
          FUN_005f4f10(local_14,(int)puVar4);
        }
      }
    }
  }
  for (piVar3 = (int *)(**(code **)(*(int *)(iVar1 + 100) + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar1 = (**(code **)(*piVar3 + 0x10))();
    if (iVar1 != 0) {
      iVar2 = iVar1;
      FUN_005aaf90();
      iVar2 = FUN_005ad870(iVar2);
      if ((iVar2 != 0) && (*(int *)(iVar1 + 0x170) != 10)) {
        fVar6 = FUN_005c9190(param_1,0,iVar1);
        puVar4 = (undefined4 *)FUN_00618b70(0x20);
        local_4._0_1_ = 2;
        if (puVar4 == (undefined4 *)0x0) {
          puVar4 = (undefined4 *)0x0;
        }
        else {
          FUN_005f5c10(puVar4);
          puVar4[6] = -(float)fVar6;
          puVar4[7] = iVar1;
          *puVar4 = &PTR_FUN_0066cc6c;
        }
        local_4 = (uint)local_4._1_3_ << 8;
        iVar2 = FUN_005c91c0(param_1,iVar1);
        if (iVar2 == 1) {
          FUN_005f4f10(this,(int)puVar4);
        }
        else {
          FUN_005aaf90();
          iVar1 = FUN_005ad870(iVar1);
          if (iVar1 == 0) {
            if (puVar4 != (undefined4 *)0x0) {
              (**(code **)*puVar4)(1);
            }
          }
          else {
            FUN_005f4f10(local_14,(int)puVar4);
          }
        }
      }
    }
  }
  iVar1 = FUN_005aaf90();
  for (piVar3 = (int *)(**(code **)(*(int *)(iVar1 + 0x9c4) + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar1 = (**(code **)(*piVar3 + 0x10))();
    if (((iVar1 != 0) &&
        (iVar2 = *(int *)(iVar1 + 0x168), iVar5 = (**(code **)(*param_1 + 4))(), iVar2 != iVar5)) &&
       ((*(int *)(iVar1 + 0x170) == 1 || (*(int *)(iVar1 + 0x170) == 3)))) {
      fVar6 = FUN_005c9190(param_1,0,iVar1);
      puVar4 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 3;
      if (puVar4 == (undefined4 *)0x0) {
        puVar4 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar4);
        puVar4[6] = -(float)fVar6;
        puVar4[7] = iVar1;
        *puVar4 = &PTR_FUN_0066cc6c;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f4f10(this,(int)puVar4);
    }
  }
  iVar1 = FUN_005f5060((int)local_14);
  while (iVar1 != 0) {
    iVar2 = *(int *)(iVar1 + 0x10);
    FUN_005f4fa0(local_14,iVar1);
    iVar5 = FUN_005f5080((int)this);
    if (iVar5 != 0) {
      *(float *)(iVar1 + 0x18) = *(float *)(iVar5 + 0x18) - _DAT_0066cbdc;
    }
    FUN_005f4f10(this,iVar1);
    iVar1 = iVar2;
  }
  local_4 = 0xffffffff;
  local_14[0] = &PTR_FUN_0066cc38;
  FUN_005f4f00(local_14);
  ExceptionList = pvStack_c;
  return this;
}

