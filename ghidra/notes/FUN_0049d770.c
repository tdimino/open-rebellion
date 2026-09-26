
undefined4 * __thiscall
FUN_0049d770(void *this,undefined4 *param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            undefined4 param_6)

{
  bool bVar1;
  bool bVar2;
  short sVar3;
  int *piVar4;
  uint *puVar5;
  void *pvVar6;
  int iVar7;
  void *pvVar8;
  undefined4 *puVar9;
  uint local_30;
  uint local_2c;
  undefined4 *local_28;
  void *local_24;
  void *local_20;
  short local_1c;
  short sStack_1a;
  short local_14;
  short sStack_12;
  undefined2 local_10;
  undefined2 uStack_e;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063719d;
  local_c = ExceptionList;
  local_2c = 0;
  local_28 = (undefined4 *)0x0;
  ExceptionList = &local_c;
  local_24 = this;
  FUN_004ece30(&local_30);
  local_4 = 0;
  bVar2 = false;
  FUN_004f26d0(&local_30,param_1);
  do {
    local_14 = 0x80;
    sStack_12 = 0;
    local_10 = 0x90;
    uStack_e = 0;
    if ((local_30 >> 0x18 < 0x80) || (0x8f < local_30 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (((bVar1) || (piVar4 = FUN_004f2d10(*(int *)this,&local_30), piVar4 == (int *)0x0)) ||
       ((void *)piVar4[7] == (void *)0x0)) {
      bVar2 = true;
    }
    else {
      puVar5 = FUN_004025b0((void *)piVar4[7],(uint *)&param_1);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_30,puVar5);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  } while (!bVar2);
  local_14 = 0x80;
  sStack_12 = 0;
  local_10 = 0x90;
  uStack_e = 0;
  if ((local_30 >> 0x18 < 0x80) || (0x8f < local_30 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    param_1 = (undefined4 *)&stack0xffffffbc;
    FUN_004f26d0(&stack0xffffffbc,&local_30);
    pvVar6 = FUN_0049dca0((void *)((int)this + 0x20));
    if (pvVar6 != (void *)0x0) {
      local_1c = (short)*(undefined4 *)((int)pvVar6 + 0x2e);
      sStack_1a = (short)((uint)*(undefined4 *)((int)pvVar6 + 0x2e) >> 0x10);
      param_1 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (param_1 == (undefined4 *)0x0) {
        param_1 = (undefined4 *)0x0;
      }
      else {
        param_1 = FUN_0041be80(param_1,param_6);
      }
      puVar9 = param_1;
      local_4 = (uint)local_4._1_3_ << 8;
      local_28 = param_1;
      if (param_1 != (undefined4 *)0x0) {
        pvVar6 = (void *)thunk_FUN_005f5060((int)this + 0x20);
        sVar3 = sStack_1a;
        for (; pvVar6 != (void *)0x0; pvVar6 = *(void **)((int)pvVar6 + 0x10)) {
          local_14 = (short)*(undefined4 *)((int)pvVar6 + 0x2e);
          sStack_12 = (short)((uint)*(undefined4 *)((int)pvVar6 + 0x2e) >> 0x10);
          if ((((param_2 & *(uint *)((int)pvVar6 + 0x24)) == param_2) &&
              ((param_4 & *(uint *)((int)pvVar6 + 0x24)) == 0)) &&
             (((param_3 & *(uint *)((int)pvVar6 + 0x28)) == param_3 &&
              ((param_5 & *(uint *)((int)pvVar6 + 0x28)) == 0)))) {
            iVar7 = FUN_0041b7b0(local_1c,sVar3,local_14,sStack_12);
            pvVar8 = (void *)FUN_00618b70(0x18);
            local_4._0_1_ = 3;
            local_20 = pvVar8;
            if (pvVar8 == (void *)0x0) {
              pvVar8 = (void *)0x0;
            }
            else {
              puVar9 = FUN_00403040(pvVar6,&param_6);
              local_2c = local_2c | 1;
              local_4 = CONCAT31(local_4._1_3_,4);
              pvVar8 = FUN_0041c360(pvVar8,puVar9,iVar7);
            }
            local_4 = 3;
            if ((local_2c & 1) != 0) {
              local_2c = local_2c & 0xfffffffe;
              FUN_00619730();
            }
            local_4 = local_4 & 0xffffff00;
            FUN_0041c070(param_1,pvVar8);
          }
          puVar9 = param_1;
        }
        FUN_0041c230((int)puVar9);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_28;
}

