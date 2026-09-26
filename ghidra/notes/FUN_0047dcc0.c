
undefined4 __fastcall FUN_0047dcc0(int param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  uint *puVar6;
  int *piVar7;
  uint *puVar8;
  undefined4 *puVar9;
  void *pvVar10;
  uint local_2c;
  uint *local_28;
  undefined4 local_24;
  undefined1 *local_20;
  undefined4 local_1c;
  undefined4 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633ba8;
  local_c = ExceptionList;
  local_24 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_2c);
  local_18 = (undefined4 *)(param_1 + 0x30);
  local_4 = 0;
  FUN_004ece80(local_18);
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffef;
  if ((*(byte *)(param_1 + 0x20) & 1) == 0) {
    local_28 = (uint *)(param_1 + 0x34);
    FUN_004ece80(local_28);
    puVar8 = local_28;
LAB_0047e0cb:
    if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) goto LAB_0047e0e3;
  }
  else {
    puVar8 = (uint *)(param_1 + 0x34);
    uVar4 = *(uint *)(param_1 + 0x34) >> 0x18;
    local_20 = (undefined1 *)0x90;
    local_1c = 0x98;
    if ((uVar4 < 0x90) || (0x97 < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_28 = puVar8;
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x68);
      local_20 = &stack0xffffffc0;
      FUN_004f26d0(&stack0xffffffc0,puVar8);
      pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x2c) & 2) != 0)) {
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
      }
    }
    if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) {
      pvVar5 = (void *)thunk_FUN_005f5060(param_1 + 0x3c);
      if (pvVar5 != (void *)0x0) {
        puVar6 = FUN_00403040(pvVar5,&local_20);
        local_4._0_1_ = 1;
        pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0x68) + 0x8c),puVar6);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        if (pvVar5 != (void *)0x0) {
          FUN_004f26d0(&local_2c,(undefined4 *)((int)pvVar5 + 0x38));
          bVar2 = false;
          while( true ) {
            local_14 = 0x90;
            local_10 = 0x98;
            if (((0x8f < local_2c >> 0x18) && (local_2c >> 0x18 < 0x98)) || (bVar2)) {
              bVar3 = false;
            }
            else {
              bVar3 = true;
            }
            FUN_00619730();
            if (!bVar3) break;
            piVar7 = FUN_004f2d10(*(int *)(param_1 + 0x24),&local_2c);
            if (piVar7 != (int *)0x0) {
              if ((void *)piVar7[7] == (void *)0x0) {
                FUN_004ece80(&local_2c);
                bVar2 = true;
              }
              else {
                puVar8 = FUN_004025b0((void *)piVar7[7],(uint *)&local_20);
                local_4._0_1_ = 2;
                FUN_004f26d0(&local_2c,puVar8);
                local_4 = (uint)local_4._1_3_ << 8;
                FUN_00619730();
              }
            }
          }
          local_14 = 0x90;
          local_10 = 0x98;
          if ((local_2c >> 0x18 < 0x90) || (0x97 < local_2c >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          puVar8 = local_28;
          if (bVar2) {
            iVar1 = *(int *)(param_1 + 0x68);
            local_20 = &stack0xffffffc0;
            FUN_004f26d0(&stack0xffffffc0,&local_2c);
            pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
            puVar8 = local_28;
            if ((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x2c) & 2) != 0)) {
              FUN_004f26d0(local_28,&local_2c);
              *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
              puVar8 = local_28;
            }
          }
        }
      }
      if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) {
        uVar4 = *(uint *)(param_1 + 0x38) >> 0x18;
        local_14 = 0x90;
        local_10 = 0x98;
        if ((uVar4 < 0x90) || (0x97 < uVar4)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          puVar9 = FUN_00419b80(*(void **)(param_1 + 0x68),(undefined4 *)(param_1 + 0x38),0,8,0,0,0,
                                0,2);
          pvVar5 = (void *)(param_1 + 0x54);
          FUN_00435790(pvVar5,(int)puVar9);
          puVar9 = FUN_004357b0(pvVar5,&local_20);
          local_4._0_1_ = 3;
          FUN_004f26d0(puVar8,puVar9);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar5);
          puVar9 = FUN_0041a400(*(void **)(param_1 + 0x68),puVar8,(undefined4 *)(param_1 + 0x38),0,2
                                ,0,0,0,0,(undefined4 *)&DAT_00000002);
          FUN_00435790(pvVar5,(int)puVar9);
          puVar9 = FUN_004357b0(pvVar5,&local_20);
          local_4._0_1_ = 4;
          FUN_004f26d0(puVar8,puVar9);
        }
        else {
          pvVar10 = FUN_00419a00(*(void **)(param_1 + 0x68),0,8,0,0,0,0,0x3c,(void *)0x1);
          pvVar5 = (void *)(param_1 + 0x54);
          FUN_00435790(pvVar5,(int)pvVar10);
          puVar9 = FUN_004357b0(pvVar5,&local_20);
          local_4._0_1_ = 5;
          FUN_004f26d0(puVar8,puVar9);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar5);
          pvVar10 = FUN_0041a340(*(void **)(param_1 + 0x68),puVar8,0,2,0,0,0,0,0,(void *)0x1);
          FUN_00435790(pvVar5,(int)pvVar10);
          puVar9 = FUN_004357b0(pvVar5,&local_20);
          local_4._0_1_ = 6;
          FUN_004f26d0(puVar8,puVar9);
        }
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        FUN_005f58b0((void *)(param_1 + 0x54));
        local_14 = 0x90;
        local_10 = 0x98;
        if ((*puVar8 >> 0x18 < 0x90) || (0x97 < *puVar8 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
        }
        goto LAB_0047e0cb;
      }
    }
  }
  FUN_004f26d0(local_18,puVar8);
  local_24 = 1;
LAB_0047e0e3:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_24;
}

