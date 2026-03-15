
int __fastcall FUN_004b04c0(int param_1)

{
  void *this;
  undefined4 *puVar1;
  void *this_00;
  int *piVar2;
  int iVar3;
  bool bVar4;
  int local_2c;
  int local_28;
  uint *local_24;
  undefined1 local_20 [4];
  undefined1 local_1c [4];
  undefined1 local_18 [4];
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638d98;
  local_c = ExceptionList;
  local_28 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_2c);
  local_24 = (uint *)(param_1 + 0x2c);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  this = FUN_004f5940((void *)(*(int *)(param_1 + 0x10) + 0x2c),local_24);
  iVar3 = 0;
  if ((this != (void *)0x0) ||
     (this = (void *)thunk_FUN_005f5060(*(int *)(param_1 + 0x10) + 0x2c), this != (void *)0x0)) {
    puVar1 = (undefined4 *)FUN_00403040(this,local_20);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_2c,puVar1);
    local_4._0_1_ = 0;
    FUN_00619730();
joined_r0x004b054b:
    while (this != (void *)0x0) {
      if (iVar3 != 0) goto LAB_004b064f;
      puVar1 = (undefined4 *)FUN_00403040(this,local_20);
      iVar3 = *(int *)(param_1 + 0xc);
      local_10 = &stack0xffffffc0;
      local_4._0_1_ = 2;
      FUN_004f26d0(&stack0xffffffc0,puVar1);
      this_00 = FUN_00419e40((void *)(iVar3 + 0x78));
      local_4._0_1_ = 0;
      FUN_00619730();
      if ((this_00 != (void *)0x0) && ((*(uint *)((int)this_00 + 0x38) & 0xf0003802) == 0)) {
        puVar1 = (undefined4 *)FUN_00403040(this_00,local_1c);
        local_4._0_1_ = 3;
        FUN_004f26d0((void *)(param_1 + 0x28),puVar1);
        local_4._0_1_ = 0;
        FUN_00619730();
        local_28 = 1;
      }
      this = *(void **)((int)this + 0x10);
      if (this == (void *)0x0) {
        this = (void *)thunk_FUN_005f5060(*(int *)(param_1 + 0x10) + 0x2c);
        iVar3 = local_28;
        if (this != (void *)0x0) goto code_r0x004b0615;
        break;
      }
      piVar2 = (int *)FUN_00403040(this,local_18);
      bVar4 = *piVar2 == local_2c;
      FUN_00619730();
      iVar3 = local_28;
      if (bVar4) goto LAB_004b063d;
    }
  }
  if (iVar3 == 0) {
LAB_004b067b:
    if (this != (void *)0x0) goto LAB_004b0688;
  }
  else {
LAB_004b064f:
    if (this != (void *)0x0) {
      puVar1 = (undefined4 *)FUN_00403040(this,local_14);
      local_4._0_1_ = 4;
      FUN_004f26d0(local_24,puVar1);
      local_4._0_1_ = 0;
      FUN_00619730();
      goto LAB_004b067b;
    }
  }
  FUN_004ece80(local_24);
LAB_004b0688:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar3;
code_r0x004b0615:
  piVar2 = (int *)FUN_00403040(this,local_14);
  bVar4 = *piVar2 == local_2c;
  FUN_00619730();
  iVar3 = local_28;
  if (bVar4) {
LAB_004b063d:
    this = (void *)0x0;
    iVar3 = local_28;
  }
  goto joined_r0x004b054b;
}

