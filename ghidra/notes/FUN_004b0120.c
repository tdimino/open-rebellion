
int * __fastcall FUN_004b0120(int param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint *puVar6;
  int *piVar7;
  undefined1 *this;
  void *this_00;
  int iVar8;
  undefined1 **ppuVar9;
  uint local_5c;
  uint local_58;
  int *local_54;
  int local_50;
  int local_4c;
  undefined1 local_48 [4];
  undefined1 *local_44;
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
  puStack_8 = &LAB_00638d62;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_58 = 0;
  local_54 = (int *)0x0;
  local_50 = 0;
  ExceptionList = &pvStack_c;
  local_4c = param_1;
  FUN_004ece30(&local_5c);
  local_4 = 0;
  FUN_004f4340(local_3c);
  iVar1 = *(int *)(param_1 + 0xc);
  local_44 = &stack0xffffff90;
  local_4._0_1_ = 1;
  FUN_004f26d0(&stack0xffffff90,(undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
  local_44 = (undefined1 *)FUN_00403d30((void *)(iVar1 + 0x2c));
  if ((local_44 != (undefined1 *)0x0) && (0 < *(int *)(local_44 + 0xac))) {
    pvVar4 = FUN_0041acd0(*(void **)(param_1 + 0xc),*(int *)(*(int *)(param_1 + 0x10) + 0x18),1,2,
                          0x17,(void *)0x1);
    this_00 = (void *)(param_1 + 0x18);
    FUN_00435790(this_00,(int)pvVar4);
    puVar5 = (undefined4 *)FUN_004357b0(this_00,local_48);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_5c,puVar5);
    do {
      local_4._0_1_ = 1;
      FUN_00619730();
      while( true ) {
        local_34 = 8;
        local_30 = 0x10;
        if ((((local_5c >> 0x18 < 8) || (0xf < local_5c >> 0x18)) || (local_50 != 0)) || (bVar2)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        iVar1 = local_4c;
        if (!bVar3) {
          FUN_005f58b0(this_00);
          local_34 = 8;
          local_30 = 0x10;
          if ((local_5c >> 0x18 < 8) || (0xf < local_5c >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          iVar1 = local_4c;
          if (!bVar2) goto LAB_004b0484;
          local_40 = &stack0xffffff90;
          iVar8 = *(int *)(local_4c + 0xc);
          FUN_004f26d0(&stack0xffffff90,&local_5c);
          pvVar4 = FUN_00419e40((void *)(iVar8 + 0x78));
          if (pvVar4 == (void *)0x0) goto LAB_004b0440;
          iVar8 = *(int *)(local_44 + 0xac);
          if (*(int *)((int)pvVar4 + 0xac) <= *(int *)(local_44 + 0xac)) {
            iVar8 = *(int *)((int)pvVar4 + 0xac);
          }
          puVar6 = (uint *)FUN_00403040(local_44,&local_44);
          local_4._0_1_ = 4;
          piVar7 = FUN_004f3220(*(int *)(iVar1 + 4),puVar6);
          local_4._0_1_ = 1;
          FUN_00619730();
          if (piVar7 == (int *)0x0) goto LAB_004b0440;
          FUN_005039d0(local_2c,piVar7,1);
          local_4 = CONCAT31(local_4._1_3_,5);
          FUN_00513120((int)local_2c);
          if (0 < iVar8) goto LAB_004b0397;
          goto LAB_004b0432;
        }
        local_40 = &stack0xffffff90;
        iVar8 = *(int *)(local_4c + 0xc);
        FUN_004f26d0(&stack0xffffff90,&local_5c);
        pvVar4 = FUN_00419e40((void *)(iVar8 + 0x78));
        if ((pvVar4 == (void *)0x0) ||
           (*(int *)((int)pvVar4 + 0x28) != *(int *)(*(int *)(iVar1 + 0x10) + 0x40))) break;
        if (*(int *)((int)pvVar4 + 0xac) < 1) {
          FUN_004ece80(&local_5c);
          bVar2 = true;
        }
        else {
          local_50 = 1;
        }
      }
      puVar5 = (undefined4 *)FUN_004357b0(this_00,local_48);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_5c,puVar5);
    } while( true );
  }
  goto LAB_004b0484;
  while( true ) {
    this = (undefined1 *)FUN_00618b70(0x20);
    local_4._0_1_ = 6;
    local_40 = this;
    if (this == (undefined1 *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      ppuVar9 = &local_44;
      pvVar4 = (void *)FUN_0052bed0((int)local_2c);
      puVar6 = FUN_004025b0(pvVar4,(uint *)ppuVar9);
      local_58 = local_58 | 1;
      local_4 = CONCAT31(local_4._1_3_,7);
      pvVar4 = FUN_004f5b10(this,puVar6,0);
    }
    local_4 = 6;
    if ((local_58 & 1) != 0) {
      local_58 = local_58 & 0xfffffffe;
      FUN_00619730();
    }
    local_4 = CONCAT31(local_4._1_3_,5);
    if (pvVar4 != (void *)0x0) {
      FUN_004f57b0(local_3c,pvVar4);
    }
    FUN_005130d0((int)local_2c);
    iVar8 = iVar8 + -1;
    if (iVar8 < 1) break;
LAB_004b0397:
    if (local_10 == 0) break;
  }
LAB_004b0432:
  local_4._0_1_ = 1;
  FUN_00503ad0(local_2c);
LAB_004b0440:
  iVar8 = thunk_FUN_005f5060((int)local_3c);
  if ((iVar8 != 0) && (piVar7 = (int *)FUN_004f5cd0(0x201), local_54 = piVar7, piVar7 != (int *)0x0)
     ) {
    piVar7[8] = *(int *)(iVar1 + 4);
    (**(code **)(*piVar7 + 0x24))();
    (**(code **)(*piVar7 + 0x2c))(&stack0xffffffa0);
  }
LAB_004b0484:
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_3c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_54;
}

