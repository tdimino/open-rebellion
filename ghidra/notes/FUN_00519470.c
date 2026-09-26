
uint __fastcall FUN_00519470(int *param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  uint *puVar4;
  void *pvVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int *this;
  int iVar6;
  uint local_2c;
  int local_28;
  undefined4 local_24 [2];
  uint local_1c;
  undefined4 local_18;
  uint local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642c58;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar2 = FUN_004f9700(param_1);
  FUN_004f4340(local_24);
  local_4 = 0;
  pvVar3 = (void *)FUN_00505900();
  do {
    if ((uVar2 == 0) || (pvVar3 == (void *)0x0)) {
      local_28 = 1;
      do {
        local_4._1_3_ = (uint3)(local_4 >> 8);
        if ((uVar2 == 0) || (2 < local_28)) {
          local_14 = 1;
          local_10 = 0xff;
          local_4 = CONCAT31(local_4._1_3_,5);
          for (this = FUN_0053f030(&local_14,0,0); (uVar2 != 0 && (this != (int *)0x0));
              this = (int *)FUN_004f6010(this,&local_14,0)) {
            local_28 = 0;
            iVar6 = FUN_004f6e10(this,&local_28);
            uVar2 = 0;
            if (iVar6 != 0) {
              iVar6 = 0;
              puVar4 = FUN_004025b0(this,&local_2c);
              local_4._0_1_ = 6;
              pvVar3 = FUN_004f5940(local_24,puVar4);
              local_4 = CONCAT31(local_4._1_3_,5);
              FUN_00619730();
              if (pvVar3 != (void *)0x0) {
                iVar6 = *(int *)((int)pvVar3 + 0x1c);
              }
              uVar2 = (uint)(local_28 == iVar6);
            }
          }
          local_4 = local_4 & 0xffffff00;
          FUN_00619730();
          local_4 = 0xffffffff;
          FUN_004f4380(local_24);
          ExceptionList = pvStack_c;
          return uVar2;
        }
        local_1c = 0x30;
        local_18 = 0x40;
        local_4 = CONCAT31(local_4._1_3_,3);
        pvVar3 = (void *)FUN_004f3d50(local_28,&local_1c,0);
        while ((pvVar3 != (void *)0x0 && (uVar2 != 0))) {
          FUN_004f26d0(&local_2c,(undefined4 *)((int)pvVar3 + 0x68));
          local_4 = CONCAT31(local_4._1_3_,4);
          puVar4 = FUN_004ece40(&local_2c);
          if (puVar4 != (uint *)0x0) {
            pvVar5 = FUN_004f5940(local_24,&local_2c);
            if (pvVar5 == (void *)0x0) {
              bVar1 = FUN_004f44b0(local_24,&local_2c,0);
              uVar2 = CONCAT31(extraout_var_00,bVar1);
              if (uVar2 == 0) goto LAB_00519628;
              pvVar5 = FUN_004f5940(local_24,&local_2c);
            }
            if ((uVar2 != 0) && (uVar2 = (uint)(pvVar5 != (void *)0x0), uVar2 != 0)) {
              *(int *)((int)pvVar5 + 0x1c) = *(int *)((int)pvVar5 + 0x1c) + 1;
            }
          }
LAB_00519628:
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00619730();
          pvVar3 = (void *)FUN_004f6010(pvVar3,&local_1c,0);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00619730();
        local_28 = local_28 + 1;
      } while( true );
    }
    FUN_004f26d0(&local_2c,(undefined4 *)((int)pvVar3 + 0x70));
    local_4 = CONCAT31(local_4._1_3_,2);
    puVar4 = FUN_004ece40(&local_2c);
    if (puVar4 != (uint *)0x0) {
      pvVar5 = FUN_004f5940(local_24,&local_2c);
      if (pvVar5 == (void *)0x0) {
        bVar1 = FUN_004f44b0(local_24,&local_2c,0);
        uVar2 = CONCAT31(extraout_var,bVar1);
        if (uVar2 == 0) goto LAB_00519525;
        pvVar5 = FUN_004f5940(local_24,&local_2c);
      }
      if ((uVar2 != 0) && (uVar2 = (uint)(pvVar5 != (void *)0x0), uVar2 != 0)) {
        *(int *)((int)pvVar5 + 0x1c) = *(int *)((int)pvVar5 + 0x1c) + 1;
      }
    }
LAB_00519525:
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    local_1c = 0x40;
    local_18 = 0x80;
    local_4._0_1_ = 1;
    pvVar3 = (void *)FUN_004f6010(pvVar3,&local_1c,1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  } while( true );
}

