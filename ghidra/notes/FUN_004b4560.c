
int __fastcall FUN_004b4560(int param_1)

{
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int iVar6;
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
  puStack_8 = &LAB_006392b8;
  local_c = ExceptionList;
  local_24 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_28);
  iVar6 = *(int *)(param_1 + 0xc);
  local_1c = &stack0xffffffc4;
  local_4 = 0;
  FUN_004f26d0(&stack0xffffffc4,(undefined4 *)(param_1 + 0x28));
  pvVar2 = FUN_00419e40((void *)(iVar6 + 0x78));
  if ((pvVar2 == (void *)0x0) || (*(int *)((int)pvVar2 + 0x58) < 1)) {
LAB_004b46f9:
    local_4 = 0xffffffff;
    FUN_00619730();
    ExceptionList = local_c;
    return local_24;
  }
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0xc),(undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,
                        0,0x200,0,0,0x100000,0x27,(void *)0x1);
  pvVar5 = (void *)(param_1 + 0x18);
  local_20 = pvVar5;
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar4 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
  local_4._0_1_ = 1;
  FUN_004f26d0(&local_28,puVar4);
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
        goto LAB_004b46f9;
      }
      iVar6 = *(int *)(param_1 + 0xc);
      local_18 = &stack0xffffffc4;
      FUN_004f26d0(&stack0xffffffc4,&local_28);
      pvVar5 = FUN_00403d30((void *)(iVar6 + 0x2c));
      if (pvVar5 != (void *)0x0) {
        iVar6 = FUN_004ae150((int)pvVar2,(int)pvVar5);
        if (iVar6 != 0) {
          local_24 = 1;
          *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) | 0x100000;
          FUN_004f26d0((void *)((int)pvVar2 + 0x2c),&local_28);
          *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) | 0x10000000;
        }
      }
    } while (local_24 != 0);
    puVar4 = (undefined4 *)FUN_004357b0(local_20,&local_1c);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_28,puVar4);
  } while( true );
}

