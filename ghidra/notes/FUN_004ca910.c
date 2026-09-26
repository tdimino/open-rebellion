
undefined4 __fastcall FUN_004ca910(int param_1)

{
  void *this;
  uint *puVar1;
  uint uVar2;
  int iVar3;
  undefined4 *puVar4;
  uint local_20;
  int local_1c;
  undefined4 local_18;
  undefined1 local_14 [4];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b238;
  local_c = ExceptionList;
  local_18 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  local_4 = 0;
  *(uint *)(param_1 + 0x28) = *(uint *)(param_1 + 0x28) & 0x80ffffff;
  local_1c = 0;
  *(undefined4 *)(param_1 + 0x6c) = 0;
  local_10 = FUN_004cc220(param_1);
  if (local_10 < 1) {
    if (local_10 < 0) {
      local_18 = 10;
    }
  }
  else {
    iVar3 = FUN_0041b270(*(void **)(param_1 + 0x2c),0x28,*(uint *)(param_1 + 0x60),
                         *(undefined4 *)(param_1 + 100),(void *)0x1);
    this = (void *)(param_1 + 0x40);
    FUN_00435790(this,iVar3);
    puVar4 = FUN_004357b0(this,local_14);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_20,puVar4);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    puVar1 = (uint *)(param_1 + 0x5c);
    *puVar1 = (*(uint *)(param_1 + 0x5c) ^ local_20) & 0xffffff ^ local_20;
    *puVar1 = local_20;
    uVar2 = local_20;
    while (((uVar2 & 0xff000000) != 0 && (local_1c == 0))) {
      iVar3 = FUN_0042e190((void *)(*(int *)(param_1 + 0x2c) + 0x138),puVar1);
      if ((iVar3 != 0) && (*(int *)(iVar3 + 0x20) <= local_10)) {
        *(undefined4 *)(param_1 + 0x6c) = 1;
        local_1c = 1;
      }
      if (local_1c == 0) {
        puVar4 = FUN_004357b0(this,local_14);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_20,puVar4);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        *puVar1 = (*puVar1 ^ local_20) & 0xffffff ^ local_20;
        *puVar1 = local_20;
      }
      uVar2 = *puVar1;
    }
    FUN_005f58b0(this);
    if (*(int *)(param_1 + 0x6c) < 1) {
      *(uint *)(param_1 + 0x28) = *(uint *)(param_1 + 0x28) | *(uint *)(param_1 + 0x68);
    }
    else {
      FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x2c) + 0x24));
      local_18 = 0xf;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_18;
}

