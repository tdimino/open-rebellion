
int * __fastcall FUN_004b28c0(int param_1)

{
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  undefined4 *puVar4;
  uint *puVar5;
  int *piVar6;
  int iVar7;
  void *this;
  int iVar8;
  int iVar9;
  void **ppvVar10;
  uint local_64;
  uint local_60;
  int local_5c;
  int *local_58;
  void *local_54;
  int local_50;
  void *local_4c;
  void *local_48;
  uint local_44;
  undefined4 local_40;
  undefined1 *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006390f2;
  pvStack_c = ExceptionList;
  local_60 = 0;
  local_58 = (int *)0x0;
  local_5c = 0;
  ExceptionList = &pvStack_c;
  local_50 = param_1;
  FUN_004ece30(&local_64);
  local_4 = 0;
  FUN_004f4340(local_34);
  local_4._0_1_ = 1;
  pvVar2 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                        (uint *)(*(int *)(param_1 + 0x10) + 0x34));
  iVar9 = *(int *)(param_1 + 0xc);
  local_3c = &stack0xffffff88;
  local_48 = pvVar2;
  FUN_004f26d0(&stack0xffffff88,(undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
  local_4c = FUN_00403d30((void *)(iVar9 + 0x2c));
  if (((pvVar2 != (void *)0x0) && (local_4c != (void *)0x0)) && (0 < *(int *)((int)local_4c + 0x94))
     ) {
    pvVar3 = FUN_0041acd0(*(void **)(param_1 + 0xc),*(int *)(*(int *)(param_1 + 0x10) + 0x18),1,2,
                          0x16,(void *)0x1);
    pvVar2 = (void *)(param_1 + 0x18);
    FUN_00435790(pvVar2,(int)pvVar3);
    puVar4 = FUN_004357b0(pvVar2,&local_54);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_64,puVar4);
    do {
      local_4._0_1_ = 1;
      FUN_00619730();
      while( true ) {
        local_44 = 8;
        local_40 = 0x10;
        if (((local_64 >> 0x18 < 8) || (0xf < local_64 >> 0x18)) || (local_5c != 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) {
          FUN_005f58b0(pvVar2);
          local_3c = (undefined1 *)0x8;
          local_38 = 0x10;
          if ((local_64 >> 0x18 < 8) || (0xf < local_64 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) goto LAB_004b2d2d;
          iVar9 = *(int *)(param_1 + 0xc);
          local_3c = &stack0xffffff88;
          FUN_004f26d0(&stack0xffffff88,&local_64);
          pvVar2 = FUN_00419e40((void *)(iVar9 + 0x78));
          if (((pvVar2 == (void *)0x0) ||
              (iVar9 = *(int *)((int)pvVar2 + 0xa8), local_5c = iVar9, iVar9 < 1)) ||
             (pvVar2 = *(void **)((int)local_4c + 0x94), local_54 = pvVar2, (int)pvVar2 < 1))
          goto LAB_004b2ceb;
          puVar5 = FUN_00403040(local_4c,&local_4c);
          local_4._0_1_ = 4;
          piVar6 = FUN_004f3220(*(int *)(param_1 + 4),puVar5);
          local_4._0_1_ = 1;
          FUN_00619730();
          if (piVar6 == (int *)0x0) goto LAB_004b2ceb;
          FUN_00504c40(local_2c,piVar6,1);
          local_4 = CONCAT31(local_4._1_3_,5);
          goto LAB_004b2b21;
        }
        iVar9 = *(int *)(param_1 + 0xc);
        local_3c = &stack0xffffff88;
        FUN_004f26d0(&stack0xffffff88,&local_64);
        pvVar3 = FUN_00419e40((void *)(iVar9 + 0x78));
        if ((pvVar3 == (void *)0x0) ||
           (*(int *)((int)pvVar3 + 0x28) != *(int *)(*(int *)(param_1 + 0x10) + 0x40))) break;
        local_5c = 1;
      }
      puVar4 = FUN_004357b0(pvVar2,&local_54);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_64,puVar4);
    } while( true );
  }
  goto LAB_004b2d2d;
LAB_004b2b21:
  do {
    if (iVar9 < 1) break;
    iVar9 = 0;
    iVar8 = 0;
    pvVar3 = (void *)0x0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      if ((*(byte *)((int)local_48 + 0x2c) & 8) == 0) {
        piVar6 = (int *)FUN_0052bed0((int)local_2c);
        iVar7 = (**(code **)(*piVar6 + 0x1e0))();
        if (iVar8 < iVar7) {
          ppvVar10 = &local_4c;
          pvVar2 = (void *)FUN_0052bed0((int)local_2c);
          puVar5 = FUN_004025b0(pvVar2,(uint *)ppvVar10);
          local_4._0_1_ = 6;
          pvVar2 = FUN_004f5940(local_34,puVar5);
          local_4 = CONCAT31(local_4._1_3_,5);
          FUN_00619730();
          if (pvVar2 == (void *)0x0) {
            piVar6 = (int *)FUN_0052bed0((int)local_2c);
            iVar8 = (**(code **)(*piVar6 + 0x1e0))();
LAB_004b2c26:
            pvVar3 = (void *)FUN_0052bed0((int)local_2c);
          }
        }
      }
      else {
        piVar6 = (int *)FUN_0052bed0((int)local_2c);
        iVar7 = (**(code **)(*piVar6 + 0x1dc))();
        if (iVar9 < iVar7) {
          puVar5 = &local_44;
          pvVar2 = (void *)FUN_0052bed0((int)local_2c);
          puVar5 = FUN_004025b0(pvVar2,puVar5);
          local_4._0_1_ = 7;
          pvVar2 = FUN_004f5940(local_34,puVar5);
          local_4 = CONCAT31(local_4._1_3_,5);
          FUN_00619730();
          if (pvVar2 == (void *)0x0) {
            piVar6 = (int *)FUN_0052bed0((int)local_2c);
            iVar9 = (**(code **)(*piVar6 + 0x1dc))();
            goto LAB_004b2c26;
          }
        }
      }
      FUN_005130d0((int)local_2c);
      pvVar2 = local_54;
    }
    if (pvVar3 != (void *)0x0) {
      this = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 8;
      local_54 = this;
      if (this == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        puVar5 = FUN_004025b0(pvVar3,(uint *)&local_3c);
        local_60 = local_60 | 1;
        local_4 = CONCAT31(local_4._1_3_,9);
        pvVar3 = FUN_004f5b10(this,puVar5,0);
      }
      local_4 = 8;
      if ((local_60 & 1) != 0) {
        local_60 = local_60 & 0xfffffffe;
        FUN_00619730();
      }
      local_4 = CONCAT31(local_4._1_3_,5);
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_34,pvVar3);
      }
    }
    pvVar2 = (void *)((int)pvVar2 + -1);
    iVar9 = local_5c + -1;
    param_1 = local_50;
    local_5c = iVar9;
    local_54 = pvVar2;
  } while (0 < (int)pvVar2);
  local_4._0_1_ = 1;
  FUN_00504d40(local_2c);
LAB_004b2ceb:
  iVar9 = thunk_FUN_005f5060((int)local_34);
  if ((iVar9 != 0) && (piVar6 = (int *)FUN_004f5cd0(0x201), local_58 = piVar6, piVar6 != (int *)0x0)
     ) {
    piVar6[8] = *(int *)(param_1 + 4);
    (**(code **)(*piVar6 + 0x24))();
    (**(code **)(*piVar6 + 0x2c))(&stack0xffffff98);
  }
LAB_004b2d2d:
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_58;
}

