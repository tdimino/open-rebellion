
int __fastcall FUN_004b6510(void *param_1)

{
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  void *pvVar6;
  uint local_2c;
  int local_28;
  void *local_24;
  undefined1 *local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639508;
  local_c = ExceptionList;
  local_28 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_2c);
  iVar5 = *(int *)((int)param_1 + 0xc);
  local_20 = &stack0xffffffc0;
  local_4 = 0;
  FUN_004f26d0(&stack0xffffffc0,(undefined4 *)((int)param_1 + 0x28));
  pvVar2 = FUN_00419e40((void *)(iVar5 + 0x78));
  if ((pvVar2 != (void *)0x0) && (0 < *(int *)((int)pvVar2 + 0x80))) {
    pvVar3 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0xc) + 0x44),
                          (uint *)(*(int *)((int)param_1 + 0x10) + 0x34));
    if (pvVar3 != (void *)0x0) {
      if (0 < *(int *)((int)pvVar3 + 0x19c)) {
        pvVar3 = FUN_0041a340(*(void **)((int)param_1 + 0xc),
                              (undefined4 *)(*(int *)((int)param_1 + 0x10) + 0x34),0,0,4,0,0,
                              0x300200,0x32,(void *)0x1);
        FUN_00435790((void *)((int)param_1 + 0x18),(int)pvVar3);
        puVar4 = FUN_004357b0((void *)((int)param_1 + 0x18),&local_24);
        local_4._0_1_ = 1;
        FUN_004f26d0(&local_2c,puVar4);
        do {
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
          do {
            local_1c = (undefined1 *)0x90;
            local_18 = 0x98;
            if (((local_2c >> 0x18 < 0x90) || (0x97 < local_2c >> 0x18)) || (local_28 != 0)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (!bVar1) {
              FUN_005f58b0((void *)((int)param_1 + 0x18));
              goto LAB_004b66c9;
            }
            iVar5 = *(int *)((int)param_1 + 0xc);
            local_20 = &stack0xffffffc0;
            FUN_004f26d0(&stack0xffffffc0,&local_2c);
            pvVar3 = FUN_00403d30((void *)(iVar5 + 0x2c));
            if (pvVar3 != (void *)0x0) {
              iVar5 = FUN_004ae240(param_1,(int)pvVar2,pvVar3);
              if (iVar5 != 0) {
                local_28 = 1;
                *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x200000;
                FUN_004f26d0((void *)((int)pvVar2 + 0x2c),&local_2c);
                *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) | 0x20000000;
              }
            }
          } while (local_28 != 0);
          puVar4 = FUN_004357b0((void *)((int)param_1 + 0x18),&local_24);
          local_4._0_1_ = 2;
          FUN_004f26d0(&local_2c,puVar4);
        } while( true );
      }
LAB_004b66c9:
      if (local_28 == 0) {
        pvVar6 = FUN_0041a340(*(void **)((int)param_1 + 0xc),
                              (undefined4 *)(*(int *)((int)param_1 + 0x10) + 0x34),0,0,4,0,0,
                              0x300200,0x21,(void *)0x1);
        pvVar3 = (void *)((int)param_1 + 0x18);
        local_24 = pvVar3;
        FUN_00435790(pvVar3,(int)pvVar6);
        pvVar6 = FUN_0041a340(*(void **)((int)param_1 + 0xc),
                              (undefined4 *)(*(int *)((int)param_1 + 0x10) + 0x34),0,0,0x24,0,0,
                              0x300200,0x14,&DAT_00000002);
        FUN_00435790(pvVar3,(int)pvVar6);
        puVar4 = FUN_004357b0(pvVar3,&local_20);
        local_4._0_1_ = 3;
        FUN_004f26d0(&local_2c,puVar4);
        do {
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
          do {
            local_14 = 0x90;
            local_10 = 0x98;
            if (((local_2c >> 0x18 < 0x90) || (0x97 < local_2c >> 0x18)) || (local_28 != 0)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (!bVar1) {
              FUN_005f58b0(local_24);
              goto LAB_004b683b;
            }
            iVar5 = *(int *)((int)param_1 + 0xc);
            local_1c = &stack0xffffffc0;
            FUN_004f26d0(&stack0xffffffc0,&local_2c);
            pvVar3 = FUN_00403d30((void *)(iVar5 + 0x2c));
            if (pvVar3 != (void *)0x0) {
              iVar5 = FUN_004ae240(param_1,(int)pvVar2,pvVar3);
              if (iVar5 != 0) {
                local_28 = 1;
                *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x200000;
                FUN_004f26d0((void *)((int)pvVar2 + 0x2c),&local_2c);
                *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) | 0x20000000;
              }
            }
          } while (local_28 != 0);
          puVar4 = FUN_004357b0(local_24,&local_20);
          local_4._0_1_ = 4;
          FUN_004f26d0(&local_2c,puVar4);
        } while( true );
      }
    }
  }
LAB_004b683b:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_28;
}

