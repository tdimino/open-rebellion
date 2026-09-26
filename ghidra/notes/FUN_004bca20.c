
int * __fastcall FUN_004bca20(int *param_1)

{
  uint *puVar1;
  bool bVar2;
  void *pvVar3;
  int iVar4;
  int *piVar5;
  void *pvVar6;
  void *pvVar7;
  uint uVar8;
  int *local_2c;
  int local_28;
  uint local_24;
  uint local_20;
  int local_1c;
  uint local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639c98;
  local_c = ExceptionList;
  local_2c = (int *)0x0;
  local_14 = (undefined1 *)0x30;
  local_10 = 0x40;
  if (((uint)param_1[0xb] >> 0x18 < 0x30) || (0x3f < (uint)param_1[0xb] >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (!bVar2) {
    ExceptionList = local_c;
    return (int *)0x0;
  }
  pvVar3 = FUN_004f5940((void *)param_1[0x10],(uint *)(param_1 + 0xb));
  if (pvVar3 == (void *)0x0) {
    ExceptionList = local_c;
    return (int *)0x0;
  }
  if ((*(uint *)((int)pvVar3 + 0x30) & 0x74000f1) != 0) {
    ExceptionList = local_c;
    return (int *)0x0;
  }
  uVar8 = param_1[10];
  puVar1 = (uint *)(param_1 + 10);
  if (*(uint *)((int)pvVar3 + 0x38) == uVar8) {
    local_14 = (undefined1 *)0x14;
    local_10 = 0x1c;
    if ((uVar8 >> 0x18 < 0x14) || (0x1b < uVar8 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      ExceptionList = local_c;
      return (int *)0x0;
    }
    if ((*(uint *)((int)pvVar3 + 0x30) & 0x7000000) != 0) {
      ExceptionList = local_c;
      return (int *)0x0;
    }
    param_1[8] = param_1[8] | 4;
    iVar4 = (**(code **)(*param_1 + 0x1c))();
    piVar5 = FUN_004bbf50(param_1,iVar4);
    ExceptionList = local_c;
    return piVar5;
  }
  local_14 = (undefined1 *)0x14;
  local_10 = 0x1c;
  if ((uVar8 >> 0x18 < 0x14) || (0x1b < uVar8 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if (((uint)param_1[0x12] >> 0x18 < 0x90) || (0x97 < (uint)param_1[0x12] >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      ExceptionList = local_c;
      return (int *)0x0;
    }
    if (*(uint *)((int)pvVar3 + 0x38) != param_1[0x12]) {
      piVar5 = FUN_004bc040(param_1,(uint *)(param_1 + 0x12));
      ExceptionList = local_c;
      return piVar5;
    }
    param_1[8] = param_1[8] | 0x10;
    ExceptionList = local_c;
    return (int *)0x0;
  }
  FUN_004ece30(&local_20);
  bVar2 = false;
  local_4 = 0;
  FUN_004ece30(&local_1c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_18);
  local_4._0_1_ = 2;
  FUN_004ece30(&local_28);
  local_4._0_1_ = 3;
  FUN_004ece30(&local_24);
  local_4 = CONCAT31(local_4._1_3_,4);
  FUN_004f26d0(&local_20,(uint *)((int)pvVar3 + 0x38));
  local_14 = (undefined1 *)0x14;
  local_10 = 0x1c;
  if ((0x13 < local_20 >> 0x18) && (local_20 >> 0x18 < 0x1c)) {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    pvVar3 = FUN_004f5940((void *)(param_1[0x11] + 0x58),&local_20);
    pvVar6 = FUN_004f5940((void *)(param_1[0x11] + 0x58),puVar1);
    if (((pvVar3 == (void *)0x0) || (pvVar6 == (void *)0x0)) ||
       ((*(uint *)((int)pvVar3 + 0x24) & 0x800) != 0)) goto LAB_004bcd88;
    FUN_004f26d0(&local_1c,(undefined4 *)((int)pvVar3 + 0x2c));
    FUN_004f26d0(&local_28,(undefined4 *)((int)pvVar6 + 0x2c));
    if (local_1c == local_28) {
      if ((*(uint *)((int)pvVar6 + 0x24) & 0x800) == 0) {
        param_1[8] = param_1[8] | 8;
        local_2c = FUN_004bc040(param_1,puVar1);
      }
      goto LAB_004bcd88;
    }
    iVar4 = param_1[0x11];
    local_14 = &stack0xffffffc0;
    FUN_004f26d0(&stack0xffffffc0,&local_1c);
    pvVar3 = FUN_00419e40((void *)(iVar4 + 0x78));
    iVar4 = param_1[0x11];
    local_14 = &stack0xffffffc0;
    FUN_004f26d0(&stack0xffffffc0,&local_28);
    pvVar7 = FUN_00419e40((void *)(iVar4 + 0x78));
    if ((pvVar3 == (void *)0x0) || (pvVar7 == (void *)0x0)) goto LAB_004bcd88;
    FUN_004f26d0(&local_18,(undefined4 *)((int)pvVar3 + 0x28));
    FUN_004f26d0(&local_24,(undefined4 *)((int)pvVar7 + 0x28));
    if (local_18 != local_24) {
      local_2c = FUN_004bc040(param_1,(uint *)(param_1 + 0x12));
      goto LAB_004bcd88;
    }
    uVar8 = *(uint *)((int)pvVar6 + 0x24);
  }
  else {
    pvVar3 = FUN_004f5940((void *)(param_1[0x11] + 0x58),puVar1);
    if (pvVar3 == (void *)0x0) goto LAB_004bcd88;
    FUN_004f26d0(&local_28,(undefined4 *)((int)pvVar3 + 0x2c));
    iVar4 = param_1[0x11];
    local_14 = &stack0xffffffc0;
    FUN_004f26d0(&stack0xffffffc0,&local_28);
    pvVar6 = FUN_00419e40((void *)(iVar4 + 0x78));
    if (pvVar6 == (void *)0x0) goto LAB_004bcd88;
    FUN_004f26d0(&local_24,(undefined4 *)((int)pvVar6 + 0x28));
    if (local_20 != local_24) {
      if (local_20 == param_1[0x12]) {
        param_1[8] = param_1[8] | 0x10;
      }
      else {
        local_2c = FUN_004bc040(param_1,(uint *)(param_1 + 0x12));
      }
      goto LAB_004bcd88;
    }
    uVar8 = *(uint *)((int)pvVar3 + 0x24);
  }
  if ((uVar8 & 0x800) == 0) {
    param_1[8] = param_1[8] | 8;
    local_2c = FUN_004bc040(param_1,(uint *)(param_1 + 10));
  }
LAB_004bcd88:
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_2c;
}

