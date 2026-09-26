
bool __thiscall FUN_00549230(void *this,int param_1,uint *param_2,void *param_3)

{
  bool bVar1;
  short sVar2;
  int iVar3;
  undefined4 uVar4;
  int iVar5;
  uint *puVar6;
  void *pvVar7;
  int *piVar8;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  bool bVar9;
  uint *puVar10;
  uint *puVar11;
  undefined4 *puVar12;
  undefined4 uVar13;
  void *pvVar14;
  uint local_74;
  uint local_70;
  int local_6c;
  void *local_68;
  uint local_64;
  undefined1 auStack_60 [4];
  undefined4 local_5c [2];
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647c10;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  local_6c = 0;
  local_68 = this;
  FUN_004ece30(&local_74);
  local_4 = 0;
  FUN_004f4340(local_5c);
  pvVar7 = param_3;
  iVar5 = param_1;
  local_4._0_1_ = 1;
  iVar3 = FUN_0054c7d0(param_1,&local_6c,&local_70,&local_64,&local_74,local_5c,param_3);
  bVar9 = iVar3 != 0;
  FUN_004ffe70(local_2c,iVar5,4);
  local_4._0_1_ = 2;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    uVar13 = 4;
    uVar4 = FUN_0052bed0((int)local_2c);
    FUN_00502db0(local_4c,uVar4,uVar13);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00513120((int)local_4c);
    while (local_30 != 0) {
      puVar12 = local_5c;
      puVar6 = &local_74;
      puVar11 = &local_64;
      puVar10 = &local_70;
      piVar8 = &local_6c;
      pvVar14 = pvVar7;
      iVar5 = FUN_0052bed0((int)local_4c);
      iVar5 = FUN_0054c7d0(iVar5,piVar8,puVar10,puVar11,puVar6,puVar12,pvVar14);
      if ((iVar5 == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
      FUN_005130d0((int)local_4c);
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4._0_1_ = 2;
    FUN_00502eb0(local_4c);
    FUN_005130d0((int)local_2c);
  }
  sVar2 = FUN_005f50e0((int)local_5c);
  *param_2 = (uint)(sVar2 != 0);
  if ((sVar2 != 0) != 0) {
    puVar6 = FUN_004ece40(&local_74);
    if (puVar6 == (uint *)0x0) {
      iVar5 = FUN_00549580((uint)local_5c,pvVar7);
      if ((iVar5 != 0) && (bVar9)) {
        bVar9 = true;
        goto LAB_00549530;
      }
    }
    else {
      param_2 = (uint *)0x0;
      pvVar7 = (void *)thunk_FUN_005f5060((int)local_5c);
      while ((pvVar7 != (void *)0x0 && (this = local_68, param_2 == (uint *)0x0))) {
        puVar6 = FUN_00403040(pvVar7,&param_1);
        uStack_54 = 0x30;
        uStack_50 = 0x40;
        local_4._0_1_ = 4;
        if ((*puVar6 >> 0x18 < 0x30) || (0x3f < *puVar6 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 2;
        FUN_00619730();
        if (bVar1) {
          puVar6 = FUN_00403040(pvVar7,auStack_60);
          local_4._0_1_ = 5;
          piVar8 = (int *)FUN_00505d40(puVar6);
          local_4._0_1_ = 2;
          FUN_00619730();
          if ((piVar8 != (int *)0x0) && (iVar5 = (**(code **)(*piVar8 + 0x1d4))(), iVar5 == 0)) {
            param_2 = (uint *)0x1;
          }
        }
        pvVar7 = *(void **)((int)pvVar7 + 0x10);
        this = local_68;
      }
      if (param_2 == (uint *)0x0) {
        iVar5 = FUN_00549850((int)local_5c,param_3);
      }
      else if (local_64 == 0) {
        bVar1 = FUN_00547740(this,local_70,&local_74,(int)local_5c,0,(uint)param_3);
        iVar5 = CONCAT31(extraout_var_00,bVar1);
      }
      else {
        bVar1 = FUN_00547880(this,local_70,&local_74,(int)local_5c,(uint)param_3);
        iVar5 = CONCAT31(extraout_var,bVar1);
      }
      if ((iVar5 != 0) && (bVar9)) {
        bVar9 = true;
        goto LAB_00549530;
      }
    }
    bVar9 = false;
  }
LAB_00549530:
  local_4._0_1_ = 1;
  FUN_004fff70(local_2c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_5c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return bVar9;
}

