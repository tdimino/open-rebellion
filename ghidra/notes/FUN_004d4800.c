
int * __fastcall FUN_004d4800(int param_1)

{
  uint *this;
  void *this_00;
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int *piVar5;
  int iVar6;
  uint *puVar7;
  int iVar8;
  void *local_54;
  int *local_50;
  undefined4 uStack_4c;
  uint *local_48;
  uint local_44;
  uint uStack_40;
  void *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c4a2;
  pvStack_c = ExceptionList;
  local_44 = 0;
  local_50 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  puVar7 = (uint *)(param_1 + 0x54);
  local_4 = 0;
  local_48 = puVar7;
  FUN_004ece80(puVar7);
  this = (uint *)(param_1 + 0x50);
  FUN_004ece80(this);
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x2000000,0,0,0,0,0,0x22,(void *)0x1);
  this_00 = (void *)(param_1 + 0x40);
  FUN_00435790(this_00,(int)pvVar3);
  puVar4 = FUN_004357b0(this_00,&local_54);
  local_4._0_1_ = 1;
  FUN_004f26d0(this,puVar4);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(this_00);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0,0,0x40000,0x35,(void *)0x1);
  FUN_00435790(this_00,(int)pvVar3);
  puVar4 = FUN_004357b0(this_00,&local_54);
  local_4._0_1_ = 2;
  FUN_004f26d0(this,puVar4);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(this_00);
  local_3c = (void *)0x90;
  local_38 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x8000000,0,0,0x40000000,0,0,0x19,&DAT_00000002
                         );
    FUN_00435790(this_00,(int)pvVar3);
    puVar4 = FUN_004357b0(this_00,&local_54);
    local_4._0_1_ = 3;
    FUN_004f26d0(this,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(this_00);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x80000,0,0,0x80000000,0,0x40000,0x17,
                          &DAT_00000002);
    FUN_00435790(this_00,(int)pvVar3);
    puVar4 = FUN_004357b0(this_00,&local_54);
    local_4._0_1_ = 4;
    FUN_004f26d0(this,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(this_00);
  }
  local_3c = (void *)0x90;
  local_38 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x8000000,0,0,0,0,0,0x19,&DAT_00000002);
    FUN_00435790(this_00,(int)pvVar3);
    puVar4 = FUN_004357b0(this_00,&local_54);
    local_4._0_1_ = 5;
    FUN_004f26d0(this,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(this_00);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x80000,0,0,0,0,0x40000,0x17,&DAT_00000002
                         );
    FUN_00435790(this_00,(int)pvVar3);
    puVar4 = FUN_004357b0(this_00,&local_54);
    local_4._0_1_ = 6;
    FUN_004f26d0(this,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(this_00);
  }
  local_3c = (void *)0x90;
  local_38 = 0x98;
  if ((*this >> 0x18 < 0x90) || (local_54 = (void *)0x1, 0x97 < *this >> 0x18)) {
    local_54 = (void *)0x0;
  }
  FUN_00619730();
  if ((local_54 != (void *)0x0) &&
     (piVar5 = FUN_004f3220(*(int *)(param_1 + 0x30),this), piVar5 != (int *)0x0)) {
    iVar8 = 100000;
    local_54 = (void *)0x0;
    FUN_005039d0(local_2c,piVar5,1);
    local_4._0_1_ = 7;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      piVar5 = (int *)FUN_0052bed0((int)local_2c);
      iVar6 = (**(code **)(*piVar5 + 500))();
      if (iVar6 < iVar8) {
        piVar5 = (int *)FUN_0052bed0((int)local_2c);
        iVar8 = (**(code **)(*piVar5 + 500))();
        local_54 = (void *)FUN_0052bed0((int)local_2c);
      }
      FUN_005130d0((int)local_2c);
    }
    if (local_54 != (void *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 8;
      local_3c = pvVar3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        puVar7 = FUN_004025b0(local_54,&uStack_40);
        local_44 = 1;
        local_4 = CONCAT31(local_4._1_3_,9);
        pvVar3 = FUN_004f5b10(pvVar3,puVar7,0);
      }
      local_4 = 8;
      if ((local_44 & 1) != 0) {
        FUN_00619730();
      }
      local_4._0_1_ = 7;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_34,pvVar3);
      }
    }
    local_4._0_1_ = 0;
    FUN_00503ad0(local_2c);
    puVar7 = local_48;
  }
  iVar8 = thunk_FUN_005f5060((int)local_34);
  if (iVar8 != 0) {
    puVar4 = FUN_0041a600(*(void **)(param_1 + 0x2c),this,0x100009,2,2);
    FUN_00435790(this_00,(int)puVar4);
    pvVar3 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),0x100009,2,0x17,(void *)0x1);
    FUN_00435790(this_00,(int)pvVar3);
    puVar4 = FUN_004357b0(this_00,&uStack_40);
    local_4._0_1_ = 10;
    FUN_004f26d0(puVar7,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(this_00);
    local_3c = (void *)0x8;
    local_38 = 0x10;
    if ((*puVar7 >> 0x18 < 8) || (local_44 = 1, 0xf < *puVar7 >> 0x18)) {
      local_44 = 0;
    }
    FUN_00619730();
    if (local_44 == 0) {
      puVar4 = FUN_0041a600(*(void **)(param_1 + 0x2c),this,0x100005,2,2);
      FUN_00435790(this_00,(int)puVar4);
      pvVar3 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),0x100005,2,0x17,(void *)0x1);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = FUN_004357b0(this_00,&uStack_40);
      local_4._0_1_ = 0xb;
      FUN_004f26d0(puVar7,puVar4);
      local_4._0_1_ = 0;
      FUN_00619730();
      FUN_005f58b0(this_00);
      local_3c = (void *)0x8;
      local_38 = 0x10;
      if ((*puVar7 >> 0x18 < 8) || (local_44 = 1, 0xf < *puVar7 >> 0x18)) {
        local_44 = 0;
      }
      FUN_00619730();
      if (local_44 == 0) {
        FUN_004ece30(&local_54);
        local_4._0_1_ = 0xc;
        pvVar3 = FUN_00419e90(*(void **)(param_1 + 0x2c),0,0,0x8000000,0,0,0,&DAT_00000002);
        FUN_00435790(this_00,(int)pvVar3);
        puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),this,0,0,0x8000000,0,0,0,2);
        FUN_00435790(this_00,(int)puVar4);
        puVar4 = FUN_004357b0(this_00,&uStack_40);
        local_4._0_1_ = 0xd;
        FUN_004f26d0(&local_54,puVar4);
        bVar1 = false;
        do {
          local_4 = CONCAT31(local_4._1_3_,0xc);
          FUN_00619730();
          do {
            local_3c = (void *)0x80;
            local_38 = 0x90;
            if ((((uint)local_54 >> 0x18 < 0x80) || (0x8f < (uint)local_54 >> 0x18)) || (bVar1)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            FUN_00619730();
            if (!bVar2) {
              FUN_005f58b0(this_00);
              local_4._0_1_ = 0;
              FUN_00619730();
              puVar7 = local_48;
              if (bVar1) goto LAB_004d4f73;
              local_50 = (int *)FUN_004f5cd0(0x200);
              if (local_50 != (int *)0x0) {
                local_50[8] = *(int *)(param_1 + 0x30);
                (**(code **)(*local_50 + 0x24))(local_34);
              }
              goto LAB_004d4fd1;
            }
            pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),(uint *)&local_54);
            if ((pvVar3 != (void *)0x0) &&
               (pvVar3 = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),
                                              *(uint *)((int)pvVar3 + 0x38)), pvVar3 != (void *)0x0)
               ) {
              puVar4 = FUN_00476f90(pvVar3,&uStack_40);
              puVar7 = local_48;
              local_4._0_1_ = 0xe;
              FUN_004f26d0(local_48,puVar4);
              local_4 = CONCAT31(local_4._1_3_,0xc);
              FUN_00619730();
              local_50 = (int *)0x90;
              uStack_4c = 0x98;
              if ((*puVar7 >> 0x18 < 0x90) || (0x97 < *puVar7 >> 0x18)) {
                bVar2 = false;
              }
              else {
                bVar2 = true;
              }
              FUN_00619730();
              if (bVar2) {
                bVar1 = true;
              }
            }
          } while (bVar1);
          puVar4 = FUN_004357b0(this_00,&local_44);
          local_4._0_1_ = 0xf;
          FUN_004f26d0(&local_54,puVar4);
        } while( true );
      }
    }
LAB_004d4f73:
    piVar5 = (int *)FUN_004f5cd0(0x201);
    local_50 = piVar5;
    if (piVar5 != (int *)0x0) {
      piVar5[8] = *(int *)(param_1 + 0x30);
      (**(code **)(*piVar5 + 0x24))(local_34);
      (**(code **)(*piVar5 + 0x2c))(puVar7);
    }
  }
LAB_004d4fd1:
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return local_50;
}

