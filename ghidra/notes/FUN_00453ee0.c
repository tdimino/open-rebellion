
void __thiscall FUN_00453ee0(void *this,void *param_1)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  void *this_00;
  undefined4 *puVar4;
  int *piVar5;
  undefined4 *puVar6;
  int iVar7;
  bool bVar8;
  int local_34;
  void *local_30;
  undefined1 local_2c [4];
  undefined4 local_28;
  undefined1 local_24 [4];
  void *local_20;
  undefined4 local_1c [2];
  int *local_14;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fead;
  local_c = ExceptionList;
  bVar1 = false;
  local_28 = 0;
  local_30 = this;
  if (*(int *)((int)this + 0x160) == 0x67) {
    puVar4 = (undefined4 *)((int)this + 0x1b4);
    iVar7 = 3;
    ExceptionList = &local_c;
    do {
      if ((*(byte *)(puVar4 + 7) & 1) != 0) {
        puVar6 = (undefined4 *)FUN_00618b70(0x20);
        local_4 = 4;
        if (puVar6 == (undefined4 *)0x0) {
          puVar6 = (undefined4 *)0x0;
        }
        else {
          puVar6 = FUN_004f5ad0(puVar6);
        }
        local_4 = 0xffffffff;
        FUN_004f26d0(puVar6 + 6,puVar4);
        FUN_004f57b0(param_1,puVar6);
      }
      puVar4 = puVar4 + 0xe;
      iVar7 = iVar7 + -1;
    } while (iVar7 != 0);
  }
  else {
    ExceptionList = &local_c;
    FUN_0060a790(local_1c,1);
    local_4 = 0;
    if (*(void **)((int)this + 0x174) != (void *)0x0) {
      FUN_00609410(*(void **)((int)this + 0x174),local_1c);
      FUN_004ece30(&local_34);
      local_4._0_1_ = 1;
      piVar5 = local_14;
      while (piVar5 != (int *)0x0) {
        pvVar2 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),piVar5[3]);
        piVar3 = FUN_0042d170(pvVar2,local_2c);
        bVar8 = local_34 != *piVar3;
        FUN_00619730();
        if (bVar8) {
          this_00 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 2;
          local_20 = this_00;
          if (this_00 == (void *)0x0) {
            pvVar2 = (void *)0x0;
          }
          else {
            puVar4 = FUN_0042d170(pvVar2,local_24);
            bVar1 = true;
            local_28 = 1;
            local_4 = CONCAT31(local_4._1_3_,3);
            pvVar2 = FUN_004f5b10(this_00,puVar4,0);
          }
          local_4 = 2;
          if (bVar1) {
            bVar1 = false;
            FUN_00619730();
          }
          local_4._0_1_ = 1;
          FUN_004f57b0(param_1,pvVar2);
        }
        piVar5 = (int *)(**(code **)(*piVar5 + 0xc))();
        this = local_30;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    FUN_0060a810(local_1c);
  }
  ExceptionList = local_c;
  return;
}

