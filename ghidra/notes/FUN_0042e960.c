
undefined4 __thiscall
FUN_0042e960(void *this,uint *param_1,int param_2,int param_3,uint param_4,undefined4 param_5,
            void *param_6)

{
  bool bVar1;
  undefined4 uVar2;
  int *piVar3;
  void *pvVar4;
  undefined4 uVar5;
  undefined4 *puVar6;
  uint *puVar7;
  uint local_30;
  undefined4 local_2c;
  void *local_28;
  void *local_24;
  void *local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c9bd;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_28 = (void *)0x0;
  local_2c = 0;
  ExceptionList = &pvStack_c;
  local_24 = this;
  FUN_004355d0(local_1c);
  local_4 = 0;
  FUN_004ece30(&local_30);
  puVar7 = param_1;
  local_4._0_1_ = 1;
  FUN_004fcdd0(param_1);
  local_28 = (void *)FUN_00618b70(0x20);
  local_4._0_1_ = 2;
  if (local_28 == (void *)0x0) {
    param_6 = (void *)0x0;
  }
  else {
    param_6 = FUN_0041be80(local_28,param_6);
  }
  uVar2 = param_5;
  local_4._0_1_ = 1;
  if (param_6 != (void *)0x0) {
    piVar3 = (int *)FUN_005f5060((int)this);
    pvVar4 = param_6;
    for (; param_6 = pvVar4, piVar3 != (int *)0x0; piVar3 = (int *)piVar3[4]) {
      if (((piVar3[10] == param_2) && ((param_4 & piVar3[6]) != 0)) && (piVar3[9] <= param_3)) {
        local_30 = (local_30 ^ piVar3[0xb]) & 0xffffff ^ piVar3[0xb];
        local_30 = local_30 ^ (local_30 ^ piVar3[0xb]) & 0xffffff;
        pvVar4 = (void *)FUN_00618b70(0x18);
        local_4._0_1_ = 3;
        local_20 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          FUN_005f4950(&param_5,uVar2);
          bVar1 = true;
          local_28 = (void *)0x1;
          local_4 = CONCAT31(local_4._1_3_,4);
          uVar5 = (**(code **)(*piVar3 + 4))(&param_5);
          pvVar4 = FUN_0041c360(pvVar4,&local_30,uVar5);
        }
        local_4 = 3;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 1;
        if (pvVar4 != (void *)0x0) {
          FUN_0041c070(param_6,pvVar4);
        }
      }
      puVar7 = param_1;
      pvVar4 = param_6;
    }
    FUN_0041c230((int)pvVar4);
    FUN_00435790(local_1c,(int)pvVar4);
    puVar6 = FUN_004357b0(local_1c,&param_1);
    local_4._0_1_ = 5;
    FUN_004f26d0(&local_30,puVar6);
    local_4._0_1_ = 1;
    FUN_00619730();
    *puVar7 = local_30;
    this = local_24;
  }
  if ((*puVar7 & 0xff000000) != 0) {
    piVar3 = (int *)FUN_0042e190(this,puVar7);
    FUN_005f4950(&param_1,uVar2);
    local_4._0_1_ = 6;
    local_2c = (**(code **)(*piVar3 + 4))(&param_1);
    local_4._0_1_ = 1;
    FUN_00619730();
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00435610(local_1c);
  ExceptionList = pvStack_c;
  return local_2c;
}

