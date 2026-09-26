
undefined4 * __thiscall
FUN_004c6460(void *this,undefined4 *param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            undefined4 *param_6)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  int *piVar4;
  uint *puVar5;
  void *pvVar6;
  int iVar7;
  void *this_00;
  undefined4 *puVar8;
  uint local_3c;
  uint local_38;
  int local_34;
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
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ab5d;
  local_c = ExceptionList;
  local_38 = 0;
  local_28 = (undefined4 *)0x0;
  ExceptionList = &local_c;
  local_24 = this;
  FUN_005f4950(&local_2c,0);
  local_4 = 0;
  FUN_005f4950(&local_30,0);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_3c);
  local_4._0_1_ = 2;
  bVar2 = false;
  FUN_004f26d0(&local_3c,param_1);
  do {
    local_14 = 0x90;
    sStack_12 = 0;
    local_10 = 0x98;
    uStack_e = 0;
    if ((local_3c >> 0x18 < 0x90) || (0x97 < local_3c >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
LAB_004c6547:
      bVar2 = true;
    }
    else {
      piVar4 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_3c);
      if ((piVar4 == (int *)0x0) || ((void *)piVar4[7] == (void *)0x0)) goto LAB_004c6547;
      puVar5 = FUN_004025b0((void *)piVar4[7],(uint *)&param_1);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_3c,puVar5);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    if (bVar2) {
      local_14 = 0x90;
      sStack_12 = 0;
      local_10 = 0x98;
      uStack_e = 0;
      if ((local_3c >> 0x18 < 0x90) || (0x97 < local_3c >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        iVar7 = *(int *)((int)this + 0x34);
        param_1 = (undefined4 *)&stack0xffffffb0;
        FUN_004f26d0(&stack0xffffffb0,&local_3c);
        pvVar6 = FUN_004c5dd0((void *)(iVar7 + 8));
        if (pvVar6 != (void *)0x0) {
          local_1c = (short)*(undefined4 *)((int)pvVar6 + 0x2c);
          sStack_1a = (short)((uint)*(undefined4 *)((int)pvVar6 + 0x2c) >> 0x10);
          param_1 = (undefined4 *)FUN_00618b70(0x20);
          local_4._0_1_ = 4;
          if (param_1 == (undefined4 *)0x0) {
            param_1 = (undefined4 *)0x0;
          }
          else {
            param_1 = FUN_0041be80(param_1,param_6);
          }
          uVar3 = param_2;
          local_4._0_1_ = 2;
          local_28 = param_1;
          if (param_1 != (undefined4 *)0x0) {
            param_6 = (undefined4 *)((int)this + 0x38);
            local_34 = 10;
            do {
              pvVar6 = (void *)*param_6;
              local_2c = *(uint *)((int)pvVar6 + 0x24);
              local_30 = *(uint *)((int)pvVar6 + 0x28);
              local_14 = (short)*(undefined4 *)((int)pvVar6 + 0x2c);
              sStack_12 = (short)((uint)*(undefined4 *)((int)pvVar6 + 0x2c) >> 0x10);
              if (((((local_2c & uVar3) == uVar3) && ((param_4 & local_2c) == 0)) &&
                  ((local_30 & param_3) == param_3)) && ((param_5 & local_30) == 0)) {
                iVar7 = FUN_0041b7b0(local_1c,sStack_1a,local_14,sStack_12);
                this_00 = (void *)FUN_00618b70(0x18);
                local_4._0_1_ = 5;
                local_20 = this_00;
                if (this_00 == (void *)0x0) {
                  pvVar6 = (void *)0x0;
                }
                else {
                  puVar8 = FUN_00403040(pvVar6,&param_2);
                  local_38 = local_38 | 1;
                  local_4 = CONCAT31(local_4._1_3_,6);
                  pvVar6 = FUN_0041c360(this_00,puVar8,iVar7);
                }
                local_4 = 5;
                if ((local_38 & 1) != 0) {
                  local_38 = local_38 & 0xfffffffe;
                  FUN_00619730();
                }
                local_4._0_1_ = 2;
                FUN_0041c070(param_1,pvVar6);
              }
              param_6 = param_6 + 1;
              local_34 = local_34 + -1;
            } while (local_34 != 0);
            FUN_0041c230((int)param_1);
          }
        }
      }
      local_4._0_1_ = 1;
      FUN_00619730();
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return local_28;
    }
  } while( true );
}

