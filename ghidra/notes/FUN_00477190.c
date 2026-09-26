
void __fastcall FUN_00477190(void *param_1)

{
  uint *puVar1;
  int iVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  undefined4 *puVar6;
  int iVar7;
  uint local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633028;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  local_4 = 0;
  FUN_004fcdd0((undefined4 *)((int)param_1 + 0x60));
  FUN_004fcdd0((undefined4 *)((int)param_1 + 100));
  if (*(int *)((int)param_1 + 0x5c) != 0) {
    FUN_00619730();
    if (*(undefined4 **)((int)param_1 + 0x5c) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)param_1 + 0x5c))();
    }
    *(undefined4 *)((int)param_1 + 0x5c) = 0;
  }
  uVar4 = *(uint *)((int)param_1 + 0x34) >> 0x18;
  *(undefined4 *)((int)param_1 + 0x1c) = 0;
  local_1c = (undefined1 *)0x80;
  local_18 = 0x90;
  if ((uVar4 < 0x80) || (0x8f < uVar4)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if ((bVar3) &&
     (pvVar5 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x58) + 0x44),
                            (uint *)((int)param_1 + 0x34)), pvVar5 != (void *)0x0)) {
    FUN_004317d0(pvVar5,0);
  }
  FUN_004ece80((uint *)((int)param_1 + 0x34));
  *(undefined4 *)((int)param_1 + 0x68) = 0;
  *(undefined4 *)((int)param_1 + 0x9c) = 0;
  pvVar5 = (void *)thunk_FUN_005f5060((int)param_1 + 0x24);
  while (pvVar5 != (void *)0x0) {
    puVar6 = FUN_00403040(pvVar5,&local_1c);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_20,puVar6);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    pvVar5 = *(void **)((int)pvVar5 + 0x10);
    FUN_004790d0(param_1,&local_20);
  }
  pvVar5 = (void *)thunk_FUN_005f5060((int)param_1 + 0x2c);
  while (pvVar5 != (void *)0x0) {
    puVar6 = FUN_00403040(pvVar5,&local_1c);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_20,puVar6);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    pvVar5 = *(void **)((int)pvVar5 + 0x10);
    FUN_004791d0(param_1,&local_20);
  }
  iVar7 = thunk_FUN_005f5060((int)param_1 + 0xb0);
  while (iVar7 != 0) {
    puVar1 = (uint *)(iVar7 + 0x18);
    iVar7 = *(int *)(iVar7 + 0x10);
    FUN_0042f690((void *)(*(int *)((int)param_1 + 0x58) + 0xa8),*puVar1);
  }
  iVar7 = thunk_FUN_005f5060((int)param_1 + 0xd4);
  while (iVar7 != 0) {
    iVar2 = *(int *)(iVar7 + 0x10);
    FUN_0042ef90((void *)(*(int *)((int)param_1 + 0x58) + 0xec),*(uint *)(iVar7 + 0x18));
    iVar7 = iVar2;
  }
  local_14 = 0x90;
  puVar6 = (undefined4 *)((int)param_1 + 0x6c);
  for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
    *puVar6 = 0;
    puVar6 = puVar6 + 1;
  }
  uVar4 = *(uint *)((int)param_1 + 0x40) >> 0x18;
  local_10 = 0x98;
  if ((uVar4 < 0x90) || (0x97 < uVar4)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    iVar7 = *(int *)((int)param_1 + 0x58);
    local_1c = &stack0xffffffcc;
    FUN_004f26d0(&stack0xffffffcc,(undefined4 *)((int)param_1 + 0x40));
    pvVar5 = FUN_00403d30((void *)(iVar7 + 0x2c));
    if (pvVar5 != (void *)0x0) {
      *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) & 0xfff8ffff;
      FUN_00433e30((int)pvVar5);
      FUN_004ece80((undefined4 *)((int)param_1 + 0x40));
    }
  }
  FUN_004769f0(param_1,0);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

