
int * __fastcall FUN_004de6f0(int param_1)

{
  uint *this;
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int *piVar6;
  uint *puVar7;
  int iVar8;
  int iVar9;
  undefined1 *puVar10;
  int local_5c;
  int *local_50;
  int local_4c;
  undefined1 local_48 [4];
  uint uStack_44;
  undefined1 *local_40;
  void *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d772;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_50 = (int *)0x0;
  local_5c = 0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  puVar7 = (uint *)(param_1 + 0x50);
  local_4 = 0;
  FUN_004ece80(puVar7);
  this = (uint *)(param_1 + 0x54);
  FUN_004ece80(this);
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x40010,0,0,0,0,0,0x1e,&DAT_00000002);
  pvVar5 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar4 = FUN_004357b0(pvVar5,&local_4c);
  local_4._0_1_ = 1;
  FUN_004f26d0(this,puVar4);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x2001,0,1,2,0,0,0x15,&DAT_00000002);
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar4 = FUN_004357b0(pvVar5,&local_4c);
  local_4._0_1_ = 2;
  FUN_004f26d0(this,puVar4);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  local_3c = (void *)0x90;
  local_38 = 0x98;
  if ((*this >> 0x18 < 0x90) || (local_4c = 1, 0x97 < *this >> 0x18)) {
    local_4c = 0;
  }
  FUN_00619730();
  if (local_4c != 0) {
    iVar9 = *(int *)(param_1 + 0x2c);
    local_40 = &stack0xffffff90;
    FUN_004f26d0(&stack0xffffff90,this);
    pvVar3 = FUN_00403d30((void *)(iVar9 + 0x2c));
    if (pvVar3 != (void *)0x0) {
      local_5c = *(int *)((int)pvVar3 + 0x9c);
    }
    puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),this,0x80000,0,0,0,0,0,1);
    FUN_00435790(pvVar5,(int)puVar4);
    puVar4 = FUN_004357b0(pvVar5,&local_4c);
    local_4._0_1_ = 3;
    FUN_004f26d0(puVar7,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar7,0x10000,0,1,2,0,0x30000,0x13,
                          &DAT_00000002);
    FUN_00435790(pvVar5,(int)pvVar3);
    puVar4 = FUN_004357b0(pvVar5,&local_4c);
    local_4._0_1_ = 4;
    FUN_004f26d0(puVar7,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
    local_3c = (void *)0x90;
    local_38 = 0x98;
    if ((*puVar7 >> 0x18 < 0x90) || (0x97 < *puVar7 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      iVar9 = *(int *)(param_1 + 0x2c);
      local_40 = &stack0xffffff90;
      FUN_004f26d0(&stack0xffffff90,puVar7);
      pvVar5 = FUN_00403d30((void *)(iVar9 + 0x2c));
      piVar6 = FUN_004f3220(*(int *)(param_1 + 0x30),puVar7);
      if ((piVar6 != (int *)0x0) && (pvVar5 != (void *)0x0)) {
        local_4c = 0;
        if (*(int *)((int)pvVar5 + 0x94) < local_5c) {
          local_5c = *(int *)((int)pvVar5 + 0x94);
        }
        FUN_00504c40(local_2c,piVar6,1);
        local_4._0_1_ = 5;
        do {
          if (local_5c < 1) break;
          FUN_00513120((int)local_2c);
          pvVar5 = (void *)0x0;
          iVar9 = 1000000;
          FUN_00513120((int)local_2c);
          while (local_10 != 0) {
            puVar10 = local_48;
            pvVar3 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_00402d80(pvVar3,puVar10);
            local_4._0_1_ = 6;
            FUN_0042e190((void *)(*(int *)(param_1 + 0x2c) + 0x138),puVar7);
            local_4._0_1_ = 5;
            FUN_00619730();
            piVar6 = (int *)FUN_0052bed0((int)local_2c);
            iVar8 = (**(code **)(*piVar6 + 0x1e0))();
            if (iVar8 < iVar9) {
              puVar7 = &uStack_44;
              pvVar3 = (void *)FUN_0052bed0((int)local_2c);
              puVar7 = FUN_004025b0(pvVar3,puVar7);
              local_4._0_1_ = 7;
              pvVar3 = FUN_004f5940(local_34,puVar7);
              local_4._0_1_ = 5;
              FUN_00619730();
              if (pvVar3 == (void *)0x0) {
                piVar6 = (int *)FUN_0052bed0((int)local_2c);
                iVar9 = (**(code **)(*piVar6 + 0x1e0))();
                pvVar5 = (void *)FUN_0052bed0((int)local_2c);
              }
            }
            FUN_005130d0((int)local_2c);
          }
          if (pvVar5 == (void *)0x0) {
            local_4c = 1;
          }
          else {
            pvVar3 = (void *)FUN_00618b70(0x20);
            local_4._0_1_ = 8;
            local_3c = pvVar3;
            if (pvVar3 == (void *)0x0) {
              pvVar5 = (void *)0x0;
            }
            else {
              puVar7 = FUN_004025b0(pvVar5,(uint *)&local_40);
              bVar2 = true;
              local_4 = CONCAT31(local_4._1_3_,9);
              pvVar5 = FUN_004f5b10(pvVar3,puVar7,0);
            }
            local_4 = 8;
            if (bVar2) {
              bVar2 = false;
              FUN_00619730();
            }
            local_4._0_1_ = 5;
            if (pvVar5 != (void *)0x0) {
              FUN_004f57b0(local_34,pvVar5);
            }
            local_5c = local_5c + -1;
          }
        } while (local_4c == 0);
        local_4._0_1_ = 0;
        FUN_00504d40(local_2c);
      }
    }
  }
  iVar9 = thunk_FUN_005f5060((int)local_34);
  if ((iVar9 != 0) && (local_50 = (int *)FUN_004f5cd0(0x201), local_50 != (int *)0x0)) {
    local_50[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*local_50 + 0x24))();
    (**(code **)(*local_50 + 0x2c))(this);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return local_50;
}

