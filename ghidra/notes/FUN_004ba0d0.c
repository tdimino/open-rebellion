
void * __thiscall FUN_004ba0d0(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  short sVar4;
  void *pvVar5;
  undefined4 *puVar6;
  void *pvVar7;
  uint local_24;
  void *local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639972;
  local_c = ExceptionList;
  bVar2 = false;
  local_18 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  iVar1 = *(int *)((int)this + 0xc);
  local_1c = &stack0xffffffc8;
  local_4 = 1;
  FUN_004f26d0(&stack0xffffffc8,(undefined4 *)((int)this + 0x28));
  local_20 = FUN_00419e40((void *)(iVar1 + 0x78));
  if (local_20 != (void *)0x0) {
    sVar4 = FUN_005f50e0(*(int *)((int)this + 0x10) + 0x2c);
    if (sVar4 == 1) {
      iVar1 = *(int *)((int)this + 0xc);
      local_1c = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,(undefined4 *)((int)this + 0x28));
      local_20 = FUN_00419e40((void *)(iVar1 + 0x78));
      if ((((local_20 != (void *)0x0) && ((*(uint *)((int)local_20 + 0x38) & 0xa00) == 0)) &&
          (pvVar7 = *(void **)((int)this + 0xc),
          *(int *)((int)pvVar7 + 0x164) <= *(int *)((int)local_20 + 0x58))) &&
         (*(int *)((int)pvVar7 + 0x168) <= *(int *)((int)local_20 + 0x5c))) {
        pvVar5 = FUN_0041a3a0(pvVar7,(undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0,0x20,0,0,
                              0x700201,(void *)0x1);
        pvVar7 = (void *)((int)this + 0x18);
        FUN_00435790(pvVar7,(int)pvVar5);
        puVar6 = FUN_004357b0(pvVar7,&local_1c);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_24,puVar6);
        do {
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
          while( true ) {
            local_14 = 0x90;
            local_10 = 0x98;
            if (((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) || (bVar2)) {
              bVar3 = false;
            }
            else {
              bVar3 = true;
            }
            FUN_00619730();
            if (!bVar3) {
              FUN_005f58b0(pvVar7);
              if (bVar2) {
                local_1c = (undefined1 *)FUN_00618b70(0x1c);
                local_4._0_1_ = 4;
                if (local_1c == (undefined1 *)0x0) {
                  pvVar7 = (void *)0x0;
                }
                else {
                  pvVar7 = FUN_004ecc70(local_1c,&local_24);
                }
                local_4 = CONCAT31(local_4._1_3_,1);
                if (pvVar7 != (void *)0x0) {
                  FUN_004f57b0((void *)((int)this + 0x5c),pvVar7);
                }
              }
              goto LAB_004ba2b1;
            }
            pvVar5 = FUN_004f5940((void *)((int)this + 0x5c),&local_24);
            if (pvVar5 != (void *)0x0) break;
            bVar2 = true;
          }
          puVar6 = FUN_004357b0(pvVar7,&local_1c);
          local_4._0_1_ = 3;
          FUN_004f26d0(&local_24,puVar6);
        } while( true );
      }
    }
LAB_004ba2b1:
    local_14 = 0x90;
    local_10 = 0x98;
    if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      iVar1 = *(int *)((int)this + 0xc);
      local_1c = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(*(int *)((int)this + 0x10) + 0x40));
      pvVar7 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar7 != (void *)0x0) && ((*(byte *)((int)pvVar7 + 0x28) & 2) == 0)) {
        FUN_004f26d0(&local_24,(undefined4 *)(*(int *)((int)this + 0x10) + 0x40));
      }
    }
    local_14 = 0x90;
    local_10 = 0x98;
    if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x80000000,0,1,2,0,0,7
                            ,&DAT_00000002);
      pvVar7 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar7,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar7,&local_1c);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_24,puVar6);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar7);
    }
    local_14 = 0x90;
    local_10 = 0x98;
    if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x3e00000,0,1,2,0,0,
                            0x21,(void *)0x1);
      pvVar7 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar7,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar7,&local_1c);
      local_4._0_1_ = 6;
      FUN_004f26d0(&local_24,puVar6);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar7);
    }
    local_14 = 0x90;
    local_10 = 0x98;
    if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      iVar1 = *(int *)((int)this + 0xc);
      local_1c = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,(undefined4 *)((int)local_20 + 0x28));
      pvVar7 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar7 != (void *)0x0) && ((*(byte *)((int)pvVar7 + 0x30) & 4) != 0)) {
        puVar6 = FUN_0041a400(*(void **)((int)this + 0xc),
                              (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),
                              (undefined4 *)((int)this + 0x28),0,0,0,0,0,0x204,
                              (undefined4 *)&DAT_00000002);
        pvVar7 = (void *)((int)this + 0x18);
        FUN_00435790(pvVar7,(int)puVar6);
        puVar6 = FUN_004357b0(pvVar7,&local_1c);
        local_4._0_1_ = 7;
        FUN_004f26d0(&local_24,puVar6);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        FUN_005f58b0(pvVar7);
      }
    }
  }
  FUN_004f26d0(param_1,&local_24);
  local_18 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

