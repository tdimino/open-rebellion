
undefined4 * FUN_0041b640(undefined4 *param_1,undefined4 *param_2)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint *puVar4;
  void *pvVar5;
  undefined4 uVar6;
  uint local_20;
  int *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b5e0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  local_4 = 0;
  bVar2 = false;
  local_18 = 0;
  FUN_004f26d0(&local_20,param_2);
  do {
    local_14 = 0x90;
    local_10 = 0x98;
    if ((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (((bVar1) || (piVar3 = FUN_004f2d10(*local_1c,&local_20), piVar3 == (int *)0x0)) ||
       ((void *)piVar3[7] == (void *)0x0)) {
      bVar2 = true;
    }
    else {
      puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_2);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_20,puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  } while (!bVar2);
  local_14 = 0x90;
  local_10 = 0x98;
  if ((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  uVar6 = local_18;
  if (bVar2) {
    param_2 = (undefined4 *)&stack0xffffffcc;
    FUN_004f26d0(&stack0xffffffcc,&local_20);
    pvVar5 = FUN_00403d30(local_1c + 0xb);
    uVar6 = local_18;
    if (pvVar5 != (void *)0x0) {
      uVar6 = *(undefined4 *)((int)pvVar5 + 0x34);
    }
  }
  local_4 = 0xffffffff;
  *param_1 = uVar6;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

