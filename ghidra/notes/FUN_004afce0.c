
int * __fastcall FUN_004afce0(int param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *pvVar4;
  int *piVar5;
  undefined1 *this;
  uint *puVar6;
  int iVar7;
  uint local_88;
  uint local_84;
  uint local_80;
  int local_7c;
  undefined4 local_78;
  int *local_74;
  int local_70;
  void *local_6c;
  undefined1 local_68 [4];
  uint local_64;
  undefined1 local_60 [4];
  undefined1 local_5c [4];
  undefined1 *local_58;
  undefined4 local_54 [2];
  undefined4 local_4c;
  undefined4 local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c [4];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638cf8;
  pvStack_c = ExceptionList;
  local_84 = 0;
  local_74 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_7c = param_1;
  FUN_004ece30(&local_88);
  local_4 = 0;
  FUN_004355d0(local_3c);
  local_4._0_1_ = 1;
  local_70 = 0;
  FUN_004ece30(&local_78);
  local_4._0_1_ = 2;
  FUN_004ece30(&local_80);
  local_4._0_1_ = 3;
  FUN_004f4340(local_54);
  local_4._0_1_ = 4;
  pvVar2 = FUN_0041acd0(*(void **)(param_1 + 0xc),*(int *)(*(int *)(param_1 + 0x10) + 0x18),0x40001,
                        0xf0000002,0xc,(void *)0x1);
  pvVar4 = (void *)(param_1 + 0x18);
  local_6c = pvVar4;
  FUN_00435790(pvVar4,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar4,local_68);
  local_4._0_1_ = 5;
  FUN_004f26d0(&local_88,puVar3);
  while( true ) {
    local_4._0_1_ = 4;
    FUN_00619730();
    local_4c = 8;
    local_48 = 0x10;
    if (((local_88 >> 0x18 < 8) || (0xf < local_88 >> 0x18)) || (local_70 != 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) break;
    iVar7 = *(int *)(param_1 + 0xc);
    local_58 = &stack0xffffff64;
    FUN_004f26d0(&stack0xffffff64,&local_88);
    pvVar4 = FUN_00419e40((void *)(iVar7 + 0x78));
    if (pvVar4 != (void *)0x0) {
      iVar7 = *(int *)(param_1 + 0xc);
      local_58 = &stack0xffffff64;
      FUN_004f26d0(&stack0xffffff64,(undefined4 *)((int)pvVar4 + 0x28));
      pvVar2 = FUN_00403d30((void *)(iVar7 + 0x2c));
      param_1 = local_7c;
      if ((pvVar2 != (void *)0x0) && ((*(byte *)((int)pvVar2 + 0x30) & 1) != 0)) {
        FUN_004f26d0(&local_78,(undefined4 *)((int)pvVar4 + 0x28));
        iVar7 = local_7c;
        pvVar4 = FUN_0041ac80(*(void **)(local_7c + 0xc),&local_88,0x4000,0x20800,9,(void *)0x1);
        FUN_00435790(local_3c,(int)pvVar4);
        puVar3 = FUN_004357b0(local_3c,local_68);
        local_4._0_1_ = 6;
        FUN_004f26d0(&local_80,puVar3);
        while( true ) {
          local_4._0_1_ = 4;
          FUN_00619730();
          local_44 = 0x14;
          local_40 = 0x1c;
          if ((local_80 >> 0x18 < 0x14) || (0x1b < local_80 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) break;
          piVar5 = FUN_004f2e20(*(int *)(iVar7 + 4),&local_80);
          if (piVar5 != (int *)0x0) {
            FUN_00504c40(local_2c,piVar5,1);
            local_4 = CONCAT31(local_4._1_3_,7);
            FUN_00513120((int)local_2c);
            if (local_10 != 0) {
              local_70 = 1;
              do {
                this = (undefined1 *)FUN_00618b70(0x20);
                local_4._0_1_ = 8;
                local_58 = this;
                if (this == (undefined1 *)0x0) {
                  pvVar4 = (void *)0x0;
                }
                else {
                  puVar6 = &local_64;
                  pvVar4 = (void *)FUN_0052bed0((int)local_2c);
                  puVar6 = FUN_004025b0(pvVar4,puVar6);
                  local_84 = local_84 | 1;
                  local_4 = CONCAT31(local_4._1_3_,9);
                  pvVar4 = FUN_004f5b10(this,puVar6,0);
                }
                local_4 = 8;
                if ((local_84 & 1) != 0) {
                  local_84 = local_84 & 0xfffffffe;
                  FUN_00619730();
                }
                local_4 = CONCAT31(local_4._1_3_,7);
                if (pvVar4 != (void *)0x0) {
                  FUN_004f57b0(local_54,pvVar4);
                }
                FUN_005130d0((int)local_2c);
              } while (local_10 != 0);
            }
            local_4._0_1_ = 4;
            FUN_00504d40(local_2c);
          }
          puVar3 = FUN_004357b0(local_3c,local_60);
          local_4._0_1_ = 10;
          FUN_004f26d0(&local_80,puVar3);
        }
        FUN_005f58b0(local_3c);
        param_1 = local_7c;
      }
    }
    puVar3 = FUN_004357b0(local_6c,local_5c);
    local_4._0_1_ = 0xb;
    FUN_004f26d0(&local_88,puVar3);
  }
  FUN_005f58b0(local_6c);
  iVar7 = thunk_FUN_005f5060((int)local_54);
  if ((iVar7 != 0) && (piVar5 = (int *)FUN_004f5cd0(0x201), local_74 = piVar5, piVar5 != (int *)0x0)
     ) {
    piVar5[8] = *(int *)(param_1 + 4);
    (**(code **)(*piVar5 + 0x24))();
    (**(code **)(*piVar5 + 0x2c))(&local_7c);
  }
  local_4._0_1_ = 3;
  FUN_004f4380(local_54);
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00435610(local_3c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_74;
}

