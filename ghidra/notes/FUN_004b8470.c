
int * __fastcall FUN_004b8470(int param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint *puVar6;
  int *piVar7;
  int iVar8;
  void *this;
  int iVar9;
  void *pvVar10;
  void **ppvVar11;
  uint local_58;
  uint local_54;
  int local_50;
  int *local_4c;
  void *local_48;
  void *local_44;
  undefined1 *local_40;
  undefined4 local_3c [2];
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063972a;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_54 = 0;
  local_4c = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_50 = param_1;
  FUN_004ece30(&local_58);
  local_4 = 0;
  FUN_004f4340(local_3c);
  iVar1 = *(int *)(param_1 + 0xc);
  local_40 = &stack0xffffff94;
  local_4._0_1_ = 1;
  FUN_004f26d0(&stack0xffffff94,(undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
  local_44 = FUN_00403d30((void *)(iVar1 + 0x2c));
  if ((local_44 != (void *)0x0) && (0 < *(int *)((int)local_44 + 0x94))) {
    pvVar4 = FUN_0041acd0(*(void **)(param_1 + 0xc),*(int *)(*(int *)(param_1 + 0x10) + 0x18),1,2,
                          0x16,(void *)0x1);
    pvVar10 = (void *)(param_1 + 0x18);
    FUN_00435790(pvVar10,(int)pvVar4);
    puVar5 = FUN_004357b0(pvVar10,&local_48);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_58,puVar5);
    do {
      local_4._0_1_ = 1;
      FUN_00619730();
      while( true ) {
        local_34 = 8;
        local_30 = 0x10;
        if (((local_58 >> 0x18 < 8) || (0xf < local_58 >> 0x18)) || (bVar2)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        iVar1 = local_50;
        if (!bVar3) {
          FUN_005f58b0(pvVar10);
          local_34 = 8;
          local_30 = 0x10;
          if ((local_58 >> 0x18 < 8) || (0xf < local_58 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          iVar1 = local_50;
          if (!bVar2) goto LAB_004b883b;
          local_40 = &stack0xffffff94;
          iVar9 = *(int *)(local_50 + 0xc);
          FUN_004f26d0(&stack0xffffff94,&local_58);
          pvVar4 = FUN_00419e40((void *)(iVar9 + 0x78));
          if (((pvVar4 == (void *)0x0) ||
              (iVar9 = *(int *)((int)pvVar4 + 0xa8), local_50 = iVar9, iVar9 < 1)) ||
             (pvVar4 = *(void **)((int)local_44 + 0x94), local_48 = pvVar4, (int)pvVar4 < 1))
          goto LAB_004b87f9;
          puVar6 = FUN_00403040(local_44,&local_44);
          local_4._0_1_ = 4;
          piVar7 = FUN_004f3220(*(int *)(iVar1 + 4),puVar6);
          local_4._0_1_ = 1;
          FUN_00619730();
          if (piVar7 == (int *)0x0) goto LAB_004b87f9;
          FUN_00504c40(local_2c,piVar7,1);
          local_4 = CONCAT31(local_4._1_3_,5);
          goto LAB_004b86ae;
        }
        local_40 = &stack0xffffff94;
        iVar9 = *(int *)(local_50 + 0xc);
        FUN_004f26d0(&stack0xffffff94,&local_58);
        pvVar4 = FUN_00419e40((void *)(iVar9 + 0x78));
        if ((pvVar4 == (void *)0x0) ||
           (*(int *)((int)pvVar4 + 0x28) != *(int *)(*(int *)(iVar1 + 0x10) + 0x40))) break;
        bVar2 = true;
      }
      puVar5 = FUN_004357b0(pvVar10,&local_48);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_58,puVar5);
    } while( true );
  }
  goto LAB_004b883b;
  while( true ) {
    iVar9 = 0;
    pvVar10 = (void *)0x0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      piVar7 = (int *)FUN_0052bed0((int)local_2c);
      iVar8 = (**(code **)(*piVar7 + 0x1dc))();
      if (iVar9 < iVar8) {
        ppvVar11 = &local_44;
        pvVar4 = (void *)FUN_0052bed0((int)local_2c);
        puVar6 = FUN_004025b0(pvVar4,(uint *)ppvVar11);
        local_4._0_1_ = 6;
        pvVar4 = FUN_004f5940(local_3c,puVar6);
        local_4 = CONCAT31(local_4._1_3_,5);
        FUN_00619730();
        if (pvVar4 == (void *)0x0) {
          piVar7 = (int *)FUN_0052bed0((int)local_2c);
          iVar9 = (**(code **)(*piVar7 + 0x1dc))();
          pvVar10 = (void *)FUN_0052bed0((int)local_2c);
        }
      }
      FUN_005130d0((int)local_2c);
      pvVar4 = local_48;
    }
    if (pvVar10 != (void *)0x0) {
      this = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 7;
      local_48 = this;
      if (this == (void *)0x0) {
        pvVar10 = (void *)0x0;
      }
      else {
        puVar6 = FUN_004025b0(pvVar10,(uint *)&local_40);
        local_54 = local_54 | 1;
        local_4 = CONCAT31(local_4._1_3_,8);
        pvVar10 = FUN_004f5b10(this,puVar6,0);
      }
      local_4 = 7;
      if ((local_54 & 1) != 0) {
        local_54 = local_54 & 0xfffffffe;
        FUN_00619730();
      }
      local_4 = CONCAT31(local_4._1_3_,5);
      if (pvVar10 != (void *)0x0) {
        FUN_004f57b0(local_3c,pvVar10);
      }
    }
    pvVar4 = (void *)((int)pvVar4 + -1);
    iVar9 = local_50 + -1;
    local_50 = iVar9;
    local_48 = pvVar4;
    if ((int)pvVar4 < 1) break;
LAB_004b86ae:
    if (iVar9 < 1) break;
  }
  local_4._0_1_ = 1;
  FUN_00504d40(local_2c);
LAB_004b87f9:
  iVar9 = thunk_FUN_005f5060((int)local_3c);
  if ((iVar9 != 0) && (piVar7 = (int *)FUN_004f5cd0(0x201), local_4c = piVar7, piVar7 != (int *)0x0)
     ) {
    piVar7[8] = *(int *)(iVar1 + 4);
    (**(code **)(*piVar7 + 0x24))();
    (**(code **)(*piVar7 + 0x2c))(&stack0xffffffa4);
  }
LAB_004b883b:
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_3c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_4c;
}

