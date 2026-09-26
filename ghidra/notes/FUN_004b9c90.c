
int __fastcall FUN_004b9c90(int param_1)

{
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  uint local_24;
  int local_20;
  undefined1 *local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639913;
  local_c = ExceptionList;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  iVar5 = *(int *)(param_1 + 0xc);
  local_1c = &stack0xffffffc8;
  local_4 = 0;
  FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(param_1 + 0x28));
  pvVar2 = FUN_00419e40((void *)(iVar5 + 0x78));
  if (pvVar2 != (void *)0x0) {
    if (0 < *(int *)((int)pvVar2 + 0x58)) {
      pvVar3 = FUN_0041a340(*(void **)(param_1 + 0xc),
                            (undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,0,0x200,0,0,0x100000,
                            0x27,(void *)0x1);
      FUN_00435790((void *)(param_1 + 0x18),(int)pvVar3);
      puVar4 = FUN_004357b0((void *)(param_1 + 0x18),&local_1c);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_24,puVar4);
      do {
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        do {
          local_14 = 0x90;
          local_10 = 0x98;
          if (((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) || (local_20 != 0)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) {
            FUN_005f58b0((void *)(param_1 + 0x18));
            goto LAB_004b9e23;
          }
          iVar5 = *(int *)(param_1 + 0xc);
          local_18 = &stack0xffffffc8;
          FUN_004f26d0(&stack0xffffffc8,&local_24);
          pvVar3 = FUN_00403d30((void *)(iVar5 + 0x2c));
          if (pvVar3 != (void *)0x0) {
            iVar5 = FUN_004ae150((int)pvVar2,(int)pvVar3);
            if (iVar5 != 0) {
              local_20 = 1;
              *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x100000;
              FUN_004f26d0((void *)((int)pvVar2 + 0x2c),&local_24);
              *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) | 0x10000000;
            }
          }
        } while (local_20 != 0);
        puVar4 = FUN_004357b0((void *)(param_1 + 0x18),&local_1c);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_24,puVar4);
      } while( true );
    }
LAB_004b9e23:
    if ((local_20 == 0) && (0 < *(int *)((int)pvVar2 + 0x5c))) {
      pvVar3 = FUN_0041a340(*(void **)(param_1 + 0xc),
                            (undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,0x400000,4,0,0,
                            0x100000,0x1b,(void *)0x1);
      FUN_00435790((void *)(param_1 + 0x18),(int)pvVar3);
      puVar4 = FUN_004357b0((void *)(param_1 + 0x18),&local_1c);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_24,puVar4);
      do {
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        do {
          local_14 = 0x90;
          local_10 = 0x98;
          if (((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) || (local_20 != 0)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) {
            FUN_005f58b0((void *)(param_1 + 0x18));
            goto LAB_004b9f73;
          }
          iVar5 = *(int *)(param_1 + 0xc);
          local_18 = &stack0xffffffc8;
          FUN_004f26d0(&stack0xffffffc8,&local_24);
          pvVar3 = FUN_00403d30((void *)(iVar5 + 0x2c));
          if (pvVar3 != (void *)0x0) {
            iVar5 = FUN_004ae150((int)pvVar2,(int)pvVar3);
            if (iVar5 != 0) {
              local_20 = 1;
              *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x100000;
              FUN_004f26d0((void *)((int)pvVar2 + 0x2c),&local_24);
              *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) | 0x10000000;
            }
          }
        } while (local_20 != 0);
        puVar4 = FUN_004357b0((void *)(param_1 + 0x18),&local_1c);
        local_4._0_1_ = 4;
        FUN_004f26d0(&local_24,puVar4);
      } while( true );
    }
  }
LAB_004b9f73:
  iVar5 = local_20;
  if (local_20 != 0) {
    local_18 = (undefined1 *)FUN_00618b70(0x1c);
    local_4._0_1_ = 5;
    if (local_18 == (undefined1 *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_004ecc70(local_18,&local_24);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    if (pvVar2 != (void *)0x0) {
      FUN_004f57b0((void *)(param_1 + 0x5c),pvVar2);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar5;
}

