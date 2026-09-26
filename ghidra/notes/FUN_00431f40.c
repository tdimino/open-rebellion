
void * __thiscall
FUN_00431f40(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            uint param_6,void *param_7)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  void *this_00;
  undefined4 *puVar4;
  int iVar5;
  undefined4 *puVar6;
  int local_24;
  uint local_20;
  uint local_1c;
  uint local_18;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d025;
  local_c = ExceptionList;
  bVar1 = false;
  local_14 = (void *)0x0;
  ExceptionList = &local_c;
  FUN_005f4950(&local_18,0);
  local_4 = 0;
  FUN_005f4950(&local_1c,0);
  local_4._0_1_ = 1;
  FUN_005f4950(&local_20,0);
  local_4._0_1_ = 2;
  local_14 = (void *)FUN_00618b70(0x20);
  local_4._0_1_ = 3;
  if (local_14 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_0041be80(local_14,param_7);
  }
  uVar2 = param_1;
  local_4._0_1_ = 2;
  param_7 = pvVar3;
  if (pvVar3 != (void *)0x0) {
    puVar6 = (undefined4 *)((int)this + 0x48);
    local_24 = 10;
    do {
      pvVar3 = (void *)*puVar6;
      local_18 = *(uint *)((int)pvVar3 + 0x28);
      local_1c = *(uint *)((int)pvVar3 + 0x2c);
      local_20 = *(uint *)((int)pvVar3 + 0x30);
      if (((((local_18 & uVar2) == uVar2) && ((param_4 & local_18) == 0)) &&
          ((local_1c & param_2) == param_2)) &&
         ((((param_5 & local_1c) == 0 && ((local_20 & param_3) == param_3)) &&
          ((param_6 & local_20) == 0)))) {
        this_00 = (void *)FUN_00618b70(0x18);
        local_4._0_1_ = 4;
        local_10 = this_00;
        if (this_00 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          puVar4 = FUN_00403040(pvVar3,&param_1);
          bVar1 = true;
          local_14 = (void *)0x1;
          local_4 = CONCAT31(local_4._1_3_,5);
          iVar5 = FUN_0041cd80(10);
          pvVar3 = FUN_0041c360(this_00,puVar4,iVar5);
        }
        local_4 = 4;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 2;
        FUN_0041c070(param_7,pvVar3);
      }
      pvVar3 = param_7;
      puVar6 = puVar6 + 1;
      local_24 = local_24 + -1;
    } while (local_24 != 0);
    FUN_0041c230((int)param_7);
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return pvVar3;
}

