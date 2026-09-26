
int * __fastcall FUN_004c8e90(int param_1)

{
  void *this;
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  uint *puVar4;
  void *pvVar5;
  undefined1 *this_00;
  int iVar6;
  undefined4 *puVar7;
  int *piVar8;
  undefined1 auStack_34 [3];
  char local_31;
  uint local_30;
  uint local_2c;
  int *local_28;
  undefined1 local_24 [4];
  undefined1 *local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b03a;
  pvStack_c = ExceptionList;
  local_2c = 0;
  local_28 = (int *)0x0;
  local_30 = 0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_1c);
  local_4 = 0;
  pvVar2 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),0x1005,0x802,0,(void *)0x1);
  this = (void *)(param_1 + 0x40);
  FUN_00435790(this,(int)pvVar2);
  puVar3 = FUN_004357b0(this,local_24);
  puVar4 = (uint *)(param_1 + 0x54);
  local_4._0_1_ = 1;
  FUN_004f26d0(puVar4,puVar3);
  do {
    local_4._0_1_ = 0;
    FUN_00619730();
    do {
      local_14 = 8;
      local_10 = 0x10;
      if (((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) || (local_31 = '\x01', local_30 != 0))
      {
        local_31 = '\0';
      }
      FUN_00619730();
      if (local_31 == '\0') {
        FUN_005f58b0(this);
        local_14 = 8;
        local_10 = 0x10;
        if ((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        local_31 = !bVar1;
        FUN_00619730();
        if (local_31 == '\0') goto LAB_004c9136;
        pvVar2 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),0x1005,0x802,0,(void *)0x1);
        FUN_00435790(this,(int)pvVar2);
        puVar3 = FUN_004357b0(this,local_24);
        local_4._0_1_ = 3;
        FUN_004f26d0(puVar4,puVar3);
        do {
          local_4._0_1_ = 0;
          FUN_00619730();
          do {
            local_14 = 8;
            local_10 = 0x10;
            if (((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) ||
               (local_31 = '\x01', local_30 != 0)) {
              local_31 = '\0';
            }
            FUN_00619730();
            if (local_31 == '\0') {
              FUN_005f58b0(this);
LAB_004c9136:
              local_14 = 8;
              local_10 = 0x10;
              if ((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) {
                bVar1 = false;
              }
              else {
                bVar1 = true;
              }
              FUN_00619730();
              if (bVar1) {
                iVar6 = *(int *)(param_1 + 0x2c);
                local_20 = &stack0xffffffb8;
                FUN_004f26d0(&stack0xffffffb8,puVar4);
                pvVar2 = FUN_00419e40((void *)(iVar6 + 0x78));
                if (pvVar2 != (void *)0x0) {
                  for (pvVar2 = (void *)thunk_FUN_005f5060((int)pvVar2 + 0x48);
                      pvVar2 != (void *)0x0; pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
                    puVar4 = FUN_00403040(pvVar2,local_24);
                    local_4._0_1_ = 5;
                    pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x58),puVar4);
                    local_4._0_1_ = 0;
                    FUN_00619730();
                    if (((pvVar5 != (void *)0x0) && ((*(uint *)((int)pvVar5 + 0x24) & 0x1000) != 0))
                       && ((*(uint *)((int)pvVar5 + 0x24) & 0x800800) == 0)) {
                      this_00 = (undefined1 *)FUN_00618b70(0x20);
                      local_4._0_1_ = 6;
                      local_20 = this_00;
                      if (this_00 == (undefined1 *)0x0) {
                        pvVar5 = (void *)0x0;
                      }
                      else {
                        puVar3 = FUN_00403040(pvVar2,&local_30);
                        local_2c = local_2c | 1;
                        local_4 = CONCAT31(local_4._1_3_,7);
                        pvVar5 = FUN_004f5b10(this_00,puVar3,0);
                      }
                      local_4 = 6;
                      if ((local_2c & 1) != 0) {
                        local_2c = local_2c & 0xfffffffe;
                        FUN_00619730();
                      }
                      local_4._0_1_ = 0;
                      if (pvVar5 != (void *)0x0) {
                        FUN_004f57b0(local_1c,pvVar5);
                      }
                    }
                  }
                }
                iVar6 = thunk_FUN_005f5060((int)local_1c);
                if (iVar6 != 0) {
                  FUN_004ece30(&local_30);
                  puVar3 = (undefined4 *)(param_1 + 0x54);
                  local_4._0_1_ = 8;
                  puVar7 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar3,8,0,0x20000,0,0,0,2);
                  FUN_00435790(this,(int)puVar7);
                  puVar7 = FUN_004357b0(this,local_24);
                  local_4._0_1_ = 9;
                  FUN_004f26d0(&local_30,puVar7);
                  local_4._0_1_ = 8;
                  FUN_00619730();
                  FUN_005f58b0(this);
                  puVar7 = FUN_0041a400(*(void **)(param_1 + 0x2c),&local_30,puVar3,0x200000,0,0,2,0
                                        ,0,(undefined4 *)&DAT_00000002);
                  FUN_00435790(this,(int)puVar7);
                  puVar7 = FUN_004357b0(this,local_24);
                  local_4._0_1_ = 10;
                  FUN_004f26d0(&local_30,puVar7);
                  local_4._0_1_ = 8;
                  FUN_00619730();
                  FUN_005f58b0(this);
                  local_14 = 0x90;
                  local_10 = 0x98;
                  if ((local_30 >> 0x18 < 0x90) || (0x97 < local_30 >> 0x18)) {
                    bVar1 = false;
                  }
                  else {
                    bVar1 = true;
                  }
                  local_31 = !bVar1;
                  FUN_00619730();
                  if (local_31 != '\0') {
                    puVar7 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar3,8,0,4,0,0,0,2);
                    FUN_00435790(this,(int)puVar7);
                    puVar7 = FUN_004357b0(this,local_24);
                    local_4._0_1_ = 0xb;
                    FUN_004f26d0(&local_30,puVar7);
                    local_4._0_1_ = 8;
                    FUN_00619730();
                    FUN_005f58b0(this);
                    puVar3 = FUN_0041a400(*(void **)(param_1 + 0x2c),&local_30,puVar3,0,0,1,2,0,0,
                                          (undefined4 *)&DAT_00000002);
                    FUN_00435790(this,(int)puVar3);
                    puVar3 = FUN_004357b0(this,local_24);
                    local_4._0_1_ = 0xc;
                    FUN_004f26d0(&local_30,puVar3);
                    local_4._0_1_ = 8;
                    FUN_00619730();
                    FUN_005f58b0(this);
                  }
                  local_14 = 0x90;
                  local_10 = 0x98;
                  if ((local_30 >> 0x18 < 0x90) || (0x97 < local_30 >> 0x18)) {
                    bVar1 = false;
                  }
                  else {
                    bVar1 = true;
                  }
                  FUN_00619730();
                  if ((bVar1) &&
                     (piVar8 = (int *)FUN_004f5cd0(0x201), local_28 = piVar8, piVar8 != (int *)0x0))
                  {
                    piVar8[8] = *(int *)(param_1 + 0x30);
                    (**(code **)(*piVar8 + 0x24))();
                    (**(code **)(*piVar8 + 0x2c))(auStack_34);
                  }
                  local_4 = (uint)local_4._1_3_ << 8;
                  FUN_00619730();
                }
              }
              local_4 = 0xffffffff;
              FUN_004f4380(local_1c);
              ExceptionList = pvStack_c;
              return local_28;
            }
            iVar6 = *(int *)(param_1 + 0x2c);
            local_20 = &stack0xffffffb8;
            FUN_004f26d0(&stack0xffffffb8,puVar4);
            pvVar2 = FUN_00419e40((void *)(iVar6 + 0x78));
            if (pvVar2 != (void *)0x0) {
              iVar6 = *(int *)(param_1 + 0x2c);
              local_20 = &stack0xffffffb8;
              FUN_004f26d0(&stack0xffffffb8,(undefined4 *)((int)pvVar2 + 0x28));
              pvVar2 = FUN_00403d30((void *)(iVar6 + 0x2c));
              if ((pvVar2 != (void *)0x0) && ((*(uint *)((int)pvVar2 + 0x28) & 0x200000) == 0)) {
                local_30 = 1;
              }
            }
          } while (local_30 != 0);
          puVar3 = FUN_004357b0(this,local_24);
          local_4._0_1_ = 4;
          FUN_004f26d0(puVar4,puVar3);
        } while( true );
      }
      iVar6 = *(int *)(param_1 + 0x2c);
      local_20 = &stack0xffffffb8;
      FUN_004f26d0(&stack0xffffffb8,puVar4);
      pvVar2 = FUN_00419e40((void *)(iVar6 + 0x78));
      if (pvVar2 != (void *)0x0) {
        iVar6 = *(int *)(param_1 + 0x2c);
        local_20 = &stack0xffffffb8;
        FUN_004f26d0(&stack0xffffffb8,(undefined4 *)((int)pvVar2 + 0x28));
        pvVar2 = FUN_00403d30((void *)(iVar6 + 0x2c));
        if ((pvVar2 != (void *)0x0) && ((*(byte *)((int)pvVar2 + 0x30) & 4) != 0)) {
          local_30 = 1;
        }
      }
    } while (local_30 != 0);
    puVar3 = FUN_004357b0(this,local_24);
    local_4._0_1_ = 2;
    FUN_004f26d0(puVar4,puVar3);
  } while( true );
}

