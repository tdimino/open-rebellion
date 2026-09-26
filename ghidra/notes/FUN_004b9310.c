
void __thiscall FUN_004b9310(void *this,uint *param_1)

{
  void *this_00;
  uint uVar1;
  bool bVar2;
  bool bVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint local_20;
  undefined1 local_1c [4];
  void *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639838;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = *param_1 | 0x4000000;
  local_18 = FUN_004f5940((void *)(*(int *)((int)this + 0xc) + 0x44),
                          (uint *)(*(int *)((int)this + 0x10) + 0x34));
  bVar2 = false;
  if (local_18 != (void *)0x0) {
    uVar1 = *param_1;
    *param_1 = uVar1 | 0x8000000;
    if ((*(uint *)((int)local_18 + 0x2c) & 0x40000) == 0) {
      FUN_004ece30(&local_20);
      local_4 = 0;
      pvVar4 = FUN_0041a3a0(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0,0x20,0,0,0x700201,
                            (void *)0x1);
      this_00 = (void *)((int)this + 0x18);
      FUN_00435790(this_00,(int)pvVar4);
      puVar5 = FUN_004357b0(this_00,local_1c);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_20,puVar5);
      while( true ) {
        local_4._0_1_ = 0;
        FUN_00619730();
        local_14 = 0x90;
        local_10 = 0x98;
        if (((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) || (bVar2)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        if (!bVar3) break;
        pvVar4 = FUN_004f5940((void *)((int)this + 0x5c),&local_20);
        if (pvVar4 == (void *)0x0) {
          bVar2 = true;
        }
        puVar5 = FUN_004357b0(this_00,local_1c);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_20,puVar5);
      }
      FUN_005f58b0(this_00);
      if (bVar2) {
        *param_1 = *param_1 | 0x20000;
      }
      else {
        *(uint *)((int)local_18 + 0x2c) = *(uint *)((int)local_18 + 0x2c) | 0x20000000;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    else {
      *param_1 = uVar1 | 0x8020000;
    }
  }
  ExceptionList = local_c;
  return;
}

