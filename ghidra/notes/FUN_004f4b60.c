
uint __thiscall FUN_004f4b60(void *this,int *param_1)

{
  bool bVar1;
  int *this_00;
  undefined1 uVar2;
  int iVar3;
  void *pvVar4;
  uint *puVar5;
  uint uVar6;
  int *piVar7;
  uint uVar8;
  undefined4 *puVar9;
  uint uStack_74;
  uint uStack_70;
  undefined4 uStack_6c;
  int *local_68;
  uint local_64 [6];
  undefined4 auStack_4c [7];
  undefined4 auStack_30 [9];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_0063f898;
  pvStack_c = ExceptionList;
  local_64[0] = 1;
  local_64[1] = 0xffffffff;
  uVar6 = *(uint *)((int)this + 0x1c);
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,local_64);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00541e70(local_64 + 2);
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  FUN_0051fcb0(this,local_64 + 2);
  if (uVar6 != 0) {
                    /* WARNING: Load size is inaccurate */
    iVar3 = (**(code **)(*this + 0x28))();
    pvVar4 = (void *)thunk_FUN_005f5060(iVar3);
    if ((pvVar4 == (void *)0x0) &&
       (pvVar4 = (void *)thunk_FUN_005f5060((int)this + 0x58), pvVar4 == (void *)0x0)) {
      local_64[0] = 1;
      local_64[1] = 0x16;
      local_4._0_1_ = 2;
      FUN_00520580(this_00,local_64);
      local_4._0_1_ = 1;
      FUN_00619730();
    }
    if (uVar6 != 0) {
      if (this_00[1] == -1) {
        uStack_74 = 0;
        local_64[0] = 1;
        local_64[1] = 0x12;
        local_4._0_1_ = 3;
        puVar5 = (uint *)FUN_00403040(pvVar4,&local_68);
        local_4._0_1_ = 4;
        uVar6 = FUN_00504e60(puVar5,&uStack_74,local_64,this_00);
        local_4._0_1_ = 3;
        FUN_00619730();
        local_4._0_1_ = 1;
        FUN_00619730();
        if (uVar6 == 0) goto LAB_004f4d96;
        if (this_00[1] == -1) {
          uStack_74 = *(uint *)(uStack_74 + 0x24) >> 6 & 3;
        }
      }
      if ((uVar6 != 0) && (this_00[1] == -1)) {
        piVar7 = (int *)FUN_0042dbe0(&local_68);
        local_64[0] = (uint)(*piVar7 == *(int *)((int)this + 0x4c));
        FUN_00619730();
        if (local_64[0] == 0) {
          iVar3 = FUN_0051cab0((uint *)((int)this + 0x4c));
          if (iVar3 == 0) {
            local_64[0] = 0x40;
            local_64[1] = 0x93;
            local_4._0_1_ = 6;
            FUN_00520580(this_00,local_64);
            local_4._0_1_ = 1;
            FUN_00619730();
            uVar6 = 0;
          }
        }
        else {
          local_64[0] = 0x40;
          local_64[1] = 0x92;
          local_4._0_1_ = 5;
          FUN_00520580(this_00,local_64);
          local_4._0_1_ = 1;
          FUN_00619730();
        }
      }
    }
  }
LAB_004f4d96:
  local_68 = (int *)0x0;
  piVar7 = param_1;
  if ((uVar6 != 0) && (this_00[1] == -1)) {
    if (uStack_74 == 1) {
      piVar7 = (int *)0x1;
    }
    else if (uStack_74 == 2) {
      piVar7 = (int *)&DAT_00000002;
    }
    else {
      local_64[0] = 1;
      local_64[1] = 0x14;
      local_4._0_1_ = 7;
      FUN_00520580(this_00,local_64);
      local_4._0_1_ = 1;
      FUN_00619730();
      piVar7 = (int *)0x0;
    }
  }
  if ((uVar6 != 0) && (this_00[1] == -1)) {
    puVar5 = (uint *)FUN_0053ca30(this,local_64);
    local_4._0_1_ = 8;
    piVar7 = FUN_004f2d10((int)piVar7,puVar5);
    local_4._0_1_ = 1;
    local_68 = piVar7;
    FUN_00619730();
    if (piVar7 == (int *)0x0) {
      local_64[0] = 0x40;
      local_64[1] = 0x20;
      local_4._0_1_ = 9;
      FUN_00520580(this_00,local_64);
      local_4._0_1_ = 1;
      FUN_00619730();
    }
  }
  param_1 = (int *)0x0;
  if (uVar6 != 0) {
    if (this_00[1] == -1) {
      pvVar4 = (void *)thunk_FUN_00506e40();
      uVar6 = (uint)(pvVar4 != (void *)0x0);
      if (uVar6 == 0) goto LAB_004f4f60;
      uVar6 = FUN_00543410(pvVar4,uStack_74,(uint *)((int)this + 0x4c),local_68,&param_1,this_00);
    }
    if ((uVar6 != 0) && (this_00[1] == -1)) {
      local_64[0] = 0x40;
      uVar8 = *(uint *)((int)this + 0x4c) >> 0x18;
      local_64[1] = 0x50;
      if ((uVar8 < 0x40) || (0x4f < uVar8)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if ((bVar1) && (((uint)param_1[9] >> 6 & 3) != uStack_74)) {
        local_64[0] = 1;
        local_64[1] = 0x28;
        local_4._0_1_ = 10;
        FUN_00520580(this_00,local_64);
        local_4._0_1_ = 1;
        FUN_00619730();
      }
    }
  }
LAB_004f4f60:
  FUN_004ece30(&uStack_70);
  local_4._0_1_ = 0xb;
  FUN_00551060(auStack_30);
  local_4._0_1_ = 0xc;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 0xc;
  if (uVar6 != 0) {
    if (this_00[1] == -1) {
      local_64[0] = 3;
      uVar6 = FUN_00551190(auStack_30,(int *)(local_64 + 2),uStack_74,local_64);
      uVar2 = (undefined1)local_4;
      if (uVar6 == 0) goto LAB_004f508a;
      puVar5 = FUN_004025b0(param_1,(uint *)&local_68);
      local_4._0_1_ = 0xd;
      uVar6 = FUN_005513a0(auStack_30,puVar5);
      local_4._0_1_ = 0xc;
      FUN_00619730();
      uVar2 = (undefined1)local_4;
    }
    local_4._0_1_ = uVar2;
    uVar2 = (undefined1)local_4;
    if ((uVar6 != 0) && (this_00[1] == -1)) {
                    /* WARNING: Load size is inaccurate */
      iVar3 = (**(code **)(*this + 0x28))();
      FUN_005f50e0((int)this + 0x58);
      FUN_005f50e0(iVar3);
      FUN_0054fa40(auStack_4c);
      local_4._0_1_ = 0xe;
      puVar5 = FUN_004025b0(param_1,local_64);
                    /* WARNING: Load size is inaccurate */
      local_4._0_1_ = 0xf;
      iVar3 = (**(code **)(*this + 0x28))();
      uVar6 = FUN_00551770(auStack_30,iVar3,(int)this + 0x58,puVar5,this_00,auStack_4c,&uStack_70);
      local_4._0_1_ = 0xe;
      FUN_00619730();
      local_4._0_1_ = 0xc;
      FUN_0054fa90(auStack_4c);
      uVar2 = (undefined1)local_4;
    }
  }
LAB_004f508a:
  local_4._0_1_ = uVar2;
  FUN_004ece30(&uStack_6c);
  local_4._0_1_ = 0x10;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 0x10;
  if ((uVar6 != 0) && (uVar2 = (undefined1)local_4, this_00[1] == -1)) {
    puVar5 = FUN_004025b0(param_1,local_64);
    uVar8 = *puVar5;
    FUN_00619730();
    if (uVar8 == uStack_70) {
      puVar9 = (undefined4 *)FUN_0053ca30(this,local_64);
      local_4._0_1_ = 0x11;
      FUN_004f26d0(&uStack_6c,puVar9);
      local_4._0_1_ = 0x10;
      FUN_00619730();
      uVar2 = (undefined1)local_4;
    }
    else {
      FUN_004f26d0(&uStack_6c,&uStack_70);
      uVar2 = (undefined1)local_4;
    }
  }
  local_4._0_1_ = uVar2;
  iVar3 = 0;
  if (uVar6 != 0) {
    if (this_00[1] == -1) {
      iVar3 = FUN_0041c210(*(int *)((int)this + 0x40));
      uVar6 = (uint)(iVar3 != 0);
    }
    if (uVar6 != 0) {
      if (this_00[1] == -1) {
        *(uint *)(iVar3 + 0x58) = uStack_74;
        FUN_004f26d0((void *)(iVar3 + 0x5c),(undefined4 *)((int)this + 0x4c));
        FUN_004f26d0((void *)(iVar3 + 0x48),&uStack_6c);
        FUN_004f26d0((void *)(iVar3 + 0x60),&uStack_70);
      }
      if (uVar6 != 0) goto LAB_004f519b;
    }
  }
  local_64[0] = 1;
  local_64[1] = 1;
  local_4._0_1_ = 0x12;
  FUN_00520580(this_00,local_64);
  local_4._0_1_ = 0x10;
  FUN_00619730();
LAB_004f519b:
  local_4._0_1_ = 0xc;
  FUN_00619730();
  local_4._0_1_ = 0xb;
  FUN_005510b0(auStack_30);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return uVar6;
}

