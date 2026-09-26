
void __fastcall FUN_00479560(int param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *pvVar4;
  uint uVar5;
  uint *puVar6;
  void *pvVar7;
  uint local_28;
  int local_24;
  int local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006333c0;
  local_c = ExceptionList;
  local_24 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_28);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  local_20 = 0;
  puVar6 = (uint *)(param_1 + 0x38);
  if ((*(uint *)(param_1 + 0x60) & 0x2000000) == 0) {
    pvVar2 = FUN_004f5940((void *)(param_1 + 0x24),puVar6);
    if (((pvVar2 != (void *)0x0) &&
        (pvVar2 = FUN_004f5940((void *)(*(int *)(param_1 + 0x58) + 0x58),puVar6),
        pvVar2 != (void *)0x0)) && (*(int *)((int)pvVar2 + 0x30) == *(int *)(param_1 + 0x18))) {
      *(uint *)((int)pvVar2 + 0x24) = *(uint *)((int)pvVar2 + 0x24) & 0xfeffffff;
    }
    FUN_004ece80(puVar6);
    goto LAB_0047999d;
  }
  pvVar2 = FUN_004f5940((void *)(param_1 + 0x24),puVar6);
  if (((pvVar2 != (void *)0x0) &&
      (pvVar2 = FUN_004f5940((void *)(*(int *)(param_1 + 0x58) + 0x58),puVar6),
      pvVar2 != (void *)0x0)) &&
     ((*(int *)((int)pvVar2 + 0x30) == *(int *)(param_1 + 0x18) &&
      (uVar5 = *(uint *)((int)pvVar2 + 0x24), (uVar5 & 0x1000000) != 0)))) {
    if ((uVar5 & 0x2000000) == 0) {
      bVar1 = true;
      local_24 = 1;
      if ((uVar5 & 0x800) == 0) {
        FUN_004f26d0(&local_28,(undefined4 *)((int)pvVar2 + 0x2c));
        pvVar7 = FUN_004f5940((void *)(param_1 + 0x2c),&local_28);
        if (pvVar7 != (void *)0x0) {
          pvVar4 = FUN_0041ac80(*(void **)(param_1 + 0x58),&local_28,0x4001,0x2803800,3,(void *)0x1)
          ;
          pvVar7 = (void *)(param_1 + 0x48);
          FUN_00435790(pvVar7,(int)pvVar4);
          puVar3 = FUN_004357b0(pvVar7,&local_1c);
          local_4._0_1_ = 1;
          FUN_004f26d0(&local_28,puVar3);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar7);
          local_1c = 0x14;
          local_18 = 0x1c;
          if ((local_28 >> 0x18 < 0x14) || (0x1b < local_28 >> 0x18)) {
            bVar1 = false;
          }
          FUN_00619730();
          if (bVar1) {
            pvVar7 = FUN_004f5940((void *)(*(int *)(param_1 + 0x58) + 0x58),&local_28);
            if ((pvVar7 != (void *)0x0) &&
               ((*(int *)((int)pvVar2 + 0x50) < *(int *)((int)pvVar7 + 0x50) ||
                ((*(byte *)((int)pvVar2 + 0x24) & 1) == 0)))) {
LAB_00479793:
              *(uint *)((int)pvVar7 + 0x24) = *(uint *)((int)pvVar7 + 0x24) | 0x1000000;
              *(uint *)((int)pvVar2 + 0x24) = *(uint *)((int)pvVar2 + 0x24) & 0xfeffffff;
              FUN_004f26d0((void *)(param_1 + 0x38),&local_28);
            }
          }
          else {
            pvVar4 = FUN_0041ac80(*(void **)(param_1 + 0x58),&local_28,0x4000,0x2803800,3,
                                  (void *)0x1);
            FUN_00435790(pvVar7,(int)pvVar4);
            puVar3 = FUN_004357b0(pvVar7,&local_1c);
            local_4._0_1_ = 2;
            FUN_004f26d0(&local_28,puVar3);
            local_4._0_1_ = 0;
            FUN_00619730();
            FUN_005f58b0(pvVar7);
            local_1c = 0x14;
            local_18 = 0x1c;
            if ((local_28 >> 0x18 < 0x14) || (0x1b < local_28 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (((bVar1) &&
                (pvVar7 = FUN_004f5940((void *)(*(int *)(param_1 + 0x58) + 0x58),&local_28),
                pvVar7 != (void *)0x0)) &&
               (*(int *)((int)pvVar2 + 0x50) < *(int *)((int)pvVar7 + 0x50))) goto LAB_00479793;
          }
        }
      }
    }
    else {
      *(uint *)((int)pvVar2 + 0x24) = uVar5 & 0xfeffffff;
    }
  }
  if (local_24 == 0) {
    FUN_004ece80((undefined4 *)(param_1 + 0x38));
    pvVar2 = (void *)thunk_FUN_005f5060(param_1 + 0x2c);
    if (pvVar2 != (void *)0x0) {
      pvVar7 = (void *)(param_1 + 0x48);
      do {
        puVar3 = FUN_00403040(pvVar2,&local_1c);
        local_4._0_1_ = 3;
        pvVar4 = FUN_0041ac80(*(void **)(param_1 + 0x58),puVar3,0x4000,0x2803800,3,(void *)0x1);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_00435790(pvVar7,(int)pvVar4);
        puVar3 = FUN_004357b0(pvVar7,&local_24);
        local_4._0_1_ = 4;
        FUN_004f26d0(&local_28,puVar3);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_005f58b0(pvVar7);
        local_14 = 0x14;
        local_10 = 0x1c;
        if ((local_28 >> 0x18 < 0x14) || (0x1b < local_28 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (((bVar1) &&
            (pvVar4 = FUN_004f5940((void *)(*(int *)(param_1 + 0x58) + 0x58),&local_28),
            pvVar4 != (void *)0x0)) && (local_20 < *(int *)((int)pvVar4 + 0x50))) {
          FUN_004f26d0((void *)(param_1 + 0x38),&local_28);
          local_20 = *(int *)((int)pvVar4 + 0x50);
        }
        pvVar2 = *(void **)((int)pvVar2 + 0x10);
      } while (pvVar2 != (void *)0x0);
    }
    uVar5 = *(uint *)(param_1 + 0x38) >> 0x18;
    local_14 = 0x14;
    local_10 = 0x1c;
    if ((uVar5 < 0x14) || (0x1b < uVar5)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if ((bVar1) &&
       (pvVar2 = FUN_004f5940((void *)(*(int *)(param_1 + 0x58) + 0x58),(uint *)(param_1 + 0x38)),
       pvVar2 != (void *)0x0)) {
      *(uint *)((int)pvVar2 + 0x24) = *(uint *)((int)pvVar2 + 0x24) | 0x1000000;
    }
  }
  for (pvVar2 = (void *)thunk_FUN_005f5060(param_1 + 0x24); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    puVar6 = FUN_00403040(pvVar2,&local_1c);
    local_4._0_1_ = 5;
    pvVar7 = FUN_004f5940((void *)(*(int *)(param_1 + 0x58) + 0x58),puVar6);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (pvVar7 != (void *)0x0) {
      *(uint *)((int)pvVar7 + 0x24) = *(uint *)((int)pvVar7 + 0x24) & 0xfdffffff;
    }
  }
LAB_0047999d:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

