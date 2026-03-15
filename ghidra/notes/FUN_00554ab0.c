
uint FUN_00554ab0(int *param_1,uint *param_2,uint *param_3,int *param_4)

{
  int *piVar1;
  bool bVar2;
  uint *puVar3;
  uint uVar4;
  uint *puVar5;
  uint uVar6;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int *this;
  void *local_34 [2];
  uint local_2c;
  void *local_28;
  uint uStack_24;
  uint uStack_20;
  uint uStack_1c;
  uint uStack_18;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649368;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 1;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_4,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar5 = param_2;
  this = (int *)0x0;
  local_34[0] = (void *)0x0;
  local_28 = (void *)0x0;
  puVar3 = FUN_004ece40(param_2);
  if (puVar3 != (uint *)0x0) {
    this = (int *)FUN_00504dc0(puVar5);
  }
  FUN_004ece30(&local_2c);
  local_4 = 1;
  uVar4 = FUN_004f6b70(param_1,&local_2c);
  if (uVar4 == 0) goto LAB_00554e50;
  if (this == (int *)0x0) {
LAB_00554b80:
    bVar2 = false;
  }
  else {
    puVar5 = FUN_004025b0(this,(uint *)&param_2);
    uVar6 = *puVar5;
    FUN_00619730();
    if (uVar6 == local_2c) goto LAB_00554b80;
    bVar2 = true;
  }
  if ((uVar4 == 0) || (!bVar2)) goto LAB_00554e50;
  FUN_004ece30(&param_2);
  local_4._0_1_ = 2;
  uVar4 = (**(code **)(*this + 0xc))(&param_2);
  if (uVar4 != 0) {
    local_14 = 0x98;
    local_10 = 0xa0;
    if (((uint)param_2 >> 0x18 < 0x98) || (0x9f < (uint)param_2 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      if (param_4[1] != -1) {
        local_14 = 0x14;
        local_10 = 0x1c;
        local_4._0_1_ = 3;
        uVar6 = (**(code **)(*this + 4))();
        if ((uVar6 < local_14) || (local_10 <= uVar6)) {
          uStack_24 = 8;
          uStack_20 = 0x10;
          local_4._0_1_ = 4;
          uVar6 = (**(code **)(*this + 4))();
          if ((uVar6 < uStack_24) || (uStack_20 <= uVar6)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          local_4._0_1_ = 3;
          FUN_00619730();
          if (bVar2) goto LAB_00554cbf;
          uStack_1c = 0x90;
          uStack_18 = 0x98;
          local_4._0_1_ = 5;
          uVar6 = (**(code **)(*this + 4))();
          if ((uVar6 < uStack_1c) || (uStack_18 <= uVar6)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          local_4._0_1_ = 3;
          FUN_00619730();
          if (bVar2) goto LAB_00554cbf;
          bVar2 = false;
        }
        else {
LAB_00554cbf:
          bVar2 = true;
        }
        local_4._0_1_ = 2;
        FUN_00619730();
        if (bVar2) {
          uVar4 = FUN_005571c0(param_1,this,0,param_3,param_4);
        }
      }
      if (uVar4 != 0) {
        if (param_4[1] != -1) {
          uStack_1c = 0x14;
          uStack_18 = 0x1c;
          local_4._0_1_ = 6;
          uVar6 = (**(code **)(*this + 4))();
          if ((uVar6 < uStack_1c) || (uStack_18 <= uVar6)) {
            local_14 = 8;
            local_10 = 0x10;
            local_4._0_1_ = 7;
            uVar6 = (**(code **)(*this + 4))();
            if ((uVar6 < local_14) || (local_10 <= uVar6)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            local_4._0_1_ = 6;
            FUN_00619730();
            if (bVar2) goto LAB_00554d75;
            bVar2 = false;
          }
          else {
LAB_00554d75:
            bVar2 = true;
          }
          local_4._0_1_ = 2;
          FUN_00619730();
          if (bVar2) {
            bVar2 = FUN_004f6b50(this,(int *)local_34);
            uVar4 = CONCAT31(extraout_var,bVar2);
            if (uVar4 == 0) goto LAB_00554e42;
            uVar4 = FUN_005571c0(param_1,local_34[0],0,param_3,param_4);
          }
        }
        piVar1 = param_4;
        if ((uVar4 != 0) && (param_4[1] != -1)) {
          local_14 = 0x14;
          local_10 = 0x1c;
          local_4._0_1_ = 8;
          uVar6 = (**(code **)(*this + 4))();
          if ((uVar6 < local_14) || (local_10 <= uVar6)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          local_4._0_1_ = 2;
          FUN_00619730();
          if (bVar2) {
            bVar2 = FUN_004f6b50(local_34[0],(int *)&local_28);
            uVar4 = CONCAT31(extraout_var_00,bVar2);
            if (uVar4 != 0) {
              uVar4 = FUN_005571c0(param_1,local_28,0,param_3,piVar1);
            }
          }
        }
      }
    }
  }
LAB_00554e42:
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
LAB_00554e50:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar4;
}

