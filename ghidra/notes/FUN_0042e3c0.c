
undefined4 __thiscall
FUN_0042e3c0(void *this,uint *param_1,void *param_2,void *param_3,uint param_4,int param_5,
            undefined4 param_6)

{
  bool bVar1;
  short sVar2;
  int iVar3;
  void *pvVar4;
  uint *puVar5;
  undefined2 extraout_var;
  undefined4 *puVar6;
  void *this_00;
  int *piVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  undefined4 local_34;
  undefined4 local_2c [4];
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c905;
  local_c = ExceptionList;
  bVar1 = false;
  local_34 = 0;
  ExceptionList = &local_c;
  FUN_0052d720(local_2c);
  puVar5 = param_1;
  local_4 = 0;
  FUN_004fcdd0(param_1);
  if ((param_2 != (void *)0x0) && (iVar3 = FUN_0052e580(param_2,(uint)param_3,local_2c), iVar3 != 0)
     ) {
    pvVar4 = (void *)thunk_FUN_005f5060((int)local_2c);
    uVar8 = param_4;
    while (pvVar4 != (void *)0x0) {
      puVar5 = FUN_0042ec10(pvVar4,&param_2);
      local_4._0_1_ = 1;
      iVar3 = FUN_0042e190(this,puVar5);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      if ((iVar3 == 0) || ((*(uint *)(iVar3 + 0x18) & uVar8) != uVar8)) {
        puVar5 = (uint *)((int)pvVar4 + 0x18);
        pvVar4 = *(void **)((int)pvVar4 + 0x10);
        FUN_005f54a0(local_2c,*puVar5);
        puVar5 = param_1;
      }
      else {
        pvVar4 = *(void **)((int)pvVar4 + 0x10);
        puVar5 = param_1;
      }
    }
  }
  if (param_5 == 0) {
    sVar2 = FUN_004f4290((int)local_2c);
    iVar3 = FUN_0041cd80(CONCAT22(extraout_var,sVar2));
    iVar10 = 0;
    pvVar4 = (void *)thunk_FUN_005f5060((int)local_2c);
    if (pvVar4 != (void *)0x0) {
      do {
        if (iVar3 <= iVar10) break;
        pvVar4 = *(void **)((int)pvVar4 + 0x10);
        iVar10 = iVar10 + 1;
      } while (pvVar4 != (void *)0x0);
      if (pvVar4 != (void *)0x0) {
        puVar6 = FUN_0042ec10(pvVar4,&param_1);
        local_4._0_1_ = 2;
        FUN_004f26d0(puVar5,puVar6);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
    }
  }
  else {
    FUN_004355d0(local_1c);
    local_4._0_1_ = 3;
    FUN_004ece30(&param_2);
    local_4._0_1_ = 4;
    param_3 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 5;
    if (param_3 == (void *)0x0) {
      param_3 = (void *)0x0;
    }
    else {
      param_3 = FUN_0041be80(param_3,param_6);
    }
    pvVar4 = param_3;
    local_4._0_1_ = 4;
    if (param_3 != (void *)0x0) {
      this_00 = (void *)thunk_FUN_005f5060((int)local_2c);
      for (; this_00 != (void *)0x0; this_00 = *(void **)((int)this_00 + 0x10)) {
        puVar5 = FUN_0042ec10(this_00,&param_6);
        local_4._0_1_ = 6;
        piVar7 = (int *)FUN_0042e190(this,puVar5);
        local_4._0_1_ = 4;
        FUN_00619730();
        if (piVar7 != (int *)0x0) {
          uVar8 = ((uint)param_2 ^ piVar7[0xb]) & 0xffffff ^ piVar7[0xb];
          param_2 = (void *)(uVar8 ^ (uVar8 ^ piVar7[0xb]) & 0xffffff);
          pvVar4 = (void *)FUN_00618b70(0x18);
          local_4._0_1_ = 7;
          if (pvVar4 == (void *)0x0) {
            pvVar4 = (void *)0x0;
          }
          else {
            FUN_005f4950(&param_4,param_5);
            bVar1 = true;
            local_4 = CONCAT31(local_4._1_3_,8);
            uVar9 = (**(code **)(*piVar7 + 4))(&param_4);
            pvVar4 = FUN_0041c360(pvVar4,&param_2,uVar9);
          }
          local_4 = 7;
          if (bVar1) {
            bVar1 = false;
            FUN_00619730();
          }
          local_4._0_1_ = 4;
          if (pvVar4 != (void *)0x0) {
            FUN_0041c070(param_3,pvVar4);
          }
        }
        pvVar4 = param_3;
      }
      FUN_0041c230((int)pvVar4);
      FUN_00435790(local_1c,(int)pvVar4);
      puVar6 = FUN_004357b0(local_1c,&param_5);
      local_4._0_1_ = 9;
      FUN_004f26d0(&param_2,puVar6);
      local_4._0_1_ = 4;
      FUN_00619730();
      *param_1 = (uint)param_2;
    }
    local_4._0_1_ = 3;
    FUN_00619730();
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00435610(local_1c);
    puVar5 = param_1;
  }
  if ((*puVar5 & 0xff000000) != 0) {
    iVar3 = FUN_0042e190(this,puVar5);
    local_34 = *(undefined4 *)(iVar3 + 0x20);
  }
  local_4 = 0xffffffff;
  FUN_0052d760(local_2c);
  ExceptionList = local_c;
  return local_34;
}

