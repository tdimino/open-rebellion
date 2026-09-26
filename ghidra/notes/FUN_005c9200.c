
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005c9200(void *param_1)

{
  void *this;
  int iVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined3 extraout_var;
  int *piVar6;
  undefined4 uVar7;
  undefined ***this_00;
  float local_40;
  float local_38;
  undefined **local_2c [2];
  undefined **appuStack_24 [3];
  undefined **appuStack_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653f49;
  local_c = ExceptionList;
  this = (void *)((int)param_1 + 0x90);
  ExceptionList = &local_c;
  iVar3 = FUN_005f5060((int)this);
  if (iVar3 != 0) {
    FUN_005f4ed0(local_2c);
    local_2c[0] = &PTR_FUN_0066cc38;
    local_4 = 0;
    iVar3 = FUN_005f5060((int)this);
    while (iVar1 = iVar3, iVar1 != 0) {
      iVar3 = *(int *)(iVar1 + 0x10);
      iVar4 = (**(code **)(**(int **)(iVar1 + 0x1c) + 0x20))();
      if (iVar4 == 1) {
        puVar5 = (undefined4 *)FUN_00618b70(0x20);
        local_4._0_1_ = 1;
        if (puVar5 == (undefined4 *)0x0) {
          puVar5 = (undefined4 *)0x0;
        }
        else {
          FUN_005f5c10(puVar5);
          puVar5[6] = *(undefined4 *)(iVar1 + 0x18);
          puVar5[7] = *(int *)(iVar1 + 0x1c);
          *puVar5 = &PTR_FUN_0066cc6c;
        }
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_005f4f10(local_2c,(int)puVar5);
        for (puVar5 = (undefined4 *)FUN_005f5060((int)this); puVar5 != (undefined4 *)0x0;
            puVar5 = (undefined4 *)puVar5[4]) {
          bVar2 = FUN_005ced00(puVar5 + 7,(int *)(iVar1 + 0x1c));
          if (CONCAT31(extraout_var,bVar2) != 0) goto LAB_005c92e4;
        }
        puVar5 = (undefined4 *)0x0;
LAB_005c92e4:
        if (puVar5 != (undefined4 *)0x0) {
          FUN_005f4fa0(this,(int)puVar5);
          (**(code **)*puVar5)(1);
        }
      }
    }
    local_40 = 1.0;
    local_38 = 0.0;
    iVar3 = FUN_005f5080((int)local_2c);
    if (iVar3 != 0) {
      local_38 = *(float *)(iVar3 + 0x18);
    }
    for (iVar3 = FUN_005f5060((int)this); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      puVar5 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (puVar5 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar5);
        puVar5[6] = local_38 + local_40;
        puVar5[7] = *(undefined4 *)(iVar3 + 0x1c);
        *puVar5 = &PTR_FUN_0066cc6c;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f4f10(local_2c,(int)puVar5);
      local_40 = local_40 - _DAT_0066cbdc;
    }
    local_38 = 0.0;
    iVar3 = FUN_005f5080((int)this);
    if (iVar3 != 0) {
      local_38 = *(float *)(iVar3 + 0x18);
    }
    local_40 = 1.0;
    for (iVar3 = FUN_005f5060((int)local_2c); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      puVar5 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 3;
      if (puVar5 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar5);
        puVar5[6] = local_38 + local_40;
        puVar5[7] = *(undefined4 *)(iVar3 + 0x1c);
        *puVar5 = &PTR_FUN_0066cc6c;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f4f10(this,(int)puVar5);
      local_40 = local_40 - _DAT_0066cbdc;
    }
    FUN_005f52c0(appuStack_18);
    appuStack_18[0] = &PTR_LAB_0066c390;
    local_4._0_1_ = 4;
    FUN_005f52c0(appuStack_24);
    appuStack_24[0] = &PTR_LAB_0066c390;
    local_4 = CONCAT31(local_4._1_3_,5);
    for (piVar6 = (int *)(**(code **)(*(int *)((int)param_1 + 0x70) + 0xc))(); piVar6 != (int *)0x0;
        piVar6 = (int *)(**(code **)(*piVar6 + 4))()) {
      uVar7 = (**(code **)(*piVar6 + 0xc))();
      iVar3 = FUN_005c91e0(param_1,uVar7);
      if (iVar3 == 0) {
        iVar3 = (**(code **)(*piVar6 + 0xc))();
        iVar3 = FUN_005c6070((int *)(iVar3 + 0x1c));
        if ((iVar3 == 0) || (iVar3 = FUN_005b7ce0(iVar3), iVar3 == 0)) {
          iVar3 = (**(code **)(*piVar6 + 0xc))();
          this_00 = appuStack_18;
        }
        else {
          iVar3 = (**(code **)(*piVar6 + 0xc))();
          this_00 = appuStack_24;
        }
        FUN_005ca4c0(this_00,iVar3);
      }
    }
    iVar3 = thunk_FUN_005f5060((int)appuStack_18);
    if (iVar3 != 0) {
      (**(code **)(**(int **)((int)param_1 + 0x34) + 4))(param_1,appuStack_18,this,DAT_0066cbac);
    }
    iVar3 = thunk_FUN_005f5060((int)appuStack_24);
    if (iVar3 != 0) {
      (**(code **)(**(int **)((int)param_1 + 0x34) + 4))(param_1,appuStack_24,local_2c,DAT_0066cbac)
      ;
    }
    local_4._0_1_ = 4;
    FUN_005ca3e0(appuStack_24);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005ca3e0(appuStack_18);
    local_4 = 0xffffffff;
    local_2c[0] = &PTR_FUN_0066cc38;
    FUN_005f4f00(local_2c);
  }
  ExceptionList = local_c;
  return;
}

