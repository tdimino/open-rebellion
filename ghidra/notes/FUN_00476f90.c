
void * __thiscall FUN_00476f90(void *this,void *param_1)

{
  void *this_00;
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  uint local_34;
  undefined4 local_30;
  undefined1 local_2c [4];
  undefined1 *local_28;
  undefined4 *local_24;
  undefined4 local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632ff7;
  local_c = ExceptionList;
  bVar2 = false;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_34);
  local_4 = 1;
  FUN_004ece30(&local_30);
  uVar3 = *(uint *)((int)this + 0x40) >> 0x18;
  iVar6 = 0;
  local_4._0_1_ = 2;
  *(undefined4 *)((int)this + 0x9c) = 0;
  local_1c = (undefined1 *)0x90;
  local_18 = 0x98;
  if ((0x8f < uVar3) && (uVar3 < 0x98)) {
    bVar2 = true;
  }
  local_24 = (undefined4 *)((int)this + 0x40);
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)((int)this + 0x58);
    local_28 = &stack0xffffffb8;
    FUN_004f26d0(&stack0xffffffb8,(undefined4 *)((int)this + 0x40));
    local_28 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if ((local_28 != (void *)0x0) && ((*(byte *)((int)local_28 + 0x28) & 2) == 0)) {
      pvVar4 = FUN_0041acd0(*(void **)((int)this + 0x58),*(int *)((int)this + 0x18),1,0,0x17,
                            (void *)0x1);
      this_00 = (void *)((int)this + 0x48);
      FUN_00435790(this_00,(int)pvVar4);
      puVar5 = FUN_004357b0(this_00,local_2c);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_34,puVar5);
      while( true ) {
        local_4._0_1_ = 2;
        FUN_00619730();
        local_14 = 8;
        local_10 = 0x10;
        if ((local_34 >> 0x18 < 8) || (0xf < local_34 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) break;
        iVar1 = *(int *)((int)this + 0x58);
        local_1c = &stack0xffffffb8;
        FUN_004f26d0(&stack0xffffffb8,&local_34);
        pvVar4 = FUN_00419e40((void *)(iVar1 + 0x78));
        if (pvVar4 != (void *)0x0) {
          iVar6 = iVar6 + *(int *)((int)pvVar4 + 0xac);
        }
        puVar5 = FUN_004357b0(this_00,local_2c);
        local_4._0_1_ = 4;
        FUN_004f26d0(&local_34,puVar5);
      }
      FUN_005f58b0(this_00);
      iVar6 = iVar6 - *(int *)((int)local_28 + 0xa4);
      *(int *)((int)this + 0x9c) = iVar6;
      if (0 < iVar6) {
        FUN_004f26d0(&local_30,local_24);
      }
    }
  }
  FUN_004f26d0(param_1,&local_30);
  local_20 = 1;
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

