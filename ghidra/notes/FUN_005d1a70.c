
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int * __fastcall FUN_005d1a70(int *param_1)

{
  int *this;
  int iVar1;
  undefined4 *puVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  float10 fVar6;
  undefined **local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065459f;
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
  if (DAT_006bc4a8 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)FUN_00618b70(0x274);
    local_4._0_1_ = 1;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc4a8 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc4a8 = FUN_005af5f0(puVar2);
    }
    local_4 = (uint)local_4._1_3_ << 8;
  }
  iVar1 = DAT_006bc4a8[(iVar1 == 0) + 0x93];
  for (piVar3 = (int *)(**(code **)(*(int *)(iVar1 + 0x58) + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar4 = (**(code **)(*piVar3 + 0x10))();
    if (iVar4 != 0) {
      fVar6 = FUN_005c9190(param_1,0,iVar4);
      puVar2 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (puVar2 == (undefined4 *)0x0) {
        puVar2 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar2);
        puVar2[6] = -(float)fVar6;
        puVar2[7] = iVar4;
        *puVar2 = &PTR_FUN_0066cc6c;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      iVar5 = FUN_005c91c0(param_1,iVar4);
      if (iVar5 == 1) {
        FUN_005f4f10(this,(int)puVar2);
      }
      else if (*(int *)(iVar4 + 0x170) == 2) {
        FUN_005f4f10(local_14,(int)puVar2);
      }
      else if (puVar2 != (undefined4 *)0x0) {
        (**(code **)*puVar2)(1);
      }
    }
  }
  for (piVar3 = (int *)(**(code **)(*(int *)(iVar1 + 100) + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar1 = (**(code **)(*piVar3 + 0x10))();
    if ((iVar1 != 0) && (*(int *)(iVar1 + 0x170) == 2)) {
      fVar6 = FUN_005c9190(param_1,0,iVar1);
      puVar2 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 3;
      if (puVar2 == (undefined4 *)0x0) {
        puVar2 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar2);
        puVar2[6] = -(float)fVar6;
        puVar2[7] = iVar1;
        *puVar2 = &PTR_FUN_0066cc6c;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      iVar4 = FUN_005c91c0(param_1,iVar1);
      if (iVar4 == 1) {
        FUN_005f4f10(this,(int)puVar2);
      }
      else if (*(int *)(iVar1 + 0x170) == 2) {
        FUN_005f4f10(local_14,(int)puVar2);
      }
      else if (puVar2 != (undefined4 *)0x0) {
        (**(code **)*puVar2)(1);
      }
    }
  }
  if (DAT_006bc4a8 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)FUN_00618b70(0x274);
    local_4._0_1_ = 4;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc4a8 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc4a8 = FUN_005af5f0(puVar2);
    }
    local_4 = (uint)local_4._1_3_ << 8;
  }
  for (piVar3 = (int *)(**(code **)(DAT_006bc4a8[0x16] + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    iVar1 = (**(code **)(*piVar3 + 0x10))();
    if (((iVar1 != 0) &&
        (iVar4 = *(int *)(iVar1 + 0x168), iVar5 = (**(code **)(*param_1 + 4))(), iVar4 != iVar5)) &&
       ((*(int *)(iVar1 + 0x170) == 1 || (*(int *)(iVar1 + 0x170) == 3)))) {
      fVar6 = FUN_005c9190(param_1,0,iVar1);
      puVar2 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 5;
      if (puVar2 == (undefined4 *)0x0) {
        puVar2 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar2);
        puVar2[6] = -(float)fVar6;
        puVar2[7] = iVar1;
        *puVar2 = &PTR_FUN_0066cc6c;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f4f10(this,(int)puVar2);
    }
  }
  iVar1 = FUN_005f5060((int)local_14);
  while (iVar1 != 0) {
    iVar4 = *(int *)(iVar1 + 0x10);
    FUN_005f4fa0(local_14,iVar1);
    iVar5 = FUN_005f5080((int)this);
    if (iVar5 != 0) {
      *(float *)(iVar1 + 0x18) = *(float *)(iVar5 + 0x18) - _DAT_0066d0fc;
    }
    FUN_005f4f10(this,iVar1);
    iVar1 = iVar4;
  }
  local_4 = 0xffffffff;
  local_14[0] = &PTR_FUN_0066cc38;
  FUN_005f4f00(local_14);
  ExceptionList = pvStack_c;
  return this;
}

