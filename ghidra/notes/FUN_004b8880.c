
int __fastcall FUN_004b8880(int param_1)

{
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  int iVar4;
  void *pvVar5;
  undefined4 *puVar6;
  uint local_28;
  int local_24;
  void *local_20;
  undefined1 *local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639758;
  local_c = ExceptionList;
  local_24 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_28);
  iVar4 = *(int *)(param_1 + 0xc);
  local_1c = &stack0xffffffc4;
  local_4 = 0;
  FUN_004f26d0(&stack0xffffffc4,(undefined4 *)(param_1 + 0x28));
  pvVar2 = FUN_00419e40((void *)(iVar4 + 0x78));
  if ((pvVar2 != (void *)0x0) && (0 < *(int *)((int)pvVar2 + 0x58))) {
    iVar4 = *(int *)(param_1 + 0xc);
    local_1c = &stack0xffffffc4;
    FUN_004f26d0(&stack0xffffffc4,(undefined4 *)(param_1 + 0x5c));
    pvVar3 = FUN_00403d30((void *)(iVar4 + 0x2c));
    if ((pvVar3 != (void *)0x0) && ((*(uint *)((int)pvVar3 + 0x30) & 0x200) != 0)) {
      iVar4 = FUN_004ae150((int)pvVar2,(int)pvVar3);
      if (iVar4 != 0) {
        local_24 = 1;
        *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x100000;
        FUN_004f26d0((void *)((int)pvVar2 + 0x2c),(undefined4 *)(param_1 + 0x5c));
        *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) | 0x10000000;
      }
    }
    if (local_24 == 0) {
      pvVar5 = FUN_0041a340(*(void **)(param_1 + 0xc),
                            (undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,0,0x200,0,0,0x100000,
                            0x27,(void *)0x1);
      pvVar3 = (void *)(param_1 + 0x18);
      local_20 = pvVar3;
      FUN_00435790(pvVar3,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar3,&local_1c);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_28,puVar6);
      do {
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        do {
          local_14 = 0x90;
          local_10 = 0x98;
          if (((local_28 >> 0x18 < 0x90) || (0x97 < local_28 >> 0x18)) || (local_24 != 0)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) {
            FUN_005f58b0(local_20);
            goto LAB_004b8a84;
          }
          iVar4 = *(int *)(param_1 + 0xc);
          local_18 = &stack0xffffffc4;
          FUN_004f26d0(&stack0xffffffc4,&local_28);
          pvVar3 = FUN_00403d30((void *)(iVar4 + 0x2c));
          if (pvVar3 != (void *)0x0) {
            iVar4 = FUN_004ae150((int)pvVar2,(int)pvVar3);
            if (iVar4 != 0) {
              local_24 = 1;
              *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x100000;
              FUN_004f26d0((void *)((int)pvVar2 + 0x2c),&local_28);
              *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) | 0x10000000;
            }
          }
        } while (local_24 != 0);
        puVar6 = FUN_004357b0(local_20,&local_1c);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_28,puVar6);
      } while( true );
    }
  }
LAB_004b8a84:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_24;
}

