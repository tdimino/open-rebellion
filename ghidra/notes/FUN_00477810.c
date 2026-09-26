
void __fastcall FUN_00477810(void *param_1)

{
  int iVar1;
  bool bVar2;
  void *this;
  uint *puVar3;
  void *pvVar4;
  uint local_1c;
  undefined1 local_18 [4];
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006330a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_1c);
  local_4 = 0;
  this = (void *)thunk_FUN_005f5060((int)param_1 + 0x24);
  while (this != (void *)0x0) {
    bVar2 = false;
    puVar3 = FUN_00403040(this,local_18);
    local_4._0_1_ = 1;
    pvVar4 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x58) + 0x58),puVar3);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if (((pvVar4 != (void *)0x0) && (*(int *)((int)pvVar4 + 0x30) == *(int *)((int)param_1 + 0x18)))
       && ((*(uint *)((int)pvVar4 + 0x24) & 0x4000) != 0)) {
      bVar2 = true;
      FUN_004f26d0(&local_1c,(undefined4 *)((int)pvVar4 + 0x2c));
      pvVar4 = FUN_004f5940((void *)((int)param_1 + 0x2c),&local_1c);
      if (pvVar4 == (void *)0x0) {
        iVar1 = *(int *)((int)param_1 + 0x58);
        local_10 = &stack0xffffffd0;
        FUN_004f26d0(&stack0xffffffd0,&local_1c);
        pvVar4 = FUN_00419e40((void *)(iVar1 + 0x78));
        if ((pvVar4 != (void *)0x0) && ((*(byte *)((int)pvVar4 + 0x38) & 4) != 0)) {
          FUN_00478fd0(param_1,&local_1c);
        }
      }
    }
    if (bVar2) {
      this = *(void **)((int)this + 0x10);
    }
    else {
      pvVar4 = *(void **)((int)this + 0x10);
      puVar3 = FUN_00403040(this,local_14);
      local_4._0_1_ = 2;
      FUN_004790d0(param_1,puVar3);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      this = pvVar4;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

