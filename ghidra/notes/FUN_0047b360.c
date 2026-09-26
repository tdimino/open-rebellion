
int * __fastcall FUN_0047b360(int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  uint *puVar4;
  void *pvVar5;
  undefined4 *puVar6;
  int *unaff_EBX;
  uint uVar7;
  int *local_34;
  uint local_30;
  undefined1 auStack_2c [4];
  void *pvStack_28;
  void *local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined1 auStack_18 [4];
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined4 uStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  uStack_8._0_1_ = 0x34;
  uStack_8._1_3_ = 0x6337;
  pvStack_c = ExceptionList;
  bVar1 = false;
  uVar7 = 0;
  local_30 = 0;
  local_34 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  FUN_004f4340(&local_1c);
  uVar2 = *(uint *)(param_1 + 0x30) >> 0x18;
  local_4._0_1_ = 1;
  local_24 = (void *)0x1;
  local_20 = 0xff;
  if ((uVar2 != 0) && (uVar2 < 0xff)) {
    bVar1 = true;
  }
  FUN_00619730();
  if ((bVar1) && (local_34 = (int *)FUN_004f5cd0(0x240), local_34 != (int *)0x0)) {
    local_34[8] = *(int *)(param_1 + 0x24);
    (**(code **)(*local_34 + 0x2c))(param_1 + 0x30);
    FUN_004f26d0(local_34 + 0x13,(undefined4 *)(param_1 + 0x2c));
    local_34[0x11] = *(int *)(param_1 + 0x18);
    for (pvVar3 = (void *)thunk_FUN_005f5060(param_1 + 0x3c); pvVar3 != (void *)0x0;
        pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
      puVar4 = FUN_00403040(pvVar3,&local_30);
      uStack_8._0_1_ = 2;
      pvVar5 = FUN_004f5940(*(void **)(param_1 + 100),puVar4);
      uStack_8._0_1_ = 1;
      FUN_00619730();
      if ((pvVar5 != (void *)0x0) && ((*(uint *)((int)pvVar5 + 0x30) & 0x74000f1) == 0)) {
        *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) | 0x4000;
        pvVar5 = (void *)FUN_00618b70(0x20);
        uStack_8._0_1_ = 3;
        pvStack_28 = pvVar5;
        if (pvVar5 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          puVar6 = FUN_00403040(pvVar3,auStack_2c);
          uVar7 = 1;
          local_34 = (int *)0x1;
          uStack_8 = CONCAT31(uStack_8._1_3_,4);
          pvVar5 = FUN_004f5b10(pvVar5,puVar6,0);
        }
        uStack_8 = 3;
        if (uVar7 != 0) {
          uVar7 = 0;
          FUN_00619730();
        }
        uStack_8._0_1_ = 1;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(auStack_18,pvVar5);
        }
      }
    }
    (**(code **)(*unaff_EBX + 0x24))(auStack_18);
    for (pvVar3 = (void *)thunk_FUN_005f5060(param_1 + 0x44); pvVar3 != (void *)0x0;
        pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
      puVar4 = FUN_00403040(pvVar3,&pvStack_28);
      local_4._0_1_ = 5;
      pvVar5 = FUN_004f5940(*(void **)(param_1 + 100),puVar4);
      local_4._0_1_ = 1;
      FUN_00619730();
      if ((pvVar5 != (void *)0x0) && ((*(uint *)((int)pvVar5 + 0x30) & 0x74000f1) == 0)) {
        *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) | 0x4000;
        pvVar5 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 6;
        local_24 = pvVar5;
        if (pvVar5 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          puVar6 = FUN_00403040(pvVar3,auStack_2c);
          uVar7 = uVar7 | 2;
          local_4 = CONCAT31(local_4._1_3_,7);
          local_30 = uVar7;
          pvVar5 = FUN_004f5b10(pvVar5,puVar6,0);
        }
        local_4 = 6;
        if ((uVar7 & 2) != 0) {
          uVar7 = uVar7 & 0xfffffffd;
          FUN_00619730();
        }
        local_4._0_1_ = 1;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(&local_1c,pvVar5);
        }
      }
      unaff_EBX = local_34;
    }
    FUN_004f43b0(unaff_EBX + 0x16,&local_1c);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(&local_1c);
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return local_34;
}

