
void * __thiscall
FUN_00431830(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            uint param_6,uint param_7,void *param_8)

{
  undefined4 uVar1;
  bool bVar2;
  void *this_00;
  undefined4 *puVar3;
  void *pvVar4;
  undefined4 *local_2c;
  int local_28;
  uint local_24;
  uint local_20;
  uint local_1c [2];
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cf25;
  local_c = ExceptionList;
  bVar2 = false;
  local_1c[1] = 0;
  ExceptionList = &local_c;
  FUN_005f4950(local_1c,0);
  local_4 = 0;
  FUN_005f4950(&local_20,0);
  local_4._0_1_ = 1;
  FUN_005f4950(&local_24,0);
  local_4._0_1_ = 2;
  local_14 = (void *)FUN_00618b70(0x20);
  local_4._0_1_ = 3;
  if (local_14 == (void *)0x0) {
    param_8 = (void *)0x0;
  }
  else {
    param_8 = FUN_0041be80(local_14,param_8);
  }
  local_4._0_1_ = 2;
  if (param_8 != (void *)0x0) {
    local_2c = (undefined4 *)((int)this + 0x48);
    local_28 = 10;
    do {
      pvVar4 = (void *)*local_2c;
      local_1c[0] = *(uint *)((int)pvVar4 + 0x28);
      local_20 = *(uint *)((int)pvVar4 + 0x2c);
      local_24 = *(uint *)((int)pvVar4 + 0x30);
      if (((((local_1c[0] & param_1) == param_1) && ((param_4 & local_1c[0]) == 0)) &&
          ((local_20 & param_2) == param_2)) &&
         ((((param_5 & local_20) == 0 && ((local_24 & param_3) == param_3)) &&
          ((param_6 & local_24) == 0)))) {
        this_00 = (void *)FUN_00618b70(0x18);
        local_4._0_1_ = 4;
        local_10 = this_00;
        if (this_00 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          uVar1 = *(undefined4 *)((int)pvVar4 + (param_7 & 0xffff) * 4 + 0x48);
          puVar3 = (undefined4 *)FUN_00403040(pvVar4,&local_14);
          bVar2 = true;
          local_1c[1] = 1;
          local_4 = CONCAT31(local_4._1_3_,5);
          pvVar4 = FUN_0041c360(this_00,puVar3,uVar1);
        }
        local_4 = 4;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 2;
        FUN_0041c070(param_8,pvVar4);
      }
      local_2c = local_2c + 1;
      local_28 = local_28 + -1;
    } while (local_28 != 0);
    FUN_0041c230((int)param_8);
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return param_8;
}

