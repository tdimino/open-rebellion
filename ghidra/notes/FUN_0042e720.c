
void * FUN_0042e720(void *param_1,uint param_2,uint param_3,undefined4 param_4,void *param_5)

{
  uint uVar1;
  int iVar2;
  void *pvVar3;
  uint *puVar4;
  void *this;
  int *piVar5;
  undefined4 uVar6;
  uint uVar7;
  uint local_28;
  void *local_24;
  uint local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c95d;
  pvStack_c = ExceptionList;
  uVar7 = 0;
  local_20 = 0;
  ExceptionList = &pvStack_c;
  FUN_0052d720(local_1c);
  local_4 = 0;
  FUN_004ece30(&local_28);
  local_4._0_1_ = 1;
  if ((param_1 != (void *)0x0) && (iVar2 = FUN_0052e580(param_1,param_2,local_1c), iVar2 != 0)) {
    pvVar3 = (void *)thunk_FUN_005f5060((int)local_1c);
    uVar1 = param_3;
    while (pvVar3 != (void *)0x0) {
      puVar4 = FUN_0042ec10(pvVar3,&param_1);
      local_4._0_1_ = 2;
      iVar2 = FUN_0042e190(local_24,puVar4);
      local_4._0_1_ = 1;
      FUN_00619730();
      if ((iVar2 == 0) || ((*(uint *)(iVar2 + 0x18) & uVar1) != uVar1)) {
        puVar4 = (uint *)((int)pvVar3 + 0x18);
        pvVar3 = *(void **)((int)pvVar3 + 0x10);
        FUN_005f54a0(local_1c,*puVar4);
        uVar7 = local_20;
      }
      else {
        pvVar3 = *(void **)((int)pvVar3 + 0x10);
        uVar7 = local_20;
      }
    }
  }
  param_1 = (void *)FUN_00618b70(0x20);
  local_4._0_1_ = 3;
  if (param_1 == (void *)0x0) {
    param_1 = (void *)0x0;
  }
  else {
    param_1 = FUN_0041be80(param_1,param_5);
  }
  pvVar3 = param_1;
  local_4._0_1_ = 1;
  if (param_1 != (void *)0x0) {
    this = (void *)thunk_FUN_005f5060((int)local_1c);
    for (; this != (void *)0x0; this = *(void **)((int)this + 0x10)) {
      puVar4 = FUN_0042ec10(this,&param_3);
      local_4._0_1_ = 4;
      piVar5 = (int *)FUN_0042e190(local_24,puVar4);
      local_4._0_1_ = 1;
      FUN_00619730();
      if (piVar5 != (int *)0x0) {
        local_28 = (local_28 ^ piVar5[0xb]) & 0xffffff ^ piVar5[0xb];
        local_28 = local_28 ^ (local_28 ^ piVar5[0xb]) & 0xffffff;
        pvVar3 = (void *)FUN_00618b70(0x18);
        local_4._0_1_ = 5;
        param_5 = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          FUN_005f4950(&param_2,param_4);
          uVar7 = uVar7 | 1;
          local_4 = CONCAT31(local_4._1_3_,6);
          local_20 = uVar7;
          uVar6 = (**(code **)(*piVar5 + 4))(&param_2);
          pvVar3 = FUN_0041c360(pvVar3,&local_28,uVar6);
        }
        local_4 = 5;
        if ((uVar7 & 1) != 0) {
          uVar7 = uVar7 & 0xfffffffe;
          FUN_00619730();
        }
        local_4._0_1_ = 1;
        if (pvVar3 != (void *)0x0) {
          FUN_0041c070(param_1,pvVar3);
        }
      }
      pvVar3 = param_1;
    }
    FUN_0041c230((int)pvVar3);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_0052d760(local_1c);
  ExceptionList = pvStack_c;
  return pvVar3;
}

