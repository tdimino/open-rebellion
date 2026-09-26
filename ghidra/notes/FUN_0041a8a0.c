
undefined4 * __thiscall
FUN_0041a8a0(void *this,undefined4 *param_1,uint param_2,uint param_3,undefined4 param_4)

{
  bool bVar1;
  bool bVar2;
  short sVar3;
  int *piVar4;
  uint *puVar5;
  void *pvVar6;
  undefined4 *puVar7;
  int iVar8;
  void *pvVar9;
  uint local_34;
  uint local_30;
  undefined4 *local_2c;
  undefined1 local_28 [4];
  undefined4 local_24;
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
  puStack_8 = &LAB_0062b475;
  local_c = ExceptionList;
  bVar2 = false;
  local_30 = 0;
  local_2c = (undefined4 *)0x0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_34);
  local_4 = 0;
  FUN_004f26d0(&local_34,param_1);
  do {
    local_14 = 0x90;
    sStack_12 = 0;
    local_10 = 0x98;
    uStack_e = 0;
    if ((local_34 >> 0x18 < 0x90) || (0x97 < local_34 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (((bVar1) || (piVar4 = FUN_004f2d10(*(int *)this,&local_34), piVar4 == (int *)0x0)) ||
       ((void *)piVar4[7] == (void *)0x0)) {
      bVar2 = true;
    }
    else {
      puVar5 = FUN_004025b0((void *)piVar4[7],(uint *)&param_1);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_34,puVar5);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  } while (!bVar2);
  local_14 = 0x90;
  sStack_12 = 0;
  local_10 = 0x98;
  uStack_e = 0;
  if ((local_34 >> 0x18 < 0x90) || (0x97 < local_34 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    param_1 = (undefined4 *)&stack0xffffffb8;
    FUN_004f26d0(&stack0xffffffb8,&local_34);
    pvVar6 = FUN_00403d30((void *)((int)this + 0x2c));
    if (pvVar6 != (void *)0x0) {
      local_1c = (short)*(undefined4 *)((int)pvVar6 + 0x34);
      sStack_1a = (short)((uint)*(undefined4 *)((int)pvVar6 + 0x34) >> 0x10);
      param_1 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (param_1 == (undefined4 *)0x0) {
        param_1 = (undefined4 *)0x0;
      }
      else {
        param_1 = FUN_0041be80(param_1,param_4);
      }
      puVar7 = param_1;
      local_4 = (uint)local_4._1_3_ << 8;
      local_2c = param_1;
      if (param_1 != (undefined4 *)0x0) {
        pvVar6 = (void *)thunk_FUN_005f5060((int)this + 0x78);
        for (; pvVar6 != (void *)0x0; pvVar6 = *(void **)((int)pvVar6 + 0x10)) {
          if (((param_2 & *(uint *)((int)pvVar6 + 0x38)) == param_2) &&
             ((param_3 & *(uint *)((int)pvVar6 + 0x38)) == 0)) {
            puVar7 = FUN_00403040(pvVar6,&param_4);
            local_4._0_1_ = 3;
            puVar7 = FUN_0041b640(&local_24,puVar7);
            local_14 = (short)*puVar7;
            sVar3 = local_14;
            sStack_12 = (short)((uint)*puVar7 >> 0x10);
            local_4._0_1_ = 0;
            FUN_00619730();
            iVar8 = FUN_0041b7b0(local_1c,sStack_1a,sVar3,sStack_12);
            pvVar9 = (void *)FUN_00618b70(0x18);
            local_4._0_1_ = 4;
            local_20 = pvVar9;
            if (pvVar9 == (void *)0x0) {
              pvVar9 = (void *)0x0;
            }
            else {
              puVar7 = FUN_00403040(pvVar6,local_28);
              local_30 = local_30 | 1;
              local_4 = CONCAT31(local_4._1_3_,5);
              pvVar9 = FUN_0041c360(pvVar9,puVar7,iVar8);
            }
            local_4 = 4;
            if ((local_30 & 1) != 0) {
              local_30 = local_30 & 0xfffffffe;
              FUN_00619730();
            }
            local_4 = local_4 & 0xffffff00;
            FUN_0041c070(param_1,pvVar9);
          }
          puVar7 = param_1;
        }
        FUN_0041c230((int)puVar7);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_2c;
}

