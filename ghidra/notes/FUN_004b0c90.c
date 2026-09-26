
int * __fastcall FUN_004b0c90(int *param_1)

{
  int iVar1;
  bool bVar2;
  ushort uVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int *piVar6;
  uint local_38;
  int local_34;
  uint local_30;
  int *local_2c;
  undefined1 local_28 [4];
  undefined1 *local_24;
  undefined4 local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638eab;
  pvStack_c = ExceptionList;
  local_2c = (int *)0x0;
  local_34 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_30);
  local_4 = 0;
  FUN_004ece30(&local_38);
  local_4._0_1_ = 1;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 2;
  local_24 = (undefined1 *)0x8;
  local_20 = 0x10;
  if (((uint)param_1[10] >> 0x18 < 8) || (0xf < (uint)param_1[10] >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    uVar3 = FUN_005f50e0(param_1[4] + 0x2c);
    if (1 < uVar3) {
      puVar4 = FUN_0041ae20((void *)param_1[3],param_1 + 10,*(int *)(param_1[4] + 0x18),1,0xf0000002
                            ,2);
      piVar6 = param_1 + 6;
      FUN_00435790(piVar6,(int)puVar4);
      puVar4 = FUN_004357b0(piVar6,local_28);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_38,puVar4);
      while( true ) {
        local_4._0_1_ = 2;
        FUN_00619730();
        local_14 = 8;
        local_10 = 0x10;
        if (((local_38 >> 0x18 < 8) || (0xf < local_38 >> 0x18)) || (local_34 != 0)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) break;
        if (local_38 != param_1[10]) {
          iVar1 = param_1[3];
          local_24 = &stack0xffffffb4;
          FUN_004f26d0(&stack0xffffffb4,&local_38);
          pvVar5 = FUN_00419e40((void *)(iVar1 + 0x78));
          if (pvVar5 != (void *)0x0) {
            iVar1 = param_1[3];
            local_24 = &stack0xffffffb4;
            FUN_004f26d0(&stack0xffffffb4,(undefined4 *)((int)pvVar5 + 0x28));
            pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
            if ((*(uint *)((int)pvVar5 + 0x30) & 0x200) == 0) {
              FUN_004f26d0(&local_30,&local_38);
              local_34 = 1;
            }
          }
        }
        puVar4 = FUN_004357b0(piVar6,local_28);
        local_4._0_1_ = 4;
        FUN_004f26d0(&local_38,puVar4);
      }
      FUN_005f58b0(piVar6);
    }
    if (local_34 == 0) {
      puVar4 = (undefined4 *)(**(code **)(*param_1 + 0x54))();
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_30,puVar4);
      local_4._0_1_ = 2;
      FUN_00619730();
      local_14 = 0x90;
      local_10 = 0x98;
      if ((local_30 >> 0x18 < 0x90) || (0x97 < local_30 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        local_34 = 1;
      }
      if (local_34 == 0) goto LAB_004b0f44;
    }
    local_24 = (undefined1 *)FUN_00618b70(0x20);
    local_4._0_1_ = 6;
    if (local_24 == (undefined1 *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_004f5b10(local_24,param_1 + 10,0);
    }
    local_4._0_1_ = 2;
    if (pvVar5 != (void *)0x0) {
      FUN_004f57b0(local_1c,pvVar5);
      piVar6 = (int *)FUN_004f5cd0(0x201);
      local_2c = piVar6;
      if (piVar6 != (int *)0x0) {
        piVar6[8] = param_1[1];
        (**(code **)(*piVar6 + 0x24))();
        (**(code **)(*piVar6 + 0x2c))(&local_34);
      }
    }
  }
LAB_004b0f44:
  local_4._0_1_ = 1;
  FUN_004f4380(local_1c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_2c;
}

