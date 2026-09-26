
bool __fastcall FUN_004d3610(int param_1)

{
  uint *this;
  void *this_00;
  bool bVar1;
  undefined4 *puVar2;
  void *pvVar3;
  undefined4 *puVar4;
  bool bVar5;
  uint local_2c;
  undefined1 local_28 [4];
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c300;
  local_c = ExceptionList;
  this = (uint *)(param_1 + 0x54);
  ExceptionList = &local_c;
  FUN_004ece80(this);
  puVar4 = (undefined4 *)(param_1 + 0x50);
  puVar2 = FUN_0041a600(*(void **)(param_1 + 0x2c),puVar4,0x100009,2,2);
  this_00 = (void *)(param_1 + 0x40);
  FUN_00435790(this_00,(int)puVar2);
  pvVar3 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),0x100009,2,0x17,(void *)0x1);
  FUN_00435790(this_00,(int)pvVar3);
  puVar2 = FUN_004357b0(this_00,&local_2c);
  local_4 = 0;
  FUN_004f26d0(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this_00);
  local_24 = 8;
  local_20 = 0x10;
  if ((*this >> 0x18 < 8) || (local_2c = 1, 0xf < *this >> 0x18)) {
    local_2c = 0;
  }
  FUN_00619730();
  if (local_2c == 0) {
    puVar2 = FUN_0041a600(*(void **)(param_1 + 0x2c),puVar4,0x100005,2,2);
    FUN_00435790(this_00,(int)puVar2);
    pvVar3 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),0x100005,2,0x17,(void *)0x1);
    FUN_00435790(this_00,(int)pvVar3);
    puVar2 = FUN_004357b0(this_00,&local_2c);
    local_4 = 1;
    FUN_004f26d0(this,puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(this_00);
    local_24 = 8;
    local_20 = 0x10;
    if ((*this >> 0x18 < 8) || (local_2c = 1, 0xf < *this >> 0x18)) {
      local_2c = 0;
    }
    FUN_00619730();
    if (local_2c == 0) {
      puVar2 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar4,0x10000000,0,0,0,0,0,2);
      FUN_00435790(this_00,(int)puVar2);
      puVar2 = FUN_004357b0(this_00,&local_2c);
      local_4 = 2;
      FUN_004f26d0(this,puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
      pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x140000,0,1,2,0,0,0x26,&DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar2 = FUN_004357b0(this_00,&local_2c);
      local_4 = 3;
      FUN_004f26d0(this,puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
      local_24 = 0x90;
      local_20 = 0x98;
      if ((*this >> 0x18 < 0x90) || (local_2c = 1, 0x97 < *this >> 0x18)) {
        local_2c = 0;
      }
      FUN_00619730();
      bVar5 = local_2c != 0;
      if (!bVar5) {
        FUN_004ece30(&local_2c);
        local_4 = 4;
        pvVar3 = FUN_00419e90(*(void **)(param_1 + 0x2c),0,0,0x8000000,0,0,0,&DAT_00000002);
        FUN_00435790(this_00,(int)pvVar3);
        puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar4,0,0,0x8000000,0,0,0,2);
        FUN_00435790(this_00,(int)puVar4);
        puVar4 = FUN_004357b0(this_00,local_28);
        local_4._0_1_ = 5;
        FUN_004f26d0(&local_2c,puVar4);
        bVar5 = false;
        do {
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00619730();
          do {
            local_1c = 0x80;
            local_18 = 0x90;
            if (((local_2c >> 0x18 < 0x80) || (0x8f < local_2c >> 0x18)) || (bVar5)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (!bVar1) {
              FUN_005f58b0(this_00);
              local_4 = 0xffffffff;
              FUN_00619730();
              ExceptionList = local_c;
              return bVar5;
            }
            pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),&local_2c);
            if ((pvVar3 != (void *)0x0) &&
               (pvVar3 = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),
                                              *(uint *)((int)pvVar3 + 0x38)), pvVar3 != (void *)0x0)
               ) {
              puVar4 = FUN_00476f90(pvVar3,local_28);
              local_4._0_1_ = 6;
              FUN_004f26d0(this,puVar4);
              local_4 = CONCAT31(local_4._1_3_,4);
              FUN_00619730();
              local_14 = 0x90;
              local_10 = 0x98;
              if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
                bVar1 = false;
              }
              else {
                bVar1 = true;
              }
              FUN_00619730();
              if (bVar1) {
                bVar5 = true;
              }
            }
          } while (bVar5);
          puVar4 = FUN_004357b0(this_00,&local_24);
          local_4._0_1_ = 7;
          FUN_004f26d0(&local_2c,puVar4);
        } while( true );
      }
    }
    else {
      bVar5 = true;
    }
  }
  else {
    bVar5 = true;
  }
  ExceptionList = local_c;
  return bVar5;
}

